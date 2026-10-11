#!/usr/bin/env python3
"""Name the code or data at addresses.

    name_at.py BANK ADDR NAME [BANK ADDR NAME ...]
    name_at.py --file FILE        (lines of "BANK ADDR NAME", # comments)

For each address, the label there becomes NAME: an existing normal label
is renamed everywhere (rename.py); a cheap local label (@loc_XXXX) is
replaced, with its references in the bank, by NAME; with no label, one is
created (label.py).  Run `make verify` afterwards.
"""
import os
import re
import subprocess
import sys

from label import ADDR, ensure_label
from rename import realign

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
ANY_LABEL = re.compile(r"^(@?[A-Za-z_]\w*):\s*$")


def labels_at(lines, addr):
    """(index, name) of the label lines directly before the line at addr."""
    for i, line in enumerate(lines):
        m = ADDR.search(line)
        if m and not line.lstrip().startswith(";") and int(m.group(1), 16) == addr:
            found = []
            j = i - 1
            while j >= 0 and (ANY_LABEL.match(lines[j]) or lines[j].strip() == "" or lines[j].startswith(";")):
                lm = ANY_LABEL.match(lines[j])
                if lm:
                    found.append((j, lm.group(1)))
                j -= 1
            return i, found
    raise SystemExit(f"address ${addr:04X} not found")


def main():
    args = sys.argv[1:]
    triples = []
    if args[:1] == ["--file"]:
        for line in open(args[1]):
            f = line.split("#", 1)[0].split()
            if f:
                triples.append(f)
    else:
        triples = [args[k:k + 3] for k in range(0, len(args), 3)]
    renames = []
    for bank, addr, name in triples:
        bank, addr = int(bank), int(addr.lstrip("$"), 16)
        path = os.path.join(SRC, f"bank{bank}.s")
        lines = open(path).read().split("\n")
        _, found = labels_at(lines, addr)
        normal = [n for _, n in found if not n.startswith("@")]
        local = [(j, n) for j, n in found if n.startswith("@")]
        if local:
            # Drop the cheap local labels, pointing their references at NAME.
            for j, n in sorted(local, reverse=True):
                del lines[j]
                if j < len(lines) and lines[j - 1].strip() == "" and lines[j].strip() == "":
                    del lines[j]
            text = "\n".join(lines)
            for _, n in local:
                text = re.sub(rf"(?<![\w@]){re.escape(n)}\b", normal[0] if normal else name, text)
            lines = text.split("\n")
            tgt = normal[0] if normal else name
            lines = [realign(l) if tgt in l and l.startswith(" ") else l for l in lines]
            if not normal:
                i, _ = labels_at(lines, addr)
                lines[i:i] = ["", f"{name}:"]
            open(path, "w").write("\n".join(lines))
        elif not normal:
            ensure_label(bank, addr, name)
        if normal and normal[0] != name:
            renames += [normal[0], name]
    if renames:
        subprocess.run([sys.executable, os.path.join(HERE, "rename.py"), *renames], check=True)
    print(f"named {len(triples)} addresses")
    return 0


if __name__ == "__main__":
    sys.exit(main())
