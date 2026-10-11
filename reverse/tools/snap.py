#!/usr/bin/env python3
"""Take screenshots and memory dumps at chosen frames of a movie.

    snap.py --movie MOVIE [--out DIR] [--dump VAR[:LEN] ...] FRAME [FRAME ...]

For each FRAME, saves the screen as DIR/<movie>-<frame>.png and prints
the bytes of each --dump variable (a symbol from ram.inc, optionally
NAME+k, or a hex address; LEN in decimal, default 1).  `--when VAR=VALUE`
instead takes a snapshot at the first frame of each run in which VAR has
VALUE (at most --limit of them), for finding the screen of a game mode.
"""
import argparse
import os
import sys

from PIL import Image

import headless
from watch import resolve

HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True, help="FM2 movie (FILE or FILE:OFFSET)")
    p.add_argument("--out", default=os.path.join(HERE, "../build/snap"))
    p.add_argument("--dump", action="append", default=[], metavar="VAR[:LEN]")
    p.add_argument("--when", action="append", default=[], metavar="VAR=VALUE",
                   help="Snapshot where all the conditions start to hold")
    p.add_argument("--delay", type=int, default=0, help="Frames to wait after a --when match")
    p.add_argument("--limit", type=int, default=4)
    p.add_argument("frames", nargs="*", type=int)
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    m = headless.movie(emu, args.movie)
    sym = headless.symbols(emu, args.symbols)
    os.makedirs(args.out, exist_ok=True)
    stem = os.path.splitext(os.path.basename(args.movie.rpartition(":")[0] or args.movie))[0]
    dumps = []
    for d in args.dump:
        var, _, n = d.partition(":")
        dumps.append((var, resolve(sym, var), int(n or 1)))
    conds = []
    for w in args.when:
        var, _, value = w.partition("=")
        conds.append((resolve(sym, var), int(value, 0)))
    wanted = set(args.frames)
    last = max(wanted) if wanted and not conds else len(m.frames) + 60
    state = {"held": False, "taken": 0, "pending": None}

    def snapshot(frame):
        path = os.path.join(args.out, f"{stem}-{frame}.png")
        bmp = path[:-4] + ".bmp"
        emu.screenshot(bmp)
        Image.open(bmp).convert("RGB").save(path)
        os.remove(bmp)
        print(f"frame {frame}: {path}")
        for var, addr, n in dumps:
            print(f"  {var:<20} " + " ".join(f"{emu.read(addr + k):02X}" for k in range(n)))
        sys.stdout.flush()

    def pre_frame(f):
        # The screen and memory now reflect the end of frame f - 1.
        if f - 1 in wanted:
            snapshot(f - 1)
        if conds:
            hold = all(emu.read(a) == v for a, v in conds)
            if hold and not state["held"] and state["taken"] < args.limit:
                state["pending"] = f - 1 + args.delay
                state["taken"] += 1
            state["held"] = hold
            if state["pending"] == f - 1:
                snapshot(f - 1)
                state["pending"] = None
        m.run_pre_frame()

    emu.run_frames(last + 2, pre_frame)
    return 0


if __name__ == "__main__":
    sys.exit(main())
