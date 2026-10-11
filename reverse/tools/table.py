#!/usr/bin/env python3
"""Turn a run of .byte data into a named table.

    table.py BANK ADDR SIZE NAME [--word] [--per-line N]

Gathers the SIZE bytes at BANK:ADDR (which may span several .byte lines
and labels), names the table NAME, and rewrites it as .byte lines, or with
--word as .word lines.  Labels inside the table become NAME+k in every
reference; an existing label at ADDR is renamed to NAME.  With --word,
values that are the address of a label in the same bank (or of a RAM/SRAM
variable from ram.inc, within its declared size) are written by name.
Run `make verify` afterwards.
"""
import argparse
import json
import os
import re
import subprocess
import sys

from label import ADDR, BYTES, LABEL, ensure_label
from rename import realign

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
RAM_DECL = re.compile(r"^([A-Za-z_]\w*)\s*=\s*\$([0-9A-F]{4})\s*(?:;\s*\[(\d+)\])?")


def rename(pairs):
    if pairs:
        args = [x for p in pairs for x in p]
        subprocess.run([sys.executable, os.path.join(HERE, "rename.py"), *args], check=True)


def symbolizer(bank):
    """Return a function giving a symbolic name for a word value."""
    exact = {}
    try:
        with open(os.path.join(HERE, "../build/zelda2.sym.json")) as f:
            for lab in json.load(f)["labels"]:
                if lab["bank"] in (bank, 7) and not lab["name"].startswith("@"):
                    if (lab["bank"] == 7) == (lab["addr"] >= 0xC000):
                        exact.setdefault(lab["addr"], lab["name"])
    except FileNotFoundError:
        pass
    ram = []
    with open(os.path.join(SRC, "ram.inc")) as f:
        for line in f:
            m = RAM_DECL.match(line)
            if m:
                ram.append((int(m.group(2), 16), int(m.group(3) or 1), m.group(1)))
    ram.sort(reverse=True)

    def name(v):
        if 0x8000 <= v and v in exact:
            return exact[v]
        for base, size, n in ram:
            if v < 0x6000:
                break
            if base <= v < base + size and not n.startswith(("ram_", "sram_")):
                return n if v == base else f"{n}+{v - base}"
        return f"${v:04X}"

    return name


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("bank", type=int)
    p.add_argument("addr")
    p.add_argument("size", type=int)
    p.add_argument("name")
    p.add_argument("--word", action="store_true")
    p.add_argument("--per-line", type=int, default=0)
    args = p.parse_args()
    bank, start = args.bank, int(args.addr.lstrip("$"), 16)
    end = start + args.size
    path = os.path.join(SRC, f"bank{bank}.s")

    # Make the table's ends line boundaries, then name it.
    first = ensure_label(bank, start)
    try:
        ensure_label(bank, end)
    except ValueError:
        pass
    renames = [] if first == args.name else [(first, args.name)]

    # Find the labels inside the table.
    lines = open(path).read().split("\n")
    addr = None
    i0 = next(i for i, l in enumerate(lines) if LABEL.match(l) and LABEL.match(l).group(1) == first)
    pending = []
    for line in lines[i0 + 1:]:
        lm = LABEL.match(line)
        if lm:
            pending.append(lm.group(1))
            continue
        m = BYTES.match(line)
        if m:
            addr = int(m.group(4), 16)
            if addr >= end:
                break
            renames += [(n, f"{args.name}+{addr - start}") for n in pending]
            pending = []
        elif ADDR.search(line) and not line.lstrip().startswith(";"):
            if int(ADDR.search(line).group(1), 16) >= end:
                break
            sys.exit(f"non-.byte line inside the table: {line.strip()}")
    rename(renames)

    # Gather the bytes and rewrite the lines.
    lines = open(path).read().split("\n")
    i0 = next(i for i, l in enumerate(lines) if l == f"{args.name}:")
    values, i = [], i0 + 1
    while i < len(lines):
        m = BYTES.match(lines[i])
        if m and int(m.group(4), 16) >= end:
            break
        if LABEL.match(lines[i]):
            break
        if m:
            values += [int(v.strip().lstrip("$"), 16) for v in m.group(2).split(",") if v.strip()]
        elif lines[i].strip() and not lines[i].lstrip().startswith(";"):
            sys.exit(f"unexpected line in table: {lines[i]}")
        i += 1
    while i > i0 + 1 and lines[i - 1] == "":
        i -= 1
    if len(values) != args.size:
        sys.exit(f"found {len(values)} bytes, expected {args.size}")

    out = []
    if args.word:
        sym = symbolizer(bank)
        words = [sym(values[k] | values[k + 1] << 8) for k in range(0, len(values) - 1, 2)]
        per = args.per_line or 4
        for k in range(0, len(words), per):
            out.append(realign(f"    .word   {', '.join(words[k:k + per])} ; ${start + 2 * k:04X}"))
    else:
        per = args.per_line or 16
        for k in range(0, len(values), per):
            vals = ", ".join(f"${v:02X}" for v in values[k:k + per])
            out.append(realign(f"    .byte   {vals} ; ${start + k:04X}"))
    lines[i0 + 1:i] = out
    with open(path, "w") as f:
        f.write("\n".join(lines))
    print(f"{args.name}: {args.size} bytes at ${start:04X}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
