#!/usr/bin/env python3
"""Extract a symbol table from the ld65 debug file.

Writes JSON:
    {
      "labels":  [{"name": ..., "bank": N, "addr": cpu_address}, ...],
      "equates": [{"name": ..., "value": V}, ...]
    }

Labels are code/data locations in PRG ROM; `bank` is the PRG bank number
(0-7).  Label names must be unique across all banks; duplicates are an
error.  Equates are RAM variables, hardware registers and constants.
"""
import argparse
import json
import re
import sys

FIELD = re.compile(r'(\w+)=("[^"]*"|[^,]*)')


def parse(line):
    kind, _, rest = line.rstrip("\n").partition("\t")
    fields = {k: v.strip('"') for k, v in FIELD.findall(rest)}
    return kind, fields


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("dbg", help="ld65 debug file")
    p.add_argument("out", help="output JSON file")
    args = p.parse_args()

    banks = {}
    labels = {}
    equates = {}
    duplicates = set()
    with open(args.dbg) as f:
        for line in f:
            kind, fields = parse(line)
            if kind == "seg" and fields["name"].startswith("BANK"):
                banks[fields["id"]] = int(fields["name"][4:])
            elif kind == "sym":
                value = int(fields["val"], 16) if "val" in fields else None
                if fields["type"] == "lab" and fields.get("seg") in banks:
                    sym = {"name": fields["name"], "bank": banks[fields["seg"]], "addr": value}
                    if labels.get(sym["name"], sym) != sym:
                        duplicates.add(sym["name"])
                    labels[sym["name"]] = sym
                elif fields["type"] == "equ" and value is not None:
                    equates[fields["name"]] = {"name": fields["name"], "value": value}

    if duplicates:
        print(f"error: labels defined more than once (see CONVENTIONS.md): {sorted(duplicates)}", file=sys.stderr)
        return 1

    out = {
        "labels": sorted(labels.values(), key=lambda s: (s["bank"], s["addr"], s["name"])),
        "equates": sorted(equates.values(), key=lambda s: (s["value"], s["name"])),
    }
    with open(args.out, "w") as f:
        json.dump(out, f, indent=1)
    return 0


if __name__ == "__main__":
    sys.exit(main())
