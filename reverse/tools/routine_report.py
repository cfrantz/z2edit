#!/usr/bin/env python3
"""Summarize the runtime evidence for each documented routine.

    routine_report.py [--provisional] [--bank N] [NAME ...]

For each routine (a label with a header comment in src/bankN.s), prints
its status, the first line of its description, whether the test movies
executed it (from the merged CDL), and the RAM/SRAM variables its
instructions read (R) and wrote (W), from the RAM access log written by
cdl_movie.py.  Indexed accesses are reported by their base variable.
Use it to check a header against what the code was seen to do.
"""
import argparse
import bisect
import collections
import json
import os
import re
import sys

from ram_usage import RAM_READ, RAM_WRITE, Instruction, Symbols, cpu_addr
from bootstrap import BANK_SIZE, OPCODE

HERE = os.path.dirname(os.path.abspath(__file__))
BUILD = os.path.join(HERE, "../build")
SRC = os.path.join(HERE, "../src")
RULE = "; " + "-" * 75


def headers(bank):
    """{label: (status, first line)} for the routines with headers."""
    out = {}
    lines = open(os.path.join(SRC, f"bank{bank}.s")).read().split("\n")
    for i, line in enumerate(lines):
        if line == RULE and i + 1 < len(lines) and lines[i + 1].startswith("; ") and lines[i + 1] != RULE:
            name = lines[i + 1][2:].strip()
            j = i + 2
            text, status = [], "?"
            while j < len(lines) and lines[j] != RULE:
                t = lines[j][1:].strip()
                if t.startswith("Status:"):
                    status = t.split(":", 1)[1].strip()
                elif t:
                    text.append(t)
                j += 1
            if re.match(r"^[A-Za-z_]\w*$", name):
                out[name] = (status, " ".join(text))
    return out


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--log", default=os.path.join(BUILD, "zelda2"))
    p.add_argument("--rom", default=os.path.join(BUILD, "zelda2.nes"))
    p.add_argument("--sym", default=os.path.join(BUILD, "zelda2.sym.json"))
    p.add_argument("--bank", type=int, action="append")
    p.add_argument("--provisional", action="store_true", help="Only provisional routines")
    p.add_argument("--width", type=int, default=0, help="Truncate descriptions (0 = full)")
    p.add_argument("names", nargs="*")
    args = p.parse_args()
    banks = args.bank or [0, 7]

    syms = Symbols(args.sym, os.path.join(SRC, "ram.inc"))
    # Name addresses inside declared arrays (NAME = $XXXX ; [n]) as NAME+k.
    arrays = []
    with open(os.path.join(SRC, "ram.inc")) as f:
        for line in f:
            m = re.match(r"^([A-Za-z_]\w*)\s*=\s*\$([0-9A-F]{4})\s*;\s*\[(\d+)\]", line)
            if m and not m.group(1).startswith(("ram_", "sram_")):
                arrays.append((int(m.group(2), 16), int(m.group(3)), m.group(1)))
    exact = syms.ram_name

    def ram_name(a):
        n = exact(a)
        if not n.startswith("$"):
            return n
        for base, size, name in arrays:
            if base < a < base + size:
                return f"{name}+{a - base}"
        return n

    syms.ram_name = ram_name
    with open(args.sym) as f:
        labels = [s for s in json.load(f)["labels"] if not s["name"].startswith(("@", "dat_"))]
    with open(f"{args.log}.cdl", "rb") as f:
        cdl = f.read()
    with open(args.rom, "rb") as f:
        rom = f.read()[16:]
    with open(f"{args.log}.ram.json") as f:
        rows = json.load(f)

    # Routine spans per bank, as PRG offsets.
    starts = collections.defaultdict(list)
    for s in labels:
        base = 0xC000 if s["bank"] == 7 else 0x8000
        starts[s["bank"]].append((s["bank"] * BANK_SIZE + s["addr"] - base, s["name"]))
    for v in starts.values():
        v.sort()

    def routine_of(offset):
        bank = offset // BANK_SIZE
        v = starts[bank]
        i = bisect.bisect_right(v, (offset, "￿")) - 1
        return v[i][1] if i >= 0 else None

    access = collections.defaultdict(lambda: collections.defaultdict(int))
    insns = {}
    for src, addr, flags in rows:
        if src not in insns:
            insns[src] = Instruction(rom, src)
        ins = insns[src]
        if ins.mode in ("zpx", "zpy", "absx", "absy", "indx", "indy") and ins.operand is not None:
            name = syms.ram_name(ins.operand)
            if ins.mode in ("indx", "indy"):
                name = f"({name})"
        else:
            name = syms.ram_name(addr)
        access[routine_of(src)][name] |= flags

    for bank in banks:
        hdr = headers(bank)
        v = starts[bank]
        for k, (start, name) in enumerate(v):
            if name not in hdr or (args.names and name not in args.names):
                continue
            status, text = hdr[name]
            if args.provisional and not status.startswith("provisional"):
                continue
            end = v[k + 1][0] if k + 1 < len(v) else (bank + 1) * BANK_SIZE
            ran = any(cdl[o] & OPCODE for o in range(start, end))
            acc = access.get(name, {})
            w = sorted(n for n, f in acc.items() if f & RAM_WRITE)
            r = sorted(n for n, f in acc.items() if f & RAM_READ and not f & RAM_WRITE)
            if args.width and len(text) > args.width:
                text = text[: args.width - 3] + "..."
            _, addr = cpu_addr(start)
            print(f"{name}  ({bank}:${addr:04X}, {status}{'' if ran else ', NOT RUN'})")
            print(f"  {text}")
            if w:
                print(f"  W: {' '.join(w)}")
            if r:
                print(f"  R: {' '.join(r)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
