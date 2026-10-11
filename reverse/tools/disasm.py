#!/usr/bin/env python3
"""Disassemble a run of .byte data in the source into instructions.

    disasm.py BANK START END [--entry ADDR ...]

Replaces the .byte lines covering BANK:START..END (exclusive) with
instructions, decoded linearly from START.  Labels already in the range
are kept; branch and jump targets inside the range get mechanical loc_
labels; targets outside it get labels through label.py.  RAM operands are
named from ram.inc (arrays as NAME+k); unnamed addresses get mechanical
ram_/sram_ declarations.  --entry adds labels at further addresses (e.g.
entry points known from pointer tables).  Use it for code the movies
never ran, which the bootstrap left as data.  Run `make verify` after.
"""
import argparse
import json
import os
import re
import sys

from bootstrap import HARDWARE, OPCODES, SIZES
from label import ADDR, BYTES, LABEL, ensure_label, mechanical
from rename import realign

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
SYM = os.path.join(HERE, "../build/zelda2.sym.json")
DECL = re.compile(r"^([A-Za-z_]\w*)\s*=\s*\$([0-9A-F]{4})\b(?:\s*;\s*\[(\d+)\])?")
MODES = {"zp": "{}", "abs": "{}", "zpx": "{},x", "absx": "{},x", "zpy": "{},y",
         "absy": "{},y", "indx": "({},x)", "indy": "({}),y", "ind": "({})"}


