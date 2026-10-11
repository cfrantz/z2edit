#!/usr/bin/env python3
"""Rename symbols throughout reverse/src.

    rename.py OLD NEW [OLD NEW ...]
    rename.py --file renames.txt      (one "OLD NEW" pair per line, # comments)

Replaces whole-word occurrences in src/*.s and src/*.inc.  Refuses to
rename to a name that already exists, or to rename a symbol that doesn't.
Run `make verify` afterwards.
"""
import argparse
import glob
import os
import re
import sys

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../src")
# An instruction or data line with a trailing address comment.
ADDR_LINE = re.compile(r"^(\s+\S.*?)\s+(; \$[0-9A-F]{4}\b.*)$")
COMMENT_COLUMN = 36


def realign(line):
    """Re-pad the trailing address comment of a line to its column."""
    m = ADDR_LINE.match(line)
    if not m:
        return line
    code, comment = m.groups()
    return f"{code:<{COMMENT_COLUMN - 1}} {comment}"


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--file", help="File of OLD NEW pairs")
    p.add_argument("pairs", nargs="*")
    args = p.parse_args()

    words = list(args.pairs)
    if args.file:
        with open(args.file) as f:
            for line in f:
                words.extend(line.split("#", 1)[0].split())
    if len(words) % 2:
        print("expected OLD NEW pairs", file=sys.stderr)
        return 1
    pairs = list(zip(words[0::2], words[1::2]))

    files = sorted(glob.glob(os.path.join(SRC, "*.s")) + glob.glob(os.path.join(SRC, "*.inc")))
    text = {f: open(f).read() for f in files}
    everything = "\n".join(text.values())
    errors = []
    olds = {old for old, _ in pairs}
    for old, new in pairs:
        if not re.search(rf"\b{re.escape(old)}\b", everything):
            errors.append(f"{old}: not found")
        if new not in olds and re.search(rf"\b{re.escape(new)}\b", everything):
            errors.append(f"{new}: already exists")
    if len({new for _, new in pairs}) != len(pairs):
        errors.append("duplicate new names")
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1

    mapping = dict(pairs)
    pattern = re.compile(r"\b(" + "|".join(re.escape(o) for o in mapping) + r")\b")
    for f, t in text.items():
        lines = []
        for line in t.split("\n"):
            new_line = pattern.sub(lambda m: mapping[m.group(1)], line)
            lines.append(realign(new_line) if new_line != line else line)
        new = "\n".join(lines)
        if new != t:
            with open(f, "w") as out:
                out.write(new)
    print(f"renamed {len(pairs)} symbols")
    return 0


if __name__ == "__main__":
    sys.exit(main())
