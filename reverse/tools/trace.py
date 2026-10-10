#!/usr/bin/env python3
"""Write an instruction trace for a range of frames of an FM2 movie.

    trace.py --rom ROM --movie MOVIE --start 5000 --frames 2 --out trace.txt

Each line holds: frame, CPU cycle, PRG bank (- for RAM), address,
instruction bytes, disassembly, CPU state.  With --symbols, the location
of each instruction is annotated with its label (label+offset).
"""
import argparse
import re
import sys

import headless

LINE = re.compile(r"^(\s*\d+\s+\d+\s+)(\s?[\d?-]+):([0-9a-f]{4}): (.*)$")


def symbolize(path, sym):
    with open(path) as f:
        lines = f.readlines()
    with open(path, "w") as f:
        for line in lines:
            m = LINE.match(line)
            if m:
                bank = m.group(2).strip()
                addr = int(m.group(3), 16)
                where = sym.name(addr, bank=int(bank)) if bank.isdigit() else f"${addr:04X}"
                line = f"{m.group(1)}{bank:>2}:{m.group(3)} {where:<24} {m.group(4)}\n"
            f.write(line)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", required=True)
    p.add_argument("--movie", help="FM2 movie to play (default: no input)")
    p.add_argument("--start", type=int, default=0, help="First frame to trace")
    p.add_argument("--frames", type=int, default=1, help="Number of frames to trace")
    p.add_argument("--symbols", help="Symbol table from tools/symbols.py")
    p.add_argument("--out", required=True)
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    pre_frame = None
    if args.movie:
        m = headless.movie(emu, args.movie)
        pre_frame = lambda _frame: m.run_pre_frame()  # noqa: E731
    if emu.run_frames(args.start, pre_frame) != args.start:
        print("CPU halted before the start frame", file=sys.stderr)
        return 1
    emu.trace_start(args.out)
    emu.run_frames(args.frames, pre_frame)
    emu.trace_stop()
    if args.symbols:
        symbolize(args.out, headless.symbols(emu, args.symbols))
    return 0


if __name__ == "__main__":
    sys.exit(main())
