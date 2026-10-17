#!/usr/bin/env python3
"""List the objects of every side view area of an area bank, with the
routine that draws each.

    area_objects.py BANK [--group 1|2] [--by-routine] [--area N]

Decodes each area's data as Area_DecodeObjects does: a 4-byte header, then
objects of a position byte (row in the high nibble, column relative to the
previous object's) and a type byte; row $D changes the floor pattern
(position only), row $E skips to the start of page (pos & 3) and reads the
next position, type $0F is an item with one more byte.  Types $10 and up
are large objects, dispatched by (type >> 4) - 1, plus $0F on row $F
(AreaBank::DrawLargeObject, Town_DrawLargeObject in bank 3); the rest are
small objects (AreaBank::DrawSmallObject; Town_DrawSmallObject in bank 3,
plus $10 on row $F).

Each object is printed as  page:column row type -> routine, after the
background layers (AreaBank::BackgroundAreas, "bg N": the layer an area
with N in header byte 3 bits 2-0 is drawn over).
--by-routine instead lists, for each routine, the areas and objects that
use it (the first few of each), which is what to load to test it.
GROUP 2 is the second table (AreaBank::AreaPtrs2: Death Mountain, Maze
Island, palaces 3/4/6); banks 3 and 5 have none.
"""
import argparse
import json
import os
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROM = os.path.join(HERE, "../build/zelda2.nes")
SYM = os.path.join(HERE, "../build/zelda2.sym.json")
AREA_PTRS = {1: 0x8523, 2: 0xA000}
AREA_COUNT = 63


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("bank", type=int)
    p.add_argument("--group", type=int, default=1)
    p.add_argument("--by-routine", action="store_true")
    p.add_argument("--area", type=lambda s: int(s, 0))
    a = p.parse_args()

    if a.group == 2 and a.bank in (3, 5):
        p.error("banks 3 and 5 have only one group of areas")
    rom = open(ROM, "rb").read()
    prg = rom[16:16 + 8 * 0x4000]
    labels = {}
    for s in json.load(open(SYM))["labels"]:
        if not s["name"].startswith("@"):
            labels.setdefault((s["bank"], s["addr"]), s["name"])
    addrs = {name: addr for (bank, addr), name in labels.items() if bank in (a.bank, 7)}

    def rd(addr):
        if addr >= 0xC000:
            return prg[7 * 0x4000 + addr - 0xC000]
        return prg[a.bank * 0x4000 + addr - 0x8000]

    def word(addr):
        return rd(addr) | rd(addr + 1) << 8

    def name(addr):
        b = 7 if addr >= 0xC000 else a.bank
        return labels.get((b, addr), f"${addr:04X}")

    def jump_table(routine):
        # The table follows the routine's `jsr JumpTable`.
        start, jt = addrs[routine], addrs["JumpTable"]
        for pc in range(start, start + 64):
            if rd(pc) == 0x20 and word(pc + 1) == jt:
                return pc + 3
        raise SystemExit(f"no jump table after {routine}")

    if a.bank == 3:
        large, small = jump_table("Town_DrawLargeObject"), jump_table("Town_DrawSmallObject")
    else:
        large = jump_table(f"Area_DrawLargeObject_Bank{a.bank}")
        small = jump_table(f"Area_DrawSmallObject_Bank{a.bank}")

    uses = defaultdict(list)

    def decode(ptr):
        length = rd(ptr)
        y, column, objs = 4, 0, []
        while y < length:
            pos = rd(ptr + y)
            if pos >> 4 == 0xE:
                column = (pos & 3) * 16
                y += 2
                pos = rd(ptr + y)
            typ = rd(ptr + y + 1)
            y += 2
            column += pos & 0x0F
            row = pos >> 4
            where = f"{column >> 4}:{column & 15:X}"
            if row == 0xD:
                objs.append((where, row, typ, "floor pattern"))
                continue
            if typ & 0xF0:
                index = (typ >> 4) - 1 + (0x0F if row == 0xF else 0)
                routine = name(word(large + 2 * index))
            elif typ == 0x0F:
                routine = f"Area_PlaceItem (kind ${rd(ptr + y):02X})"
                y += 1
            else:
                index = typ + (0x10 if a.bank == 3 and row == 0xF else 0)
                routine = name(word(small + 2 * index))
            objs.append((where, row, typ, routine))
        return length, objs

    def show(label, ptr):
        length, objs = decode(ptr)
        for where, row, typ, routine in objs:
            if routine != "floor pattern":
                uses[routine.split(" ")[0]].append((label, where, row, typ))
        if not a.by_routine:
            print(f"{label} (${ptr:04X}, {length} bytes, header {' '.join(f'{rd(ptr + k):02X}' for k in range(1, 4))}):")
            for where, row, typ, routine in objs:
                print(f"    {where:>5} row ${row:X} type ${typ:02X} -> {routine}")

    if a.area is None:
        # Background layers (header byte 3 bits 2-0 = n: BackgroundAreas[n - 1]).
        for n in range(1, 8):
            ptr = word(0x8000 + 2 * (n - 1))
            if 0x8000 <= ptr < 0xC000:
                show(f"bg {n}", ptr)
    base = AREA_PTRS[a.group]
    for area in range(AREA_COUNT):
        if a.area is not None and area != a.area:
            continue
        ptr = word(base + 2 * area)
        if 0x8000 <= ptr < 0xC000:
            show(f"area ${area:02X}", ptr)
    if a.by_routine:
        for routine in sorted(uses):
            u = uses[routine]
            first = ", ".join(f"{ar} {w} r{r:X} ${t:02X}" for ar, w, r, t in u[:6])
            print(f"{routine}: {len(u)} uses; {first}")


if __name__ == "__main__":
    main()
