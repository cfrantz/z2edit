#!/usr/bin/env python3
"""Find or create a label at an address in a bank.

    label.py BANK ADDR [NAME]

Prints the label at BANK:ADDR.  If there is none, creates one (named NAME,
or a mechanical dat_/loc_ name) by inserting a label line, splitting a
.byte line if the address falls inside it, or defining `NAME := * + k` if
the address falls inside an instruction or .word.

Also usable as a module: ensure_label(bank, addr, name=None) -> name.
"""
import os
import re
import sys

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../src")
ADDR = re.compile(r"; \$([0-9A-F]{4})\b")
LABEL = re.compile(r"^([A-Za-z_]\w*):\s*$")
INNER = re.compile(r"^([A-Za-z_]\w*) := \* \+ (\d+)\s*$")
BYTES = re.compile(r"^(\s+\.byte\s+)(.*?)\s*(; \$([0-9A-F]{4})\b.*)$")
COMMENT_COLUMN = 36


def mechanical(bank, addr, prefix="dat"):
    return f"{prefix}_{addr:04X}" if bank == 7 else f"{prefix}_{bank}_{addr:04X}"


def item_size(line):
    """Bytes emitted by an addressed source line, if known."""
    m = BYTES.match(line)
    if m:
        return len([v for v in m.group(2).split(",") if v.strip()])
    if re.match(r"^\s+\.word\b", line):
        return 2
    return None


def ensure_label(bank, addr, name=None):
    path = os.path.join(SRC, f"bank{bank}.s")
    lines = open(path).read().split("\n")
    for i, line in enumerate(lines):
        m = ADDR.search(line)
        if not m or line.lstrip().startswith(";"):
            continue
        start = int(m.group(1), 16)
        if start == addr:
            # Existing label on the preceding lines?
            j = i - 1
            while j >= 0 and (LABEL.match(lines[j]) or INNER.match(lines[j]) or lines[j] == ""):
                lm = LABEL.match(lines[j])
                if lm:
                    return lm.group(1)
                j -= 1
            name = name or mechanical(bank, addr)
            lines[i:i] = ["", f"{name}:"]
            break
        if start < addr:
            # Is addr inside this line?  Find where the next addressed line starts.
            nxt = None
            for k in range(i + 1, len(lines)):
                n = ADDR.search(lines[k])
                if n and not lines[k].lstrip().startswith(";"):
                    nxt = int(n.group(1), 16)
                    break
            end = nxt if nxt is not None and nxt > start else start + (item_size(line) or 1)
            if not start < addr < end:
                continue
            for k in range(i - 1, -1, -1):
                im = INNER.match(lines[k])
                if im and start + int(im.group(2)) == addr:
                    return im.group(1)
                if not INNER.match(lines[k]):
                    break
            name = name or mechanical(bank, addr)
            bm = BYTES.match(line)
            if bm:
                vals = [v.strip() for v in bm.group(2).split(",")]
                k = addr - start
                first = f"{bm.group(1)}{', '.join(vals[:k])}"
                second = f"{bm.group(1)}{', '.join(vals[k:])}"
                lines[i : i + 1] = [
                    f"{first:<{COMMENT_COLUMN - 1}} ; ${start:04X}",
                    "",
                    f"{name}:",
                    f"{second:<{COMMENT_COLUMN - 1}} ; ${addr:04X}",
                ]
            else:
                lines[i:i] = [f"{name} := * + {addr - start}"]
            break
    else:
        raise ValueError(f"bank {bank} address ${addr:04X} not found")
    with open(path, "w") as f:
        f.write("\n".join(lines))
    return name


def main():
    if len(sys.argv) not in (3, 4):
        print(__doc__, file=sys.stderr)
        return 1
    bank = int(sys.argv[1])
    addr = int(sys.argv[2].lstrip("$"), 16)
    print(ensure_label(bank, addr, sys.argv[3] if len(sys.argv) == 4 else None))
    return 0


if __name__ == "__main__":
    sys.exit(main())
