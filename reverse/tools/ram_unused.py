#!/usr/bin/env python3
"""Inventory the RAM and SRAM that the game apparently doesn't use.

    ram_unused.py [--log PREFIX] [--all] [--ranges]

Combines two kinds of evidence for every address in $0000-$07FF and
$6000-$7FFF:

Dynamic, from the RAM access log of the test movies (cdl_movie.py's
<PREFIX>.ram.json, default build/zelda2, all movies merged).  Each logged
access is made by an instruction, which is one of:
  clear  a write by an instruction in a RAM clearing routine (Ram_Clear*),
         or an indexed write touching more than 32 addresses that only
         ever stores (a clearing loop);
         the overworld's row buffers ($0410-$04CF, Overworld_*) are uses;
  copy   an indexed or indirect access by the save routines (Save_*) or
         by Mode_LoadWorld (their block copies into SRAM; in worlds without an
         overworld map it reads the stack page as one), or by an indexed
         instruction touching more than 32 addresses (block copies, e.g.
         a table copied to SRAM): the byte moves, but nothing looks at it;
  use    anything else: a direct, indexed or pointer access.

Static, from the source: an address named directly by an instruction
operand (also in code the movies never ran), or inside the extent of an
array declared in ram.inc (`[N]` in its comment) that indexed
instructions use.

Each address gets a status:
  used        read and written by uses
  read-only   read by uses, set only by clears/copies or not at all
  write-only  written by uses, never read by one
  copied      only copied (and maybe cleared)
  cleared     only cleared
  untouched   never accessed in the movies
  table       only copied, but its copy block has bytes in use: an entry
              of a table in use (e.g. an enemy type the movies never met)
  element     not used itself, inside a declared array that is used
plus `static` (named directly by code), the ram.inc name covering it, and
the nearest base of an indexed operand in the source up to 16 bytes
below the range (unexecuted indexed code could reach it).
The OAM page ($0200-$02FF) counts as used: OAM DMA reads it.  By default the report lists ranges whose
status isn't used/read-only; --all lists every range.  The stack page is
special: pushes aren't logged (tools/stack_depth.py measures them); see
docs/memory/unused-ram.md.
"""
import argparse
import bisect
import collections
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..")
BULK = 32
READ, WRITE, POINTER = 1, 2, 4
DECL = re.compile(r"^([A-Za-z_]\w*)\s*=\s*(\$[0-9A-Fa-f]+|[A-Za-z_]\w*(?:[+-]\d+)?)\s*(?:;\s*(?:\[(\d+)\])?(.*))?$")
OPERAND = re.compile(r"^\s+[a-z]{3}\s+(?:a:)?\(?([A-Za-z_]\w*|\$[0-9A-F]{2,4})(?:\s*([+-])\s*(\$?[0-9A-Fa-f]+|\d+))?\)?(,[xy])?\)?(?:,y)?\s*(?:;|$)")


def load_decls():
    """name -> (address, size or None, comment)"""
    raw = {}
    for path in ("src/ram.inc",):
        for line in open(os.path.join(ROOT, path)):
            m = DECL.match(line.strip())
            if m:
                raw[m.group(1)] = (m.group(2), int(m.group(3)) if m.group(3) else None, (m.group(4) or "").strip())
    decls = {}

    def value(expr, depth=0):
        if expr.startswith("$"):
            return int(expr[1:], 16)
        m = re.match(r"([A-Za-z_]\w*)([+-]\d+)?$", expr)
        if m and m.group(1) in raw and depth < 8:
            base = value(raw[m.group(1)][0], depth + 1)
            return None if base is None else base + int(m.group(2) or 0)
        return None

    for name, (expr, size, comment) in raw.items():
        v = value(expr)
        if v is not None and (v < 0x800 or 0x6000 <= v < 0x8000):
            decls[name] = (v, size, comment)
    return decls


def routine_starts():
    with open(os.path.join(ROOT, "build/zelda2.sym.json")) as f:
        labels = json.load(f)["labels"]
    by_bank = collections.defaultdict(list)
    for s in labels:
        if not s["name"].startswith(("@", "loc_")):
            by_bank[s["bank"]].append((s["addr"], s["name"]))
    for v in by_bank.values():
        v.sort()
    return by_bank


def routine_of(by_bank, offset):
    bank, off = divmod(offset, 0x4000)
    addr = (0xC000 if bank == 7 else 0x8000) + off
    rows = by_bank[bank]
    i = bisect.bisect_right(rows, (addr, "￿")) - 1
    return rows[i][1] if i >= 0 else f"${addr:04X}"


