#!/usr/bin/env python3
"""Turn routine-internal loc_ labels into ca65 cheap local labels.

    localize.py [--dry-run] src/bank7.s [...]

A cheap local label (@name) is visible only between two normal labels, so a
loc_ label can become local when every reference to it lies between the
same two normal labels as its definition.  Labels referenced from other
scopes or files stay normal (they are entry points that still need a name).
Each label that becomes local is named by its shape:

- `@done`: its first instruction is rts;
- `@loop`: it is reached only from below, by a conditional branch with
  no rts, rti or jmp between the label and the branch;
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


def is_loop(lines, i, refs):
    """Whether label line i heads a loop closed by one of refs."""
    for r in refs:
        op = INSTRUCTION.match(lines[r])
        if not op or op.group(1) in ("jmp", "jsr") or not op.group(1).startswith("b") or op.group(1) == "bit":
            continue
        between = [INSTRUCTION.match(t) for t in lines[i + 1:r]]
        if not any(m and m.group(1) in ("rts", "rti", "jmp") for m in between):
            return True
    return False


def relabel_loops(path, dry_run):
    """Re-check @loop labels named by an earlier version of this tool."""
    with open(path) as f:
        lines = f.read().split("\n")
    scope_start = [0] * len(lines)
    s = 0
    for i, line in enumerate(lines):
        if NORMAL_LABEL.match(line):
            s = i
        scope_start[i] = s
    scopes = {}
    for i, line in enumerate(lines):
        m = re.match(r"^(@loop\d*):", line)
        if m:
            scopes.setdefault(scope_start[i], []).append((i, m.group(1)))
    renames = []
    for start, labels in scopes.items():
        end = next((j for j in range(start + 1, len(lines)) if NORMAL_LABEL.match(lines[j])), len(lines))
        n = 0
        for i, old in labels:
            refs = [j for j in range(start, end) if re.search(rf"(?<![\w@]){old}\b", code_part(lines[j])) and j != i]
            if is_loop(lines, i, refs):
                n += 1
                new = "@loop" if n == 1 else f"@loop{n}"
            else:
                addr = next(re.search(r"; \$([0-9A-F]{4})", t).group(1) for t in lines[i + 1:] if re.search(r"; \$[0-9A-F]{4}", t))
                new = f"@loc_{addr}"
            renames.append((start, end, i, old, new))
    # Apply per scope, via temporary names to allow swaps.
    for start, end, i, old, new in renames:
        tmp = "@__" + new[1:]
        for j in range(start, end):
            if j == i:
                lines[j] = tmp + lines[j][len(old):]
            else:
                code, sep, com = lines[j].partition(";")
                new_code = re.sub(rf"(?<![\w@]){old}\b", tmp, code)
                if new_code != code:
                    lines[j] = realign(new_code + sep + com)
    text = "\n".join(lines).replace("@__", "@")
    changed = sum(1 for r in renames if r[3] != r[4])
    print(f"{os.path.basename(path)}: {changed} loop labels renamed")
    if not dry_run:
        with open(path, "w") as f:
            f.write(text)


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
        elif refs[name] and all(r > i for r in refs[name]) and is_loop(lines, i, refs[name]):
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
    p.add_argument("--relabel-loops", action="store_true", help="Re-check existing @loop labels")
    p.add_argument("files", nargs="+")
    args = p.parse_args()
    if args.relabel_loops:
        for path in args.files:
            relabel_loops(path, args.dry_run)
        return 0
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
