#!/usr/bin/env python3
"""Tabulate which PRG bank is mapped at $8000 in each game state and world.

Plays a movie headlessly.  For every instruction executed in the
switchable window, records (game state $0736, world $0707, bank) and
reports cycle-weighted counts.  Used to verify which banks hold what.

Cycles spent in the main thread's idle loop (Main_Loop, $C010-$C031) are
reported separately, as "idle", and excluded from the bank percentages.
"""
import argparse
import collections
import sys

import headless

# Main_Loop: from Main_Loop up to (not including) the jsr to the area loader,
# plus the jmp back.  Offsets within the 16K bank.
IDLE = set(range(0x0010, 0x0027)) | {0x002F}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--rom", required=True)
    p.add_argument("--every", type=int, default=15, help="Sample every N frames")
    p.add_argument("movie")
    args = p.parse_args()

    emu = headless.emulator(args.rom)
    m = headless.movie(emu, args.movie)
    counts = collections.Counter()
    idle = collections.Counter()
    total = len(m.frames) + 60
    emu.trace = True
    for _ in range(0, total, args.every):
        emu.tracebuf = {}
        emu.run_frames(args.every, lambda _f: m.run_pre_frame())
        state, world = emu.read(0x736), emu.read(0x707)
        for addr, cycles in emu.tracebuf.items():
            bank = addr.bank()
            if bank is None:
                continue
            bank %= emu.rom_prg_banks
            if bank == 7 and addr.offset() & 0x3FFF in IDLE:
                idle[(state, world)] += cycles
            else:
                counts[(state, world, bank)] += cycles
    by_key = collections.defaultdict(dict)
    for (state, world, bank), c in counts.items():
        by_key[(state, world)][bank] = c
    print("state world  idle%  cycles-by-bank (% of non-idle)")
    for (state, world), banks in sorted(by_key.items()):
        t = sum(banks.values())
        i = idle[(state, world)]
        parts = "  ".join(f"{b}:{100 * c / t:5.1f}" for b, c in sorted(banks.items()) if c / t >= 0.001)
        print(f"  ${state:02X}   {world}   {100 * i / (i + t):5.1f}  {parts}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
