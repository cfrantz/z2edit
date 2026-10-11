#!/usr/bin/env python3
"""Turn routine-internal loc_ labels into ca65 cheap local labels.

    localize.py [--dry-run] src/bank7.s [...]

A cheap local label (@name) is visible only between two normal labels, so a
loc_ label can become local when every reference to it lies between the
same two normal labels as its definition.  Labels referenced from other
scopes or files stay normal (they are entry points that still need a name).
Each label that becomes local is named by its shape:

- `@done`: its first instruction is rts;
- `@loop`: it is reached only from below (a backward branch or jump);
- `@loc_XXXX` otherwise (mechanical, keeping its address).

A second `@done` or `@loop` in the same scope gets a number (`@loop2`).
Run `make verify` afterwards.
"""
import argparse
import glob
import os
import re
import sys

from rename import realign

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../src")
NORMAL_LABEL = re.compile(r"^([A-Za-z_]\w*)(?::|\s*:=)")
LOCAL_LABEL = re.compile(r"^(@\w+):")
LOC = re.compile(r"\bloc_(?:\d_)?[0-9A-F]{4}\b")
INSTRUCTION = re.compile(r"^\s+([a-z]{3})\b")


def code_part(line):
    return line.split(";", 1)[0]


def localize(path, others, dry_run):
    with open(path) as f:
        lines = f.read().split("\n")

    defs = {}
    for i, line in enumerate(lines):
        m = NORMAL_LABEL.match(line)
        if m and LOC.fullmatch(m.group(1)):
            defs[m.group(1)] = i
    refs = {name: [] for name in defs}
    for i, line in enumerate(lines):
        if NORMAL_LABEL.match(line) or LOCAL_LABEL.match(line):
            continue
        for name in LOC.findall(code_part(line)):
            if name in refs:
                refs[name].append(i)

    # Labels referenced from another file stay normal.
    external = {name for name in defs if any(re.search(rf"\b{name}\b", code_part(t)) for t in others)}

    # Iterate to a fixed point: a label that stays normal splits the scope
    # around it, which may force others to stay normal too.
    keep = set(external)
    while True:
        boundaries = sorted(
            i for i, line in enumerate(lines)
            if (m := NORMAL_LABEL.match(line)) and (m.group(1) not in defs or m.group(1) in keep)
        )

        def scope(i):
            s = -1
            for b in boundaries:
                if b <= i:
                    s = b
                else:
                    break
            return s

        changed = False
        for name, i in defs.items():
            if name in keep:
                continue
            if any(scope(r) != scope(i) for r in refs[name]):
                keep.add(name)
                changed = True
        if not changed:
            break

    # Name the labels that become local.
    taken = {}
    for i, line in enumerate(lines):
        m = LOCAL_LABEL.match(line)
        if m:
            taken.setdefault(scope(i), set()).add(m.group(1))
    renames = {}
    for name, i in sorted(defs.items(), key=lambda kv: kv[1]):
        if name in keep:
            continue
        first = next((INSTRUCTION.match(t) for t in lines[i + 1:] if INSTRUCTION.match(t)), None)
        if first and first.group(1) == "rts":
            base = "@done"
        elif refs[name] and all(r > i for r in refs[name]):
            base = "@loop"
        else:
            base = "@loc_" + name[-4:]
        used = taken.setdefault(scope(i), set())
        new, n = base, 2
        while new in used:
            new, n = f"{base}{n}", n + 1
        used.add(new)
        renames[name] = new

    for i, line in enumerate(lines):
        m = NORMAL_LABEL.match(line)
        if m and m.group(1) in renames:
            lines[i] = renames[m.group(1)] + line[len(m.group(1)):]
            continue
        code, sep, comment = line.partition(";")
        new = LOC.sub(lambda m: renames.get(m.group(0), m.group(0)), code)
        if new != code:
            lines[i] = realign(new + sep + comment)

    print(f"{os.path.basename(path)}: {len(renames)} local, {len(keep)} kept "
          f"({len(external)} referenced from other files)")
    if not dry_run:
        with open(path, "w") as f:
            f.write("\n".join(lines))
    return keep


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--dry-run", action="store_true")
    p.add_argument("--list-kept", action="store_true", help="List labels that stay normal")
    p.add_argument("files", nargs="+")
    args = p.parse_args()
    all_files = sorted(glob.glob(os.path.join(SRC, "*.s")) + glob.glob(os.path.join(SRC, "*.inc")))
    for path in args.files:
        others = []
        for o in all_files:
            if os.path.abspath(o) != os.path.abspath(path):
                with open(o) as f:
                    others.extend(f.read().split("\n"))
        keep = localize(path, others, args.dry_run)
        if args.list_kept:
            for name in sorted(keep):
                print("  " + name)
    return 0


if __name__ == "__main__":
    sys.exit(main())
