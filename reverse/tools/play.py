#!/usr/bin/env python3
"""Play a movie up to a game state, then take over with scripted input.

    play.py --movie MOVIE --until VAR=VALUE[,VAR=VALUE...] [--after N]
            [--press K:BUTTONS[:LEN] ...] [--poke VAR=VALUE ...]
            [--watch VAR ...] [--shot K ...] [--frames N]

Plays MOVIE until all the --until conditions hold (and then N more
frames of the movie, with --after), then stops following the movie: from
there, frame K (counted from the takeover) presses BUTTONS (any of
A B SELECT START UP DOWN LEFT RIGHT, joined with +; LEN frames, default
2) on controller 1 (or 2 with a "2/" prefix, e.g. 2/UP+A).  --poke writes
memory at the takeover.  Prints every change of the --watch variables
with the frame (relative to the takeover) and the writing routine, and
saves screenshots at the --shot frames to build/snap/play-K.png.
Runs --frames frames (default 300) after the takeover.
"""
import argparse
import os
import sys

from PIL import Image

import headless
from watch import resolve

HERE = os.path.dirname(os.path.abspath(__file__))
BUTTONS = {"A": 0x01, "B": 0x02, "SELECT": 0x04, "START": 0x08,
           "UP": 0x10, "DOWN": 0x20, "LEFT": 0x40, "RIGHT": 0x80}


class Done(Exception):
    pass


def parse_press(spec):
    k, _, rest = spec.partition(":")
    buttons, _, n = rest.partition(":")
    pad = 0
    if buttons.startswith("2/"):
        pad, buttons = 1, buttons[2:]
    mask = 0
    for b in buttons.split("+"):
        mask |= BUTTONS[b.upper()]
    return int(k), pad, mask, int(n or 2)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True)
    p.add_argument("--until", required=True)
    p.add_argument("--after", type=int, default=0)
    p.add_argument("--press", action="append", default=[])
    p.add_argument("--poke", action="append", default=[])
    p.add_argument("--watch", action="append", default=[])
    p.add_argument("--shot", action="append", type=int, default=[])
    p.add_argument("--frames", type=int, default=300)
    p.add_argument("--out", default=os.path.join(HERE, "../build/snap"))
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    m = headless.movie(emu, args.movie)
    sym = headless.symbols(emu, args.symbols)
    conds = []
    for c in args.until.split(","):
        var, _, value = c.partition("=")
        conds.append((resolve(sym, var), int(value, 0)))
    presses = [parse_press(s) for s in args.press]
    pokes = []
    for s in args.poke:
        var, _, value = s.partition("=")
        pokes.append((resolve(sym, var), int(value, 0)))
    os.makedirs(args.out, exist_ok=True)
    state = {"start": None, "matched": None, "k": None}

    for var in args.watch:
        addr = resolve(sym, var)

        def callback(cpu, address, value, var=var, addr=addr):
            old = emu.read(addr)
            if value != old and state["k"] is not None:
                print(f"{state['k']:6} {var:<20} {old:02X} -> {value:02X}  {sym.name(cpu.pc)}", flush=True)
            return value

        emu.set_write_callback(addr, callback)

    def pre_frame(f):
        if state["start"] is None:
            if state["matched"] is None and all(emu.read(a) == v for a, v in conds):
                state["matched"] = f
                print(f"conditions hold at movie frame {f}", flush=True)
            if state["matched"] is not None and f >= state["matched"] + args.after:
                state["start"] = f
                for a, v in pokes:
                    emu.write(a, v)
            else:
                m.run_pre_frame()
                return
        k = state["k"] = f - state["start"]
        pads = [0, 0]
        for start, pad, mask, n in presses:
            if start <= k < start + n:
                pads[pad] |= mask
        emu.controller_value(0, pads[0])
        emu.controller_value(1, pads[1])
        if k - 1 in args.shot:
            path = os.path.join(args.out, f"play-{k - 1}.png")
            bmp = path[:-4] + ".bmp"
            emu.screenshot(bmp)
            Image.open(bmp).convert("RGB").save(path)
            os.remove(bmp)
            print(f"{k - 1:6} screenshot {path}", flush=True)
        if k > args.frames:
            raise Done()

    try:
        emu.run_frames(len(m.frames) + 100000, pre_frame)
    except Done:
        pass
    if state["start"] is None:
        print("conditions never held", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