class RamNames:
    def __init__(self):
        self.path = os.path.join(SRC, "ram.inc")
        self.exact, self.arrays, self.new = {}, [], {}
        for line in open(self.path):
            m = DECL.match(line)
            if m:
                a = int(m.group(2), 16)
                name = m.group(1)
                if a not in self.exact or self.exact[a].startswith(("ram_", "sram_")):
                    self.exact[a] = name
                if m.group(3) and not name.startswith(("ram_", "sram_")):
                    self.arrays.append((a, int(m.group(3)), name))

    def name(self, a):
        if a in self.exact and not self.exact[a].startswith(("ram_", "sram_")):
            return self.exact[a]
        for base, size, n in self.arrays:
            if base < a < base + size:
                return f"{n}+{a - base}"
        if a in self.exact:
            return self.exact[a]
        if a < 0x800 or 0x6000 <= a < 0x8000:
            n = f"{'sram' if a >= 0x6000 else 'ram'}_{a:04X}"
            self.new[a] = n
            self.exact[a] = n
            return n
        return None

    def save(self):
        if not self.new:
            return
        lines = open(self.path).read().split("\n")
        for a, n in sorted(self.new.items()):
            decl = realign(f"{n} = ${a:04X}")
            # Insert before the first declaration with a higher address.
            for i, line in enumerate(lines):
                m = DECL.match(line)
                if m and int(m.group(2), 16) > a:
                    lines.insert(i, decl)
                    break
            else:
                lines.append(decl)
        open(self.path, "w").write("\n".join(lines))


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("bank", type=int)
    p.add_argument("start")
    p.add_argument("end")
    p.add_argument("--entry", action="append", default=[])
    args = p.parse_args()
    bank = args.bank
    start, end = int(args.start.lstrip("$"), 16), int(args.end.lstrip("$"), 16)
    # Make the range's ends line boundaries.
    ensure_label(bank, start)
    try:
        ensure_label(bank, end)
    except ValueError:
        pass
    path = os.path.join(SRC, f"bank{bank}.s")
    lines = open(path).read().split("\n")

    # Collect the bytes and the labels in the range.
    data, labels, first, last = {}, {}, None, None
    pending = []
    for i, line in enumerate(lines):
        m = BYTES.match(line)
        a = int(ADDR.search(line).group(1), 16) if ADDR.search(line) and not line.lstrip().startswith(";") else None
        lm = LABEL.match(line)
        if lm:
            pending.append(lm.group(1))
            continue
        if a is None:
            continue
        if start <= a < end:
            if not m:
                sys.exit(f"not .byte data at ${a:04X}: {line.strip()}")
            vals = [int(v.strip().lstrip("$"), 16) for v in m.group(2).split(",") if v.strip()]
            for k, v in enumerate(vals):
                data[a + k] = v
            for n in pending:
                labels[a] = labels.get(a) or n
            first = i if first is None else first
            last = i
        pending = []
    if sorted(data) != list(range(start, end)):
        sys.exit("the range is not contiguous .byte data")
    # Include label lines right before the first data line.
    while first > 0 and (LABEL.match(lines[first - 1]) or lines[first - 1].strip() == ""
                         or lines[first - 1].lstrip().startswith("; (")):
        first -= 1

    syms = {}
    for s in json.load(open(SYM))["labels"]:
        if not s["name"].startswith("@"):
            syms.setdefault((s["bank"], s["addr"]), s["name"])
    ram = RamNames()

    # Decode.
    insns, a = [], start
    while a < end:
        op = data[a]
        if op not in OPCODES:
            sys.exit(f"unknown opcode ${op:02X} at ${a:04X}")
        mnem, mode = OPCODES[op]
        size = SIZES[mode]
        if a + size > end:
            sys.exit(f"instruction at ${a:04X} crosses the end")
        operand = data[a + 1] if size == 2 else (data[a + 1] | data[a + 2] << 8 if size == 3 else None)
        insns.append((a, mnem, mode, size, operand))
        a += size
    starts = {i[0] for i in insns}
    for e in args.entry:
        labels.setdefault(int(e.lstrip("$"), 16), mechanical(bank, int(e.lstrip("$"), 16), "loc"))

    outside = []  # (bank, addr, prefix) labels to create elsewhere

    def rom_ref(target, prefix):
        if start <= target < end:
            if target not in starts:
                sys.exit(f"reference into the middle of an instruction at ${target:04X}")
            labels.setdefault(target, mechanical(bank, target, prefix))
            return labels[target]
        tb = 7 if target >= 0xC000 else bank
        n = syms.get((tb, target))
        if n:
            return n
        n = mechanical(tb, target, prefix)
        outside.append((tb, target, n))
        syms[(tb, target)] = n
        return n

    # First pass to place labels for targets, then format.
    out_insns = []
    for a, mnem, mode, size, operand in insns:
        if mode in ("imp",):
            text = ""
        elif mode == "acc":
            text = "a"
        elif mode == "imm":
            text = f"#${operand:02X}"
        elif mode == "rel":
            target = (a + 2 + (operand - 256 if operand >= 128 else operand)) & 0xFFFF
            text = rom_ref(target, "loc")
        else:
            if operand in HARDWARE:
                sym = HARDWARE[operand]
            elif operand >= 0x8000:
                if mnem in ("sta", "stx", "sty", "inc", "dec", "asl", "lsr", "rol", "ror"):
                    sym = f"${operand:04X}"
                else:
                    sym = rom_ref(operand, "loc" if mnem in ("jsr", "jmp") else "dat")
            else:
                sym = ram.name(operand) or (f"${operand:04X}" if size == 3 else f"${operand:02X}")
            if size == 3 and operand < 0x100 and mode != "ind":
                sym = "a:" + sym
            text = MODES[mode].format(sym)
        out_insns.append((a, mnem, text))

    out = ["; (disassembled with tools/disasm.py; not executed in the test movies)"]
    sizes = {i[0]: i[3] for i in insns}
    for a, mnem, text in out_insns:
        if a in labels:
            out += ["", f"{labels[a]}:"]
        for k in range(1, sizes[a]):
            if a + k in labels:
                out.append(f"{labels[a + k]} := * + {k}")
        out.append(realign(f"    {mnem:<8}{text}".rstrip() + f" ; ${a:04X}"))
    out.append("; (end of disassembled code)")
    out.append("")
    lines[first:last + 1] = out
    open(path, "w").write("\n".join(lines))
    ram.save()
    for tb, target, n in outside:
        ensure_label(tb, target, n)
    print(f"bank {bank} ${start:04X}-${end - 1:04X}: {len(insns)} instructions, "
          f"{len(outside)} outside labels, {len(ram.new)} new RAM names")
    return 0


if __name__ == "__main__":
    sys.exit(main())
