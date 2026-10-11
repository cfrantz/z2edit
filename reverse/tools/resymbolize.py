#!/usr/bin/env python3
"""Rewrite the entries of .word tables with current symbol names.

    resymbolize.py BANK TABLE [TABLE ...]

Re-evaluates each entry of the named .word tables in src/bankBANK.s from
the ROM and writes it with table.py's symbolizer (labels in the bank or
bank 7, SRAM variables), so that pointers named after the table was
built show their names.  Run `make` first so build/zelda2.sym.json is
current, and `make verify` afterwards.
"""
import os
import re
import sys

from label import ADDR
from rename import realign
from table import symbolizer

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
ROM = os.path.join(HERE, "../build/zelda2.nes")


def main():
    bank, tables = int(sys.argv[1]), set(sys.argv[2:])
    prg = open(ROM, "rb").read()[16:]
    sym = symbolizer(bank)
    path = os.path.join(SRC, f"bank{bank}.s")
    lines = open(path).read().split("\n")
    current = None
    for i, line in enumerate(lines):
        m = re.match(r"^([A-Za-z_]\w*):", line)
        if m:
            current = m.group(1)
            continue
        if current in tables and re.match(r"^\s+\.word\b", line):
            a = int(ADDR.search(line).group(1), 16)
            n = len(line.split(";")[0].split(None, 1)[1].split(","))
            off = (7 * 0x4000 + a - 0xC000) if a >= 0xC000 else bank * 0x4000 + a - 0x8000
            words = [sym(prg[off + 2 * k] | prg[off + 2 * k + 1] << 8) for k in range(n)]
            lines[i] = realign(f"    .word   {', '.join(words)} ; ${a:04X}")
    open(path, "w").write("\n".join(lines))
    return 0


if __name__ == "__main__":
    sys.exit(main())
