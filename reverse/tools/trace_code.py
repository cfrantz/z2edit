#!/usr/bin/env python3
"""Find and disassemble code reachable from entry points through .byte data.

    trace_code.py [--dry-run] BANK ENTRY [ENTRY ...]

Follows the flow of control from each entry address (in BANK, or bank 7
for $C000 and up): through branches, jumps and subroutine calls within
the same bank, stopping at rts/rti, indirect jumps, and calls to
JumpTable-style dispatchers (whose inline tables follow the call).  Code
it reaches that the source still has as .byte data is disassembled with
disasm.py, one contiguous run at a time; entries become loc_ labels.
"""
import argparse
import os
import re
import subprocess
import sys

from bootstrap import OPCODES, SIZES

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
ROM = os.path.join(HERE, "../build/zelda2.nes")
DISPATCHERS = {"JumpTable", "JumpTableBySubStep"}


def source_kinds(bank):
    """address -> 'byte' or 'code' for the lines of bank's source."""
    kinds = {}
    for line in open(os.path.join(SRC, f"bank{bank}.s")):
        m = re.search(r"; \$([0-9A-F]{4})", line)
        if not m or line.lstrip().startswith(";"):
            continue
        a = int(m.group(1), 16)
        bm = re.match(r"^\s+\.byte\s+(.*?)\s*;", line)
        if bm:
            for k in range(len([v for v in bm.group(1).split(",") if v.strip()])):
                kinds[a + k] = "byte"
        elif not re.match(r"^\s+\.(word|addr)", line):
            kinds[a] = "code"
    return kinds


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--dry-run", action="store_true")
    p.add_argument("bank", type=int)
    p.add_argument("entries", nargs="+")
    args = p.parse_args()
    prg = open(ROM, "rb").read()[16:]
    import json
    syms = {}
    for s in json.load(open(os.path.join(HERE, "../build/zelda2.sym.json")))["labels"]:
        syms[(s["bank"], s["addr"])] = s["name"]

    results = {}
    for bank in sorted({args.bank, 7}):
        results[bank] = set()
    kinds = {b: source_kinds(b) for b in results}

    def byte(bank, a):
        off = (7 * 0x4000 + a - 0xC000) if a >= 0xC000 else bank * 0x4000 + a - 0x8000
        return prg[off]

    todo = []
    for e in args.entries:
        a = int(e.lstrip("$"), 16)
        todo.append((7 if a >= 0xC000 else args.bank, a))
    entries = set(todo)
    seen = set()
    while todo:
        bank, a = todo.pop()
        while (bank, a) not in seen:
            if kinds[bank].get(a) != "byte":
                break  # already code (or not in the source as data)
            seen.add((bank, a))
            op = byte(bank, a)
            if op not in OPCODES:
                print(f"bank {bank} ${a:04X}: bad opcode ${op:02X}; stopping this path", file=sys.stderr)
                break
            mnem, mode = OPCODES[op]
            size = SIZES[mode]
            operand = byte(bank, a + 1) | (byte(bank, a + 2) << 8 if size == 3 else 0) if size > 1 else None
            for k in range(size):
                results[bank].add(a + k)
            nxt = a + size
            if mode == "rel":
                t = (nxt + (operand - 256 if operand >= 128 else operand)) & 0xFFFF
                todo.append((bank, t))
            elif mnem in ("jmp", "jsr") and mode == "abs":
                tb = 7 if operand >= 0xC000 else bank
                if tb in kinds:
                    todo.append((tb, operand))
                if mnem == "jsr" and syms.get((tb, operand)) in DISPATCHERS:
                    break
                if mnem == "jmp":
                    break
            elif mnem in ("rts", "rti", "brk") or (mnem == "jmp" and mode == "ind"):
                break
            a = nxt

    for bank, addrs in results.items():
        if not addrs:
            continue
        runs, start, prev = [], None, None
        for a in sorted(addrs):
            if start is None:
                start = prev = a
            elif a == prev + 1:
                prev = a
            else:
                runs.append((start, prev + 1))
                start = prev = a
        runs.append((start, prev + 1))
        for s, e in runs:
            ents = [f"{a:04X}" for b, a in entries if b == bank and s < a < e]
            print(f"bank {bank} ${s:04X}-${e - 1:04X}" + (f" entries {' '.join(ents)}" if ents else ""))
            if not args.dry_run:
                cmd = [sys.executable, os.path.join(HERE, "disasm.py"), str(bank), f"{s:04X}", f"{e:04X}"]
                for x in ents:
                    cmd += ["--entry", x]
                subprocess.run(cmd, check=True)
                subprocess.run(["make", "-s", "-C", os.path.join(HERE, ".."), "build/zelda2.sym.json"],
                               check=False, capture_output=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
