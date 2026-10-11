#!/usr/bin/env python3
"""Report how the game uses each RAM and SRAM address.

Reads the RAM access log written by cdl_movie.py (<log>.ram.json) and, for
every address, lists the routines that read it, write it or use it as a
pointer, and how: directly, or as an element of an indexed array.  Indexed
instructions are grouped by their base address, giving array candidates
with their observed extent.

    ram_usage.py [--log PREFIX] [ADDR[-ADDR] ...]

With no addresses, reports every address and writes the array summary.
Addresses are hex, with or without a leading $.
"""
import argparse
import collections
import json
import os
import re
import sys

from bootstrap import BANK_SIZE, FIXED_BANK, OPCODES

HERE = os.path.dirname(os.path.abspath(__file__))
BUILD = os.path.join(HERE, "../build")
RAM_INC = os.path.join(HERE, "../src/ram.inc")

# Flag bits; these must match rust/nes/src/cdl.rs.
RAM_READ = 0x01
RAM_WRITE = 0x02
RAM_POINTER = 0x04

INDEXED = {"zpx", "zpy", "absx", "absy"}
# An indexed instruction touching more bytes than this is a bulk operation
# (e.g. clearing memory) rather than an array access.
BULK = 32
DIRECT = {"zp", "abs"}


def cpu_addr(offset):
    bank, off = divmod(offset, BANK_SIZE)
    return bank, (0xC000 if bank == FIXED_BANK else 0x8000) + off


class Symbols:
    """Names for ROM locations (routines) and RAM addresses."""

    def __init__(self, path, ram_inc):
        with open(path) as f:
            d = json.load(f)
        self.by_bank = collections.defaultdict(list)
        for s in d["labels"]:
            if not s["name"].startswith("dat_"):
                self.by_bank[s["bank"]].append((s["addr"], s["name"]))
        for v in self.by_bank.values():
            v.sort()
        self.ram = {}
        with open(ram_inc) as f:
            for line in f:
                m = re.match(r"^([A-Za-z_]\w*)\s*=\s*\$([0-9A-Fa-f]+)", line)
                if m:
                    v = int(m.group(2), 16)
                    # Prefer real names over mechanical ones.
                    if v not in self.ram or self.ram[v].startswith(("ram_", "sram_")):
                        self.ram[v] = m.group(1)

    def routine(self, offset):
        """The code label (routine or branch target) preceding the
        instruction at PRG offset."""
        bank, addr = cpu_addr(offset)
        best = None
        for a, name in self.by_bank[bank]:
            if a > addr:
                break
            best = (a, name)
        if best is None:
            return f"${addr:04X}"
        return best[1]

    def ram_name(self, addr):
        return self.ram.get(addr, f"${addr:04X}")


class Instruction:
    def __init__(self, rom, offset):
        op = rom[offset]
        self.mnemonic, self.mode = OPCODES.get(op, ("???", "imp"))
        if self.mode in ("zp", "zpx", "zpy", "indx", "indy"):
            self.operand = rom[offset + 1]
        elif self.mode in ("abs", "absx", "absy", "ind"):
            self.operand = rom[offset + 1] | rom[offset + 2] << 8
        else:
            self.operand = None


def load(prefix, rom_path):
    with open(f"{prefix}.ram.json") as f:
        rows = json.load(f)
    with open(rom_path, "rb") as f:
        rom = f.read()[16:]
    insns = {}
    by_addr = collections.defaultdict(list)  # addr -> [(src, flags)]
    by_src = collections.defaultdict(dict)  # src -> {addr: flags}
    for src, addr, flags in rows:
        if src not in insns:
            insns[src] = Instruction(rom, src)
        by_addr[addr].append((src, flags))
        by_src[src][addr] = flags
    return insns, by_addr, by_src


def flag_str(flags):
    return "".join(c if flags & b else "-" for c, b in (("R", RAM_READ), ("W", RAM_WRITE), ("P", RAM_POINTER)))


