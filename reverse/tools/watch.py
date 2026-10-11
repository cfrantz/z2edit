#!/usr/bin/env python3
"""Log writes to RAM variables while playing a movie.

    watch.py --movie MOVIE [--frames N] [--changes] VAR [VAR ...]

VAR is a symbol from ram.inc (optionally NAME+k) or a hex address.  Each
line gives the frame, the variable, the old and new values, and the
routine that wrote it.  With --changes, writes that don't change the value
are not shown.
"""
import argparse
import os
import sys

import headless

HERE = os.path.dirname(os.path.abspath(__file__))


def resolve(sym, var):
    name, _, k = var.partition("+")
    try:
        base = sym.addr(name)
    except KeyError:
        base = int(name.lstrip("$"), 16)
    return base + int(k or 0)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True, help="FM2 movie (FILE or FILE:OFFSET)")
    p.add_argument("--frames", type=int, default=0, help="Frames to play (0 = whole movie)")
    p.add_argument("--changes", action="store_true", help="Only show writes that change the value")
    p.add_argument("vars", nargs="+")
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    m = headless.movie(emu, args.movie)
    sym = headless.symbols(emu, args.symbols)
    frame = [0]

    def make_callback(var, addr):
        def callback(cpu, address, value):
            old = emu.read(addr)
            if value != old or not args.changes:
                # The PC has already advanced past the writing instruction.
                print(f"{frame[0]:7} {var:<20} {old:02X} -> {value:02X}  {sym.name(cpu.pc)}", flush=True)
            return value

        return callback

    for var in args.vars:
        addr = resolve(sym, var)
        emu.set_write_callback(addr, make_callback(var, addr))

    def pre_frame(f):
        frame[0] = f
        m.run_pre_frame()

    emu.run_frames(args.frames or len(m.frames) + 60, pre_frame)
    return 0


if __name__ == "__main__":
    sys.exit(main())
