#!/usr/bin/env python3
"""Rename symbols throughout reverse/src.

    rename.py OLD NEW [OLD NEW ...]
    rename.py --file renames.txt      (one "OLD NEW" pair per line, # comments)
    rename.py --array NAME BASE COUNT [--size N]

Replaces whole-word occurrences in src/*.s, src/*.inc and docs/**/*.md.  Refuses to
rename to a name that already exists, or to rename a symbol that doesn't.
Run `make verify` afterwards.

NEW may be an expression NAME+k, for a symbol that is an element of an
array: references become NAME+k (folding OLD+n into NAME+(k+n)), and the
declaration (or label) of OLD is deleted.  NAME must already exist or be the NEW of
another pair.

--array NAME BASE COUNT renames ram_BASE (or sram_BASE) to NAME and each
existing mechanical name within BASE+1..BASE+COUNT*SIZE-1 to NAME+k (SIZE defaults to 1).
"""
import argparse
import glob
import os
import re
import sys

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../src")
DOCS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../docs")
# An instruction or data line with a trailing address comment.
ADDR_LINE = re.compile(r"^(\s+\S.*?)\s+(; \$[0-9A-F]{4}\b.*)$")
COMMENT_COLUMN = 36
BYTE_COMMENT_COLUMN = 77
# A symbol declaration: name = value [; comment]
DECLARATION = re.compile(r"^([A-Za-z_]\w*)\s*=\s*(\S+)(?:\s*(;.*))?$")
VALUE_COLUMN = 24
DECL_COMMENT_COLUMN = 36


def realign(line):
    """Re-pad the trailing address comment of a line, or the value of a
    declaration, to its column."""
    d = DECLARATION.match(line)
    if d:
        name, value, comment = d.groups()
        line = f"{name:<{VALUE_COLUMN}}= {value}"
        return f"{line:<{DECL_COMMENT_COLUMN}}{comment}" if comment else line
    m = ADDR_LINE.match(line)
    if not m:
        return line
    code, comment = m.groups()
    if code.lstrip().startswith(".byte"):
        # Data lines put the comment at a wider column, with no separating
        # space once the line is full.
        return f"{code:<{BYTE_COMMENT_COLUMN - 1}}{comment}"
    return f"{code:<{COMMENT_COLUMN - 1}} {comment}"


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--file", help="File of OLD NEW pairs")
    p.add_argument("--array", nargs=3, metavar=("NAME", "BASE", "COUNT"), help="Name an array")
    p.add_argument("--size", type=int, default=1, help="Element size for --array")
    p.add_argument("pairs", nargs="*")
    args = p.parse_args()

    words = list(args.pairs)
    if args.file:
        with open(args.file) as f:
            for line in f:
                words.extend(line.split("#", 1)[0].split())
    if args.array:
        name, base, count = args.array
        base = int(base.lstrip("$"), 16)
        prefix = "sram" if base >= 0x6000 else "ram"
        words.extend([f"{prefix}_{base:04X}", name])
        for k in range(1, int(count) * args.size):
            words.extend([f"{prefix}_{base + k:04X}", f"{name}+{k}"])
    if len(words) % 2:
        print("expected OLD NEW pairs", file=sys.stderr)
        return 1
    pairs = list(zip(words[0::2], words[1::2]))

    files = sorted(glob.glob(os.path.join(SRC, "*.s")) + glob.glob(os.path.join(SRC, "*.inc")))
    text = {f: open(f).read() for f in files}
    everything = "\n".join(text.values())
    # Documentation follows renames too (but is not searched for collisions).
    docs = sorted(glob.glob(os.path.join(DOCS, "**", "*.md"), recursive=True))
    doc_text = {f: open(f).read() for f in docs}
    # Code only, for collision checks: a word in a comment is not a symbol.
    code = "\n".join(line.split(";", 1)[0] for t in text.values() for line in t.split("\n"))
    errors = []
    if args.array:
        # Elements of the array that have no mechanical name are fine.
        pairs = [(old, new) for old, new in pairs if "+" not in new or re.search(rf"\b{old}\b", everything)]
    olds = {old for old, _ in pairs}
    plain = {new for _, new in pairs if "+" not in new}
    for old, new in pairs:
        if not re.search(rf"\b{re.escape(old)}\b", everything):
            errors.append(f"{old}: not found")
        if "+" in new:
            name = new.split("+")[0]
            if name not in plain and not re.search(rf"\b{re.escape(name)}\b", code):
                errors.append(f"{new}: {name} does not exist")
        elif new not in olds and re.search(rf"\b{re.escape(new)}\b", code):
            errors.append(f"{new}: already exists")
    if len({new for _, new in pairs}) != len(pairs):
        errors.append("duplicate new names")
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1

    mapping = dict(pairs)
    exprs = {old for old, new in pairs if "+" in new}
    pattern = re.compile(r"\b(" + "|".join(re.escape(o) for o in mapping) + r")\b(\+(\d+)\b)?")
    declaration = re.compile(r"^\s*([A-Za-z_]\w*)\s*=")

    def replace(m):
        new, n = mapping[m.group(1)], m.group(3)
        if n is None:
            return new
        name, _, k = new.partition("+")
        return f"{name}+{int(k or 0) + int(n)}"

    for f, t in text.items():
        lines = []
        for line in t.split("\n"):
            d = declaration.match(line)
            if d and d.group(1) in exprs:
                continue  # array element: declared by the array's base
            lbl = re.match(r"^([A-Za-z_]\w*):\s*$", line)
            if lbl and lbl.group(1) in exprs:
                # A label inside a table: refer to it relative to the table.
                if lines and lines[-1] == "":
                    lines.pop()
                continue
            new_line = pattern.sub(replace, line)
            lines.append(realign(new_line) if new_line != line else line)
        new = "\n".join(lines)
        if new != t:
            with open(f, "w") as out:
                out.write(new)
    for f, t in doc_text.items():
        new = pattern.sub(replace, t)
        if new != t:
            with open(f, "w") as out:
                out.write(new)
    print(f"renamed {len(pairs)} symbols")
    return 0


if __name__ == "__main__":
    sys.exit(main())
