#!/usr/bin/env python3
"""Measure how deep the 6502 stack goes while playing a movie.

    stack_depth.py MOVIE FRAMES [--below ADDR]

Puts write callbacks on $0100 up to ADDR (exclusive, hex; default $0200)
and plays MOVIE (FILE or FILE:OFFSET) for FRAMES frames.  A write landing
at the stack pointer (or one byte either side of it) is a push; any other
write is an ordinary variable store, reported with its routine.  Prints
the deepest push seen, with the routine and frame, every 20000 frames and
at the end.

The access log (cdl_movie.py) doesn't record pushes, hence this.  Watching
only below the depth already known (e.g. --below 1EA) runs much faster,
since every call pushes near the top of the page.
"""
import argparse
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import headless  # noqa: E402


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("movie")
    p.add_argument("frames", type=int)
    p.add_argument("--below", default="200")
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    a = p.parse_args()

    emu = headless.emulator(a.rom)
    mv = headless.movie(emu, a.movie)
    sym = headless.symbols(emu, a.symbols)
    st = {"min": 0x200, "where": None, "frame": 0, "other": {}}
    frame = [0]

    def on_write(cpu, addr, value):
        sp = 0x100 + cpu.sp
        if addr in (sp - 1, sp, sp + 1):
            if addr < st["min"]:
                st["min"], st["where"], st["frame"] = addr, sym.name(cpu.pc), frame[0]
        else:
            st["other"].setdefault(addr, sym.name(cpu.pc))
        return value

    for addr in range(0x100, int(a.below, 16)):
        emu.set_write_callback(addr, on_write)

    def report():
        deepest = f"${st['min']:04X} in {st['where']} on frame {st['frame']}" if st["where"] else "none seen"
        print(f"{frame[0]}: deepest push {deepest}", flush=True)

    def pre_frame(f):
        frame[0] = f
        mv.run_pre_frame()
        if f and f % 20000 == 0:
            report()

    emu.run_frames(a.frames, pre_frame)
    report()
    print("other writes: " + " ".join(f"${x:04X} {r}" for x, r in sorted(st["other"].items())))
    return 0


if __name__ == "__main__":
    sys.exit(main())
