#!/usr/bin/env python3
"""Add or replace routine header comments.

    header.py FILE [FILE ...]
    header.py --status STATUS NAME [NAME ...]

Each FILE holds blocks like:

    === Routine_Name
    One or more lines of description, wrapped as written.

    In:      a = ...
    Status:  provisional

A block's text becomes the header above the label `Routine_Name:` in
whichever src/*.s file defines it (replacing an existing header there).
A block without a Status line gets `Status:  provisional`.  Lines are
copied as they are, so wrap them to 76 columns.

--status sets the Status line of the named routines' headers (e.g. to
`confirmed`).
"""
import glob
import os
import re
import sys

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../src")
RULE = "; " + "-" * 75


def parse(path):
    blocks, name, text = [], None, []
    with open(path) as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("=== "):
                if name:
                    blocks.append((name, text))
                name, text = line[4:].strip(), []
            elif name:
                text.append(line)
    if name:
        blocks.append((name, text))
    result = []
    for name, text in blocks:
        while text and not text[-1].strip():
            text.pop()
        while text and not text[0].strip():
            text.pop(0)
        if not any(t.startswith("Status:") for t in text):
            text += ["", "Status:  provisional"]
        result.append((name, text))
    return result


def header(name, text):
    lines = [RULE, f"; {name}", ";"]
    lines += [f"; {t}".rstrip() for t in text]
    lines.append(RULE)
    return lines


def apply(files, name, text):
    for path, lines in files.items():
        for i, line in enumerate(lines):
            if line.startswith(name + ":"):
                # Remove an existing header (rule ... rule directly above).
                j = i
                if j > 0 and lines[j - 1] == RULE:
                    k = j - 2
                    while k >= 0 and lines[k] != RULE:
                        k -= 1
                    if k >= 0:
                        del lines[k:j]
                        i = k
                new = header(name, text)
                if i > 0 and lines[i - 1].strip():
                    new.insert(0, "")
                lines[i:i] = new
                return path
    raise SystemExit(f"label {name} not found")


def set_status(files, status, names):
    todo = set(names)
    for path, lines in files.items():
        for i, line in enumerate(lines):
            if line == RULE and i + 1 < len(lines) and lines[i + 1][2:].strip() in todo:
                name = lines[i + 1][2:].strip()
                j = i + 2
                while j < len(lines) and lines[j] != RULE:
                    if lines[j].startswith("; Status:"):
                        lines[j] = f"; Status:  {status}"
                        todo.discard(name)
                        break
                    j += 1
    if todo:
        raise SystemExit("no header with a Status line: " + " ".join(sorted(todo)))


def main():
    files = {}
    for path in sorted(glob.glob(os.path.join(SRC, "*.s"))):
        with open(path) as f:
            files[path] = f.read().split("\n")
    if sys.argv[1:2] == ["--status"]:
        set_status(files, sys.argv[2], sys.argv[3:])
        for path, lines in files.items():
            with open(path, "w") as f:
                f.write("\n".join(lines))
        return 0
    changed = set()
    for arg in sys.argv[1:]:
        for name, text in parse(arg):
            changed.add(apply(files, name, text))
    for path in changed:
        with open(path, "w") as f:
            f.write("\n".join(files[path]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
