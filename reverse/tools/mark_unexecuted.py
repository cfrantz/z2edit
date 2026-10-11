#!/usr/bin/env python3
"""Refresh the "not executed in the test movies" markers of bank files.

    mark_unexecuted.py [--cdl build/zelda2.cdl] BANK [BANK ...]

Removes the "; (code below not executed in the test movies)" and
"; (end of code not executed in the test movies)" lines of src/bankN.s
and writes them again around each run of instructions the CDL says never
ran, as tools/bootstrap.py did.  Run it after merging new runs into the
CDL (tools/cdl_movie.py --merge).
"""
import argparse
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
OPCODE = 0x01

BEGIN = "; (code below not executed in the test movies)"
END = "; (end of code not executed in the test movies)"
INSN = re.compile(r"^    (?:[a-z]{3}\b|\.byte .*\(overlapping instruction\)).*; \$([0-9A-F]{4})")
DATA = re.compile(r"^    \.(?:byte|word|res)\b")
LABEL = re.compile(r"^(?:[A-Za-z_@][A-Za-z0-9_]*:|[A-Za-z_][A-Za-z0-9_]* := \* \+ \d+)$")


def prg_offset(bank, cpu):
    return bank * 0x4000 + (cpu - (0xC000 if bank == 7 else 0x8000))


def insert_at(out):
    """Where the "code below" marker goes before the instruction about to
    be appended to `out`:
    above the labels (and the blank line and "unreferenced" note) that
    lead into it, but below a routine header."""
    i = len(out)
    while i > 0 and (LABEL.match(out[i - 1]) or out[i - 1].startswith("; (unreferenced")):
        i -= 1
    if i > 0 and out[i - 1] == "":
        i -= 1
    if i > 0 and out[i - 1].startswith(";") and i < len(out):
        return len(out)
    return i


def refresh(path, bank, cdl):
    with open(path) as f:
        lines = [l.rstrip("\n") for l in f]
    out = []
    unexecuted = False
    last = 0  # where the line after the last instruction goes
    disassembled = False
    for line in lines:
        if line in (BEGIN, END):
            continue
        # tools/disasm.py marks its own output as never run.
        if line.startswith("; (disassembled with tools/disasm.py"):
            disassembled = True
        elif line == "; (end of disassembled code)":
            disassembled = False
        m = None if disassembled else INSN.match(line)
        if m:
            ran = cdl[prg_offset(bank, int(m.group(1), 16))] & OPCODE
            if ran == unexecuted:
                if unexecuted:
                    out.insert(last, END)
                else:
                    out.insert(insert_at(out), BEGIN)
                unexecuted = not ran
            out.append(line)
            last = len(out)
            continue
        if DATA.match(line) and unexecuted:
            out.insert(last, END)
            unexecuted = False
        out.append(line)
    if unexecuted:
        out.insert(last, END)
    with open(path, "w") as f:
        f.write("\n".join(out) + "\n")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--cdl", default=os.path.join(HERE, "../build/zelda2.cdl"))
    p.add_argument("banks", nargs="+", type=int)
    args = p.parse_args()
    with open(args.cdl, "rb") as f:
        cdl = f.read()
    for bank in args.banks:
        refresh(os.path.join(HERE, f"../src/bank{bank}.s"), bank, cdl)


if __name__ == "__main__":
    main()
