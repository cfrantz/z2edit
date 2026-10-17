#!/usr/bin/env python3
"""Compare the code and tables the five area banks (1-5) share.

    bank_compare.py [--diff STEM] [--renamed] [--min N]

Splits src/bank1.s-bank5.s into blocks at their global labels (a block
runs to the next global label) and normalizes each block's lines:
comments and address comments dropped, `_BankN` / `_BankNB` suffixes
removed (bank 4's first-group copies have none), local labels renumbered in order of appearance, and
`.assert` lines skipped.  In a block with instructions, data lines
(.byte, .word, .dbyt) and `NAME := ...` aliases are dropped too (they are
tables that happen to follow the code), except the .word lines of an
inline jump table (after `jsr JumpTable`), and a branch or jump to a label
whose code is just `rts` (a local one, or another routine's) becomes a
branch to `RTS`, so copies that reach different nearby `rts`es match.  Blocks are then grouped by name stem (the
label without its `_BankN` suffix; bank 4's second copies, `_Bank4B`,
count as bank "4B").

For each stem in two or more banks it prints the banks and which of them
hold identical copies (copies separated by `|`), e.g.

    Enemy_Walk  1 2 4 4B 5 | 3      (6 lines)

--diff STEM prints the differences between the copies of one stem
(unified diffs against the first bank's copy).  --renamed lists blocks of
at least --min lines (default 6) whose normalized bodies are identical
but whose stems differ, across or within banks (copies under other
names).  --similar lists pairs of blocks of at least --min lines, in
different banks and with different stems, whose bodies are at least
--ratio alike (default 0.8; difflib's ratio), e.g. bank 3's Town_*
routines against the others' Area_* ones.
"""
import argparse
import difflib
import os
import re
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../src")
BANKS = [1, 2, 3, 4, 5]
GLOBAL = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*):")
SUFFIX = re.compile(r"_Bank([1-5])(B?)\b")
LOCAL = re.compile(r"@[A-Za-z_][A-Za-z0-9_]*")


def blocks(bank):
    """Yield (label, raw lines) for each global label of a bank."""
    label, lines = None, []
    with open(os.path.join(SRC, f"bank{bank}.s")) as f:
        for line in f:
            m = GLOBAL.match(line)
            if m:
                if label:
                    yield label, lines
                label, lines = m.group(1), []
            elif label:
                lines.append(line)
    if label:
        yield label, lines


DATA = re.compile(r"^\s*(\.byte|\.word|\.dbyt|\.res)\b|:=")
BRANCH = re.compile(r"^(b[a-z]{2}|jmp) (\S+)$")


def strip(lines):
    return [" ".join(SUFFIX.sub("", ln.split(";", 1)[0]).split()) for ln in lines]


def rts_labels(lines):
    """Local labels in lines whose next statement is rts."""
    out, pending = set(), []
    for ln in strip(lines):
        if not ln or ln.startswith(".assert"):
            continue
        if ln.endswith(":") and ln.startswith("@"):
            pending.append(ln[:-1])
            continue
        if ln == "rts":
            out.update(pending)
        pending = []
    return out


def normalize(lines, rts_globals):
    code_lines = [ln for ln in strip(lines) if ln and not ln.startswith(".assert")]
    has_code = any(not DATA.search(ln) and not ln.endswith(":") for ln in code_lines)
    local_rts = rts_labels(lines)
    out, locals_ = [], {}
    inline = False
    for ln in code_lines:
        # an inline jump table (the .word lines after jsr JumpTable) is code
        if ln == "jsr JumpTable":
            inline = True
        elif not ln.startswith(".word"):
            inline = inline and ln.endswith(":")
        if has_code and DATA.search(ln) and not (inline and ln.startswith(".word")):
            continue
        m = BRANCH.match(ln)
        if m and (m.group(2) in local_rts or m.group(2) in rts_globals):
            ln = f"{m.group(1)} RTS"

        def local(mm):
            return locals_.setdefault(mm.group(0), f"@L{len(locals_)}")

        out.append(LOCAL.sub(local, ln))
    # labels of rts-only targets that nothing else uses are noise
    return out


def bank_key(bank, label):
    m = SUFFIX.search(label)
    if m and m.group(2):
        return f"{bank}B"
    return str(bank)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--diff")
    p.add_argument("--renamed", action="store_true")
    p.add_argument("--min", type=int, default=6)
    p.add_argument("--similar", action="store_true")
    p.add_argument("--ratio", type=float, default=0.8)
    a = p.parse_args()

    raw = {bank: list(blocks(bank)) for bank in BANKS}
    rts_globals = set()
    for bank in BANKS:
        for label, lines in raw[bank]:
            body = [ln for ln in strip(lines) if ln and not ln.startswith(".assert")]
            if body[:1] == ["rts"]:
                rts_globals.add(SUFFIX.sub("", label))

    stems = defaultdict(dict)       # stem -> bank key -> normalized lines
    names = {}                      # (stem, bank key) -> label
    for bank in BANKS:
        for label, lines in raw[bank]:
            stem = SUFFIX.sub("", label)
            key = bank_key(bank, label)
            stems[stem][key] = normalize(lines, rts_globals)
            names[(stem, key)] = label

    if a.diff:
        copies = stems[a.diff]
        keys = list(copies)
        base = keys[0]
        for k in keys[1:]:
            d = list(difflib.unified_diff(copies[base], copies[k], f"bank {base}", f"bank {k}", lineterm="", n=1))
            print(f"== {names[(a.diff, base)]} vs {names[(a.diff, k)]}: " + ("identical" if not d else ""))
            for line in d[2:]:
                print("  " + line)
        return

    if a.similar:
        items = [(k, s, b) for s, copies in stems.items() for k, b in copies.items() if len(b) >= a.min]
        seen = set()
        for i, (k1, s1, b1) in enumerate(items):
            for k2, s2, b2 in items[i + 1:]:
                if s1 == s2 or k1[0] == k2[0] or (s1, s2) in seen:
                    continue
                if min(len(b1), len(b2)) < a.ratio * max(len(b1), len(b2)):
                    continue
                r = difflib.SequenceMatcher(None, b1, b2, autojunk=False).ratio()
                if r >= a.ratio:
                    seen.add((s1, s2))
                    print(f"{r:.2f} {len(b1):3}/{len(b2):<3} {names[(s1, k1)]} ({k1}) ~ {names[(s2, k2)]} ({k2})")
        return

    if a.renamed:
        by_body = defaultdict(list)
        for stem, copies in stems.items():
            for k, body in copies.items():
                if len(body) >= a.min:
                    by_body[tuple(body)].append((k, stem))
        for body, where in sorted(by_body.items(), key=lambda kv: -len(kv[0])):
            if len({s for _, s in where}) > 1:
                print(f"{len(body):4} lines: " + ", ".join(f"{names[(s, k)]} ({k})" for k, s in sorted(where)))
        return

    for stem in sorted(stems):
        copies = stems[stem]
        if len(copies) < 2:
            continue
        groups = []
        for k, body in copies.items():
            for g in groups:
                if copies[g[0]] == body:
                    g.append(k)
                    break
            else:
                groups.append([k])
        size = max(len(b) for b in copies.values())
        print(f"{stem:40} {' | '.join(' '.join(g) for g in groups):24} ({size} lines)")


if __name__ == "__main__":
    main()
