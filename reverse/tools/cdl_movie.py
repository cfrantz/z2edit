#!/usr/bin/env python3
"""Play FM2 movies headlessly and collect a Code/Data Log (CDL).

Each movie is played from power-on in a fresh emulator.  The per-movie logs
are merged (bitwise OR of the flag bytes, union of the indirect jumps) and
written to:

    <out>.cdl          One flags byte per PRG ROM byte (see rust/nes/src/cdl.rs).
    <out>.jumps.json   Indirect control transfers observed.
    <out>.refs.json    For each instruction (by PRG offset), the PRG banks it
                       read from or transferred control to.
    <out>.ram.json     RAM and SRAM accesses: [instruction PRG offset, CPU
                       address, flags] rows (RAM_* flags in cdl.rs).
    <out>.progress.tsv Game state sampled every 30s of game time, to check
                       that each movie stayed in sync with the emulator.

A movie may be given as FILE:N to set its input frame offset; the
no-glitch movie (recorded with FCEUX 2.1.5) needs FILE:0.

With --merge, the positional arguments are instead prefixes of previously
written logs, which are merged into <out>.
"""
import argparse
import json
import logging
import sys

import headless

# Flag bits; these must match rust/nes/src/cdl.rs.
OPCODE = 0x01
OPERAND = 0x02
DATA = 0x04
DATA_INDIRECT = 0x08
INDIRECT_TARGET = 0x10
SUB_ENTRY = 0x20
WINDOW_8000 = 0x40
WINDOW_C000 = 0x80

JUMP_KINDS = {0: "indirect", 1: "rts_dispatch", 2: "vector"}
# Source offset reported when the source of a jump isn't in PRG ROM.
NO_SRC = 2**64 - 1


# RAM locations sampled for the progress log (from the zelda2 emulator plugin).
PROGRESS = {"game_state": 0x736, "world": 0x707, "region": 0x706, "area": 0x561}


def play(rom, movie_file, max_frames, progress):
    emu = headless.emulator(rom)
    movie = headless.movie(emu, movie_file)
    frames = min(len(movie.frames) + 60, max_frames) if max_frames else len(movie.frames) + 60

    def pre_frame(frame):
        movie.run_pre_frame()
        if frame % 1800 == 0:
            progress.write("\t".join([movie_file, str(frame)] + [f"{emu.read(a):02x}" for a in PROGRESS.values()]) + "\n")
            progress.flush()

    emu.cdl_start()
    done = emu.run_frames(frames, pre_frame)
    if done != frames:
        logging.error("%s: CPU halted after %d of %d frames", movie_file, done, frames)
    cdl = emu.cdl_stop()
    return cdl.prg, cdl.jumps, cdl.refs, cdl.ram


def summarize(prg):
    print("bank   opcode  operand    data  untouched")
    for bank in range(len(prg) // 16384):
        b = prg[bank * 16384 : (bank + 1) * 16384]
        op = sum(1 for x in b if x & OPCODE)
        opnd = sum(1 for x in b if x & OPERAND)
        data = sum(1 for x in b if x & (DATA | DATA_INDIRECT) and not x & (OPCODE | OPERAND))
        none = sum(1 for x in b if not x & (OPCODE | OPERAND | DATA | DATA_INDIRECT))
        print(f"{bank:4} {op:8} {opnd:8} {data:7} {none:10}")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", help="NES ROM file (required unless --merge)")
    p.add_argument("--out", required=True, help="Output path prefix")
    p.add_argument("--max-frames", type=int, default=0, help="Limit frames per movie (0 = whole movie)")
    p.add_argument("--merge", action="store_true", help="Merge previously written logs")
    p.add_argument("movies", nargs="+", help="FM2 movie files (or log prefixes with --merge)")
    args = p.parse_args()
    logging.basicConfig(level=logging.WARNING)

    merged = None
    jumps = set()
    refs = set()
    ram = {}
    progress = None if args.merge else open(f"{args.out}.progress.tsv", "w")
    if progress:
        progress.write("\t".join(["movie", "frame"] + list(PROGRESS)) + "\n")
    for m in args.movies:
        if args.merge:
            with open(f"{m}.cdl", "rb") as f:
                prg = f.read()
            with open(f"{m}.jumps.json") as f:
                kinds = {v: k for k, v in JUMP_KINDS.items()}
                j = [(kinds[r["kind"]], r["src"], r["target_cpu"], r["target"]) for r in json.load(f)]
            with open(f"{m}.refs.json") as f:
                r = [(int(src), bank) for src, banks in json.load(f).items() for bank in banks]
            with open(f"{m}.ram.json") as f:
                a = json.load(f)
        else:
            prg, j, r, a = play(args.rom, m, args.max_frames, progress)
        refs.update(r)
        for src, addr, flags in a:
            ram[(src, addr)] = ram.get((src, addr), 0) | flags
        merged = bytearray(prg) if merged is None else bytearray(a | b for a, b in zip(merged, prg))
        jumps.update((int(k), NO_SRC if s is None else s, t, o) for k, s, t, o in j)

    with open(f"{args.out}.cdl", "wb") as f:
        f.write(merged)
    with open(f"{args.out}.jumps.json", "w") as f:
        rows = [
            {"kind": JUMP_KINDS[k], "src": None if s == NO_SRC else s, "target_cpu": t, "target": o}
            for k, s, t, o in sorted(jumps, key=lambda r: (r[0], r[1], r[2]))
        ]
        json.dump(rows, f, indent=1)
    with open(f"{args.out}.refs.json", "w") as f:
        by_src = {}
        for src, bank in sorted(refs):
            by_src.setdefault(src, []).append(bank)
        json.dump(by_src, f)
    with open(f"{args.out}.ram.json", "w") as f:
        json.dump([[s, a, fl] for (s, a), fl in sorted(ram.items())], f)
    summarize(merged)
    print(f"{len(jumps)} indirect control transfers")
    return 0


if __name__ == "__main__":
    sys.exit(main())
