#!/usr/bin/env python3
"""Tally the callers of routines, and register values on entry, during a movie.

    callers.py --movie MOVIE [--frames N] [--reg x] ROUTINE [ROUTINE ...]

For each routine, prints how many times it was entered from each JSR site
(by symbol name) with each value of the chosen register.  The caller is
taken from the return address on the stack, so it is only meaningful for
routines entered by JSR.
"""
import argparse
import collections
import os
import sys

import headless

HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True, help="FM2 movie (FILE or FILE:OFFSET)")
    p.add_argument("--frames", type=int, default=0, help="Frames to play (0 = whole movie)")
    p.add_argument("--reg", default="x", choices=["a", "x", "y"], help="Register to tally")
    p.add_argument("routines", nargs="+")
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    m = headless.movie(emu, args.movie)
    sym = headless.symbols(emu, args.symbols)
    counts = {r: collections.Counter() for r in args.routines}

    def make_callback(routine):
        def callback(cpu):
            sp = cpu.sp
            ret = emu.read(0x100 + ((sp + 1) & 0xFF)) | emu.read(0x100 + ((sp + 2) & 0xFF)) << 8
            site = ret - 2  # the JSR instruction
            counts[routine][(sym.name(site), getattr(cpu, args.reg))] += 1
            return cpu

        return callback

    for r in args.routines:
        sym.break_at(r, make_callback(r))
    frames = args.frames or len(m.frames) + 60
    emu.run_frames(frames, lambda _f: m.run_pre_frame())

    for r in args.routines:
        print(f"{r}:")
        by_site = collections.defaultdict(dict)
        for (site, value), n in counts[r].items():
            by_site[site][value] = n
        for site, values in sorted(by_site.items()):
            vals = " ".join(f"{args.reg}={v}:{n}" for v, n in sorted(values.items()))
            print(f"    {site:<28} {vals}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
