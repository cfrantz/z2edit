#!/usr/bin/env python3
"""Load side view areas in the emulator and print the tile maps the game
builds for them.

    area_dump.py --movie MOVIE --at VAR=VALUE,... --until VAR=VALUE [--after N] \
        --areas N[,N...] [--pages P[,P...]] [--link X] [--set VAR=VALUE,...]

Starts a game at an area as play.py --at does, waits for --until (and
--after more frames), then for each area in --areas sets areaId and
gameMode = GAME_MODE_LOAD_ROOM, which makes Main_Loop load that area of
the current bank and group (Area_Load), and waits until the side view
runs again (gameMode $0B).  It then prints sramAreaTiles, one page after
another (13 rows of 16 tiles, row labels as the area data's rows).  Link
is first moved to X --link (default $20), and the --set variables are
written (e.g. currentRegion=1 for the second area group of banks 1 and 2).  The screen isn't redrawn
properly after such a load, so there are no screenshots.
"""
import argparse
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import headless  # noqa: E402
from z2edit import Address  # noqa: E402

PAGE = 0xD0
MODE_SIDE_VIEW, MODE_LOAD_ROOM = 0x0B, 0x14


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True)
    p.add_argument("--at", required=True)
    p.add_argument("--until", required=True)
    p.add_argument("--after", type=int, default=60)
    p.add_argument("--areas", required=True)
    p.add_argument("--pages", default="0,1,2,3")
    p.add_argument("--set", default="")
    p.add_argument("--link", default="20", help="Link's X (hex, 16-bit) in each area; the screen follows him")
    a = p.parse_args()

    emu = headless.emulator(a.rom)
    mv = headless.movie(emu, a.movie)
    eq = headless.symbols(emu, a.symbols).equates
    rd, wr = emu.read, emu.write

    def resolve(spec):
        name, _, k = spec.partition("+")
        base = eq[name] if name in eq else int(name, 16)
        return base + (int(k, 0) if k else 0)

    code = []
    for c in a.at.split(","):
        var, _, value = c.partition("=")
        addr = resolve(var)
        code += [0xA9, int(value, 0), 0x8D, addr & 0xFF, addr >> 8]
    code.append(0x60)
    for k, b in enumerate(code):
        emu[Address.Prg(0, 0xAA3F + k)] = b
    conds = [(resolve(v), int(x, 0)) for v, _, x in (c.partition("=") for c in a.until.split(","))]

    state = {"phase": "start", "wait": 0, "areas": [int(x, 0) for x in a.areas.split(",")]}

    class Done(Exception):
        pass

    def dump(area):
        print(f"area ${area:02X}", flush=True)
        for page in [int(x) for x in a.pages.split(",")]:
            print(f"  page {page}")
            for row in range(13):
                base = eq["sramAreaTiles"] + page * PAGE + row * 16
                print(f"    {row:X}: " + " ".join(f"{rd(base + c):02X}" for c in range(16)))

    def pre_frame(f):
        if state["phase"] == "start":
            mv.run_pre_frame()
            if all(rd(ad) == v for ad, v in conds):
                state["phase"], state["wait"] = "after", a.after
            return
        emu.controller_value(0, 0)
        emu.controller_value(1, 0)
        if state["phase"] == "after":
            state["wait"] -= 1
            if state["wait"] <= 0:
                state["phase"] = "next"
            return
        if state["phase"] == "next":
            if not state["areas"]:
                raise Done()
            state["area"] = state["areas"].pop(0)
            wr(eq["areaId"], state["area"])
            for c in filter(None, a.set.split(",")):
                var, _, value = c.partition("=")
                wr(resolve(var), int(value, 0))
            wr(eq["objX"], int(a.link, 16) & 0xFF)
            wr(eq["objXHi"], int(a.link, 16) >> 8)
            wr(eq["gameMode"], MODE_LOAD_ROOM)
            state["phase"], state["wait"] = "loading", 0
            return
        if state["phase"] == "loading":
            state["wait"] += 1
            if rd(eq["gameMode"]) == MODE_SIDE_VIEW and state["wait"] > 120:
                dump(state["area"])
                state["phase"] = "next"
            elif state["wait"] > 600:
                print(f"area ${state['area']:02X}: did not load (gameMode ${rd(eq['gameMode']):02X})")
                state["phase"] = "next"

    try:
        emu.run_frames(10 ** 6, pre_frame)
    except Done:
        pass


if __name__ == "__main__":
    main()