def describe(addr, insns, by_addr, by_src, syms):
    """Lines describing how addr is accessed."""
    out = []
    groups = collections.defaultdict(lambda: [0, set()])  # (how, routine) -> [flags, mnemonics]
    for src, flags in by_addr.get(addr, []):
        ins = insns[src]
        if flags & RAM_POINTER:
            how = "pointer"
        elif ins.mode in DIRECT:
            how = "direct"
        elif ins.mode in INDEXED and len(by_src[src]) > BULK:
            touched = by_src[src]
            how = f"bulk ${min(touched):04X}-${max(touched):04X}"
        elif ins.mode in INDEXED:
            touched = by_src[src]
            how = f"{ins.mode[-1]}-indexed from {syms.ram_name(ins.operand)} (+{min(touched) - ins.operand}..+{max(touched) - ins.operand})"
        elif ins.mode in ("indx", "indy"):
            how = f"via ({syms.ram_name(ins.operand)}){',y' if ins.mode == 'indy' else ',x'}"
        else:
            how = ins.mode
        g = groups[(how, syms.routine(src))]
        g[0] |= flags
        g[1].add(ins.mnemonic)
    for (how, routine), (flags, mnemonics) in sorted(groups.items(), key=lambda kv: (kv[0][0], kv[0][1])):
        out.append(f"    {flag_str(flags)}  {routine:<28} {how}  [{' '.join(sorted(mnemonics))}]")
    return out


def arrays(insns, by_src, syms):
    """Indexed bases and the union of offsets accessed through them."""
    bases = collections.defaultdict(lambda: [set(), set()])  # base -> [offsets, routines]
    for src, touched in by_src.items():
        ins = insns[src]
        if ins.mode not in INDEXED or len(touched) > BULK:
            continue
        offs = {a - ins.operand for a in touched}
        bases[ins.operand][0].update(offs)
        bases[ins.operand][1].add(syms.routine(src))
    return bases


def parse_range(s):
    lo, _, hi = s.partition("-")
    lo = int(lo.lstrip("$"), 16)
    hi = int(hi.lstrip("$"), 16) if hi else lo
    return range(lo, hi + 1)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--log", default=os.path.join(BUILD, "zelda2"), help="cdl_movie.py output prefix")
    p.add_argument("--rom", default=os.path.join(BUILD, "zelda2.nes"))
    p.add_argument("--sym", default=os.path.join(BUILD, "zelda2.sym.json"))
    p.add_argument("--arrays", action="store_true", help="Only print the array summary")
    p.add_argument("addrs", nargs="*", help="Addresses or ranges to report (hex)")
    args = p.parse_args()

    syms = Symbols(args.sym, RAM_INC)
    insns, by_addr, by_src = load(args.log, args.rom)

    if args.addrs:
        addrs = [a for r in args.addrs for a in parse_range(r)]
    else:
        addrs = list(range(0x0000, 0x0800)) + list(range(0x6000, 0x8000))

    if not args.arrays:
        for addr in addrs:
            lines = describe(addr, insns, by_addr, by_src, syms)
            flags = 0
            for _, f in by_addr.get(addr, []):
                flags |= f
            if not lines:
                if args.addrs:
                    print(f"${addr:04X} {syms.ram_name(addr)}: not accessed")
                continue
            print(f"${addr:04X} {syms.ram_name(addr)}  {flag_str(flags)}")
            print("\n".join(lines))

    if args.arrays or not args.addrs:
        wanted = set(addrs)
        print("\nIndexed bases (array candidates):")
        for base, (offs, routines) in sorted(arrays(insns, by_src, syms).items()):
            if base not in wanted and not args.arrays:
                continue
            lo, hi = min(offs), max(offs)
            print(f"  ${base:04X} {syms.ram_name(base):<20} +{lo}..+{hi} ({len(offs)} offsets)  {len(routines)} routines")
        untouched = [a for a in addrs if a not in by_addr]
        print(f"\n{len(untouched)} of {len(addrs)} addresses were never accessed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