def static_refs(decls, bases=None):
    """Addresses named directly by instruction operands in src/*.s (and,
    in bases, the base address of each indexed operand)."""
    names = {n: v for n, (v, _, _) in decls.items()}
    out = collections.defaultdict(set)   # addr -> files
    for fn in sorted(os.listdir(os.path.join(ROOT, "src"))):
        if not fn.endswith(".s"):
            continue
        for line in open(os.path.join(ROOT, "src", fn)):
            if "#" in line.split(";")[0]:
                continue
            m = OPERAND.match(line)
            if not m:
                continue
            sym, sign, k = m.group(1), m.group(2), m.group(3)
            if sym.startswith("$"):
                addr = int(sym[1:], 16)
            elif sym in names:
                addr = names[sym]
            else:
                continue
            if k:
                kv = int(k[1:], 16) if k.startswith("$") else int(k)
                addr += kv if sign == "+" else -kv
            out[addr].add(fn)
            if bases is not None and m.group(4):
                bases.add(addr)
    return out


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--log", default=os.path.join(ROOT, "build/zelda2"))
    p.add_argument("--all", action="store_true")
    a = p.parse_args()

    decls = load_decls()
    by_bank = routine_starts()
    rows = json.load(open(a.log + ".ram.json"))
    touched = collections.defaultdict(set)
    flags_of = collections.defaultdict(int)
    for src, addr, fl in rows:
        touched[src].add(addr)
        flags_of[src] |= fl
    rom = open(os.path.join(ROOT, "build/zelda2.nes"), "rb").read()[16:]
    indexed_ops = {0x95, 0x9D, 0x99, 0xB5, 0xBD, 0xB9, 0xB6, 0xBE, 0x96, 0x94, 0xB4, 0xBC,
                   0x15, 0x1D, 0x19, 0x35, 0x3D, 0x39, 0x55, 0x5D, 0x59, 0x75, 0x7D, 0x79,
                   0xD5, 0xDD, 0xD9, 0xF5, 0xFD, 0xF9, 0xF6, 0xFE, 0xD6, 0xDE, 0x16, 0x1E,
                   0x36, 0x3E, 0x56, 0x5E, 0x76, 0x7E}

    def kind(src):
        name = routine_of(by_bank, src)
        if name.startswith("Ram_Clear"):
            return "clear"
        if (name.startswith("Save_") or name == "Mode_LoadWorld") and \
                (rom[src] in indexed_ops or rom[src] & 0x1F in (0x01, 0x11)):
            return "copy"
        if rom[src] in indexed_ops and len(touched[src]) > BULK and not name.startswith("Overworld_"):
            return "clear" if flags_of[src] == WRITE else "copy"
        return "use"

    kinds = {src: kind(src) for src in touched}
    acc = collections.defaultdict(lambda: collections.defaultdict(int))  # addr -> kind -> flags
    for src, addr, fl in rows:
        acc[addr][kinds[src]] |= fl

    bases = set()
    static = static_refs(decls, bases)
    sorted_bases = sorted(bases)

    def near_base(addr):
        """The closest indexed base at most 16 bytes below addr, if any."""
        i = bisect.bisect_right(sorted_bases, addr) - 1
        return sorted_bases[i] if i >= 0 and addr - sorted_bases[i] <= 16 else None
    # Declared names covering each address (exact, or inside an array).
    cover = {}
    for name, (v, size, _) in sorted(decls.items(), key=lambda kv: (kv[1][1] or 1)):
        for i in range(size or 1):
            cover.setdefault(v + i, name if i == 0 else f"{name}+{i}")

    def status(addr):
        k = acc.get(addr, {})
        use = k.get("use", 0)
        if use & READ and use & (WRITE | POINTER):
            return "used"
        if use & (READ | POINTER):
            return "read-only"
        if use & WRITE:
            return "write-only"
        if "copy" in k:
            return "copied"
        if "clear" in k:
            return "cleared"
        return "untouched"

    addrs = list(range(0x0000, 0x0800)) + list(range(0x6000, 0x8000))
    base = {addr: status(addr) for addr in addrs}
    live = {"used", "read-only", "write-only"}
    # A copied byte belongs to a table in use if its copy block (every
    # address the copying instructions touch) has a byte that is used.
    blocks = collections.defaultdict(set)    # addr -> copy instructions
    for src, addrs_ in touched.items():
        if kinds[src] == "copy":
            for x in addrs_:
                blocks[x].add(src)
    for addr in addrs:
        if base[addr] == "copied" and any(base.get(x) in live for s in blocks[addr] for x in touched[s]):
            base[addr] = "table"
    # Declared arrays in use: their other elements are part of a live structure.
    for name, (v, size, _) in decls.items():
        if size and size > 1 and any(base.get(v + i) in live | {"table"} for i in range(size)):
            for i in range(size):
                if base.get(v + i) in ("copied", "cleared", "untouched"):
                    base[v + i] = "element"
    for addr in range(0x0200, 0x0300):   # OAM DMA reads the page
        if base[addr] in ("write-only", "cleared", "untouched"):
            base[addr] = "used"

    rowsout = []
    for addr in addrs:
        st = base[addr]
        name = cover.get(addr, "")
        stat = "static" if addr in static else ""
        rowsout.append((addr, st, stat, re.sub(r"\+\d+$", "", name)))
    # merge consecutive addresses with the same status, static flag and name
    ranges = []
    for addr, st, stat, name in rowsout:
        if ranges and ranges[-1][1] == addr - 1 and tuple(ranges[-1][2:]) == (st, stat, name):
            ranges[-1][1] = addr
        else:
            ranges.append([addr, addr, st, stat, name])
    for lo, hi, st, stat, name in ranges:
        if not a.all and st in ("used", "read-only", "table", "element"):
            continue
        span = f"${lo:04X}" if lo == hi else f"${lo:04X}-${hi:04X}"
        nb = near_base(lo)
        near = f"  (indexed base ${nb:04X} {lo - nb} below)" if nb is not None and nb != lo else ""
        print(f"{span:13} {hi - lo + 1:5}  {st:10} {stat:6} {name}{near}")


if __name__ == "__main__":
    main()
