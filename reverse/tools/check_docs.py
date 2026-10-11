#!/usr/bin/env python3
"""Report symbol names in docs/ that no longer exist in the source.

    check_docs.py

Looks for `code spans` and mechanical names (sub_, loc_, dat_, ram_,
sram_) in docs/**/*.md and reports those that are not labels or equates
in build/zelda2.sym.json.  NAME_BankN stands for per-bank copies
(NAME_Bank1 ...).  Run after renaming symbols.
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..")
MECHANICAL = re.compile(r"\b(?:sub|loc|dat|s?ram)_(?:\d_)?[0-9A-F]{4}\b")
SPAN = re.compile(r"`([A-Za-z_][\w:]*)(?:[+,][^`]*)?`")
IDENT = re.compile(r"^[A-Za-z_]\w*(?:::\w+)?$")


def main():
    with open(os.path.join(ROOT, "build/zelda2.sym.json")) as f:
        d = json.load(f)
    names = {s["name"] for s in d["labels"]} | {e["name"] for e in d["equates"]}
    # Macros and scopes are not in the symbol table.
    for path in glob.glob(os.path.join(ROOT, "src/*")):
        names |= set(re.findall(r"^\s*\.(?:macro|scope)\s+(\w+)", open(path).read(), flags=re.M))
    bad = 0
    for path in sorted(glob.glob(os.path.join(ROOT, "docs/**/*.md"), recursive=True)):
        for n, line in enumerate(open(path), 1):
            found = set(MECHANICAL.findall(line))
            for m in SPAN.finditer(line):
                word = m.group(1)
                # Only check things that look like symbols: camelCase,
                # PascalCase_With_Underscores or UPPER_SNAKE constants.
                if IDENT.match(word) and "::" not in word and re.search(r"[a-z][A-Z]|_|^[A-Z][a-z]+[A-Z]", word):
                    found.add(word)
            for word in sorted(found):
                # NAME_BankN stands for the per-bank copies NAME_Bank1, ...
                if word.endswith("_BankN") and any(re.fullmatch(re.escape(word[:-1]) + r"\dB?", x) for x in names):
                    continue
                if word not in names:
                    print(f"{os.path.relpath(path, ROOT)}:{n}: {word}")
                    bad += 1
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
