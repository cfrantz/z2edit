#!/usr/bin/env python3
"""Update the generated section of STATUS.md.

Reports, per PRG bank:
  - CDL coverage: bytes executed as code, read as data, never accessed.
  - Labels: total, still auto-named (sub_/loc_/dat_), and named.
  - Named routines marked "Status: confirmed" / "Status: provisional".
And for RAM (ram.inc): variables defined, still auto-named (ram_/sram_), and named.
"""
import argparse
import json
import os
import re
import sys

BANK_SIZE = 0x4000
OPCODE, OPERAND, DATA, DATA_INDIRECT = 0x01, 0x02, 0x04, 0x08
AUTO_LABEL = re.compile(r"^(sub|loc|dat)_")
AUTO_RAM = re.compile(r"^s?ram_[0-9A-F]{4}$")
BEGIN = "<!-- BEGIN GENERATED: tools/status.py -->"
END = "<!-- END GENERATED -->"


def pct(n, d):
    return f"{100 * n / d:.1f}%" if d else "-"


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--cdl", default="build/zelda2.cdl")
    p.add_argument("--symbols", default="build/zelda2.sym.json")
    p.add_argument("--src", default="src")
    p.add_argument("--status", default="STATUS.md")
    args = p.parse_args()

    with open(args.cdl, "rb") as f:
        cdl = f.read()
    with open(args.symbols) as f:
        syms = json.load(f)
    with open(os.path.join(args.src, "ram.inc")) as f:
        ram = [n for n in re.findall(r"^(\w+)\s*=", f.read(), re.M) if n != "RAM_INC"]

    out = [BEGIN, "", "### Code/data coverage (from CDL)", ""]
    out.append("| Bank | Code | Data | Unaccessed |")
    out.append("|-----:|-----:|-----:|-----------:|")
    for bank in range(len(cdl) // BANK_SIZE):
        b = cdl[bank * BANK_SIZE : (bank + 1) * BANK_SIZE]
        code = sum(1 for x in b if x & (OPCODE | OPERAND))
        data = sum(1 for x in b if x & (DATA | DATA_INDIRECT) and not x & (OPCODE | OPERAND))
        none = sum(1 for x in b if not x & (OPCODE | OPERAND | DATA | DATA_INDIRECT))
        out.append(f"| {bank} | {code} | {data} | {none} ({pct(none, BANK_SIZE)}) |")

    out += ["", "### Labels", ""]
    out.append("| Bank | Labels | Auto-named | Named | Confirmed | Provisional |")
    out.append("|-----:|-------:|-----------:|------:|----------:|------------:|")
    totals = [0, 0, 0, 0, 0]
    for bank in range(len(cdl) // BANK_SIZE):
        labels = [s["name"] for s in syms["labels"] if s["bank"] == bank]
        auto = sum(1 for n in labels if AUTO_LABEL.match(n))
        src = os.path.join(args.src, f"bank{bank}.s")
        text = open(src).read() if os.path.exists(src) else ""
        confirmed = len(re.findall(r"^;\s*Status:\s*confirmed", text, re.M))
        provisional = len(re.findall(r"^;\s*Status:\s*provisional", text, re.M))
        row = [len(labels), auto, len(labels) - auto, confirmed, provisional]
        totals = [a + b for a, b in zip(totals, row)]
        out.append(f"| {bank} | " + " | ".join(map(str, row)) + " |")
    out.append("| **all** | " + " | ".join(f"**{t}**" for t in totals) + " |")

    auto = sum(1 for n in ram if AUTO_RAM.match(n))
    out += ["", "### RAM variables", ""]
    out.append(f"{len(ram)} defined, {auto} auto-named, {len(ram) - auto} named ({pct(len(ram) - auto, len(ram))}).")
    out += ["", END]

    text = open(args.status).read() if os.path.exists(args.status) else f"# Status\n\n{BEGIN}\n{END}\n"
    if BEGIN not in text or END not in text:
        text += f"\n{BEGIN}\n{END}\n"
    head, _, rest = text.partition(BEGIN)
    _, _, tail = rest.partition(END)
    with open(args.status, "w") as f:
        f.write(head + "\n".join(out) + tail)
    return 0


if __name__ == "__main__":
    sys.exit(main())
