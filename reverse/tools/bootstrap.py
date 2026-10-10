#!/usr/bin/env python3
"""Generate the initial ca65 disassembly of Zelda II from the ROM and a CDL.

This tool is run ONCE to bootstrap `reverse/src`.  After that, the source
files are maintained by hand.  It refuses to overwrite existing bank files
unless --force is given.

Inputs:
    --rom   the reference ROM
    --cdl   prefix of the logs written by cdl_movie.py
            (<prefix>.cdl, <prefix>.jumps.json, <prefix>.refs.json)

Outputs (in --out):
    bank0.s ... bank7.s   one file per PRG bank
    ram.inc               symbols for every RAM/SRAM address referenced
    globals.inc           .global declarations for cross-bank labels
    constants.inc         empty, for game-wide constants
    interfaces.inc        empty, for bank interfaces (see CONVENTIONS.md)
    zelda2.inc            includes the above (and hardware.inc)

Classification of each PRG byte:
    - Bytes the CDL saw executed are code.
    - Code statically reachable from executed code (branch targets not taken
      in the movies, fall-through paths) is also decoded as code, provided
      it decodes cleanly and doesn't collide with known data.  Such code is
      marked "not executed" in the output.
    - Everything else is emitted as .byte, with bytes never accessed
      marked "unreferenced".
"""
import argparse
import json
import os
import sys

BANK_SIZE = 0x4000
NUM_BANKS = 8
FIXED_BANK = 7

# CDL flags; these must match rust/nes/src/cdl.rs.
OPCODE = 0x01
OPERAND = 0x02
DATA = 0x04
DATA_INDIRECT = 0x08
INDIRECT_TARGET = 0x10
SUB_ENTRY = 0x20

# Addressing mode -> instruction size.
SIZES = {
    "imp": 1, "acc": 1, "imm": 2, "zp": 2, "zpx": 2, "zpy": 2,
    "indx": 2, "indy": 2, "rel": 2, "abs": 3, "absx": 3, "absy": 3, "ind": 3,
}

# The official 6502 opcodes: opcode -> (mnemonic, mode).
OPCODES = {}
for _line in """
00 brk imp  01 ora indx 05 ora zp   06 asl zp   08 php imp  09 ora imm  0a asl acc  0d ora abs  0e asl abs
10 bpl rel  11 ora indy 15 ora zpx  16 asl zpx  18 clc imp  19 ora absy 1d ora absx 1e asl absx
20 jsr abs  21 and indx 24 bit zp   25 and zp   26 rol zp   28 plp imp  29 and imm  2a rol acc  2c bit abs
2d and abs  2e rol abs  30 bmi rel  31 and indy 35 and zpx  36 rol zpx  38 sec imp  39 and absy 3d and absx
3e rol absx 40 rti imp  41 eor indx 45 eor zp   46 lsr zp   48 pha imp  49 eor imm  4a lsr acc  4c jmp abs
4d eor abs  4e lsr abs  50 bvc rel  51 eor indy 55 eor zpx  56 lsr zpx  58 cli imp  59 eor absy 5d eor absx
5e lsr absx 60 rts imp  61 adc indx 65 adc zp   66 ror zp   68 pla imp  69 adc imm  6a ror acc  6c jmp ind
6d adc abs  6e ror abs  70 bvs rel  71 adc indy 75 adc zpx  76 ror zpx  78 sei imp  79 adc absy 7d adc absx
7e ror absx 81 sta indx 84 sty zp   85 sta zp   86 stx zp   88 dey imp  8a txa imp  8c sty abs  8d sta abs
8e stx abs  90 bcc rel  91 sta indy 94 sty zpx  95 sta zpx  96 stx zpy  98 tya imp  99 sta absy 9a txs imp
9d sta absx a0 ldy imm  a1 lda indx a2 ldx imm  a4 ldy zp   a5 lda zp   a6 ldx zp   a8 tay imp  a9 lda imm
aa tax imp  ac ldy abs  ad lda abs  ae ldx abs  b0 bcs rel  b1 lda indy b4 ldy zpx  b5 lda zpx  b6 ldx zpy
b8 clv imp  b9 lda absy ba tsx imp  bc ldy absx bd lda absx be ldx absy c0 cpy imm  c1 cmp indx c4 cpy zp
c5 cmp zp   c6 dec zp   c8 iny imp  c9 cmp imm  ca dex imp  cc cpy abs  cd cmp abs  ce dec abs  d0 bne rel
d1 cmp indy d5 cmp zpx  d6 dec zpx  d8 cld imp  d9 cmp absy dd cmp absx de dec absx e0 cpx imm  e1 sbc indx
e4 cpx zp   e5 sbc zp   e6 inc zp   e8 inx imp  e9 sbc imm  ea nop imp  ec cpx abs  ed sbc abs  ee inc abs
f0 beq rel  f1 sbc indy f5 sbc zpx  f6 inc zpx  f8 sed imp  f9 sbc absy fd sbc absx fe inc absx
""".split("\n"):
    _f = _line.split()
    for _i in range(0, len(_f), 3):
        OPCODES[int(_f[_i], 16)] = (_f[_i + 1], _f[_i + 2])
assert len(OPCODES) == 151, len(OPCODES)

# Instructions that end a flow of control.
TERMINATORS = {"jmp", "rts", "rti", "brk"}
# Instructions that write their memory operand (used to recognize mapper writes).
WRITES = {"sta", "stx", "sty", "inc", "dec", "asl", "lsr", "rol", "ror"}

HARDWARE = {
    0x2000: "PPUCTRL", 0x2001: "PPUMASK", 0x2002: "PPUSTATUS", 0x2003: "OAMADDR",
    0x2004: "OAMDATA", 0x2005: "PPUSCROLL", 0x2006: "PPUADDR", 0x2007: "PPUDATA",
    0x4000: "SQ1_VOL", 0x4001: "SQ1_SWEEP", 0x4002: "SQ1_LO", 0x4003: "SQ1_HI",
    0x4004: "SQ2_VOL", 0x4005: "SQ2_SWEEP", 0x4006: "SQ2_LO", 0x4007: "SQ2_HI",
    0x4008: "TRI_LINEAR", 0x4009: "TRI_UNUSED", 0x400A: "TRI_LO", 0x400B: "TRI_HI",
    0x400C: "NOISE_VOL", 0x400D: "NOISE_UNUSED", 0x400E: "NOISE_LO", 0x400F: "NOISE_HI",
    0x4010: "DMC_FREQ", 0x4011: "DMC_RAW", 0x4012: "DMC_START", 0x4013: "DMC_LEN",
    0x4014: "OAMDMA", 0x4015: "SND_CHN", 0x4016: "JOY1",
}
MMC1_REGS = [(0x8000, "MMC1_CONTROL"), (0xA000, "MMC1_CHR0"), (0xC000, "MMC1_CHR1"), (0xE000, "MMC1_PRG")]


def base_addr(bank):
    return 0xC000 if bank == FIXED_BANK else 0x8000


class Rom:
    def __init__(self, rom, cdl_prefix):
        with open(rom, "rb") as f:
            data = f.read()
        self.prg = data[16 : 16 + NUM_BANKS * BANK_SIZE]
        if cdl_prefix:
            with open(f"{cdl_prefix}.cdl", "rb") as f:
                self.cdl = bytearray(f.read())
            with open(f"{cdl_prefix}.jumps.json") as f:
                self.jumps = json.load(f)
            with open(f"{cdl_prefix}.refs.json") as f:
                self.refs = {int(k): v for k, v in json.load(f).items()}
        else:
            self.cdl = bytearray(len(self.prg))
            self.jumps = []
            self.refs = {}
        # Per-byte classification: None, ("op", size), ("cont", start).
        self.insn = [None] * len(self.prg)
        self.static = [False] * len(self.prg)
        self.no_return = set()
        # Inline jump tables following a JSR to a dispatcher:
        # table start offset -> [target offset or None, ...]
        self.tables = {}
        self.table_entry = {}  # entry offset -> target offset or None
        self.unconfirmed = set()  # table entries neither read nor executed
        self.dispatchers = set()  # CPU addresses of jump-table dispatchers

    def to_cpu(self, offset):
        bank = offset // BANK_SIZE
        return bank, base_addr(bank) + offset % BANK_SIZE

    def to_offset(self, bank, cpu):
        base = base_addr(bank)
        if not base <= cpu < base + BANK_SIZE:
            return None
        return bank * BANK_SIZE + cpu - base

    def decode(self, offset):
        """Decode the instruction at a PRG offset.  Returns
        (mnemonic, mode, size, operand) or None if not an official opcode
        or if it crosses a bank boundary."""
        op = self.prg[offset]
        if op not in OPCODES:
            return None
        mnem, mode = OPCODES[op]
        size = SIZES[mode]
        if (offset % BANK_SIZE) + size > BANK_SIZE:
            return None
        if size == 2:
            operand = self.prg[offset + 1]
        elif size == 3:
            operand = self.prg[offset + 1] | self.prg[offset + 2] << 8
        else:
            operand = None
        return mnem, mode, size, operand

    def target_bank(self, offset, cpu):
        """Determine which bank a CPU address referenced by the instruction
        at `offset` refers to.  Returns None if unknown/ambiguous."""
        bank = offset // BANK_SIZE
        if cpu >= 0xC000:
            return FIXED_BANK
        if cpu < 0x8000:
            return None
        if bank != FIXED_BANK:
            return bank
        banks = self.refs.get(offset, [])
        banks = [b for b in banks if b != FIXED_BANK]
        return banks[0] if len(banks) == 1 else None

    def flow_targets(self, offset, d):
        """Return the control-flow successors of the instruction at offset
        that lie within ROM, as PRG offsets."""
        mnem, mode, size, operand = d
        bank, pc = self.to_cpu(offset)
        out = []
        if mode == "rel":
            target = (pc + 2 + (operand - 256 if operand >= 128 else operand)) & 0xFFFF
            t = self.to_offset(bank, target)
            if t is not None:
                out.append(t)
        elif mnem in ("jsr", "jmp") and mode == "abs":
            tb = self.target_bank(offset, operand)
            if tb is not None:
                t = self.to_offset(tb, operand)
                if t is not None:
                    out.append(t)
        if mnem not in TERMINATORS and offset not in self.no_return:
            out.append(offset + size)
        return out

    def classify(self):
        # 1. Executed code from the CDL.
        executed = [i for i, f in enumerate(self.cdl) if f & OPCODE]
        for i in executed:
            d = self.decode(i)
            if d is None:
                print(f"warning: executed byte at PRG ${i:05X} is not an official opcode", file=sys.stderr)
                continue
            self.insn[i] = ("op", d[2])
        for i in executed:
            if self.insn[i] is None:
                continue
            for k in range(1, self.insn[i][1]):
                if self.insn[i + k] is None:
                    self.insn[i + k] = ("cont", i)
            # A JSR whose following byte never executed may not return
            # (e.g. it calls a dispatcher that consumes inline data).
            d = self.decode(i)
            if d[0] == "jsr" and i + 3 < len(self.cdl) and not self.cdl[i + 3] & OPCODE:
                self.no_return.add(i)

        # 2. Inline jump tables.  A dispatcher is a JSR target that pulls the
        #    return address and indexes a word table following the JSR.  It
        #    is recognized by an executed call site followed by bytes that
        #    were read indirectly.  Every JSR to a dispatcher starts a table.
        jsr_sites = {}
        for i in executed:
            d = self.decode(i)
            if d and d[0] == "jsr":
                jsr_sites.setdefault(d[3], []).append(i)
        for target, sites in jsr_sites.items():
            for i in sites:
                if i not in self.no_return:
                    continue
                window = range(i + 3, min(i + 3 + 64, (i // BANK_SIZE + 1) * BANK_SIZE))
                for k in window:
                    if self.cdl[k] & OPCODE:
                        break
                    if self.cdl[k] & DATA_INDIRECT:
                        self.dispatchers.add(target)
                        break
        work = []
        for target in self.dispatchers:
            for i in jsr_sites[target]:
                self.no_return.add(i)
                work.extend(t for t in self.find_table(i + 3) if t is not None)

        # 3. Static recursive descent from executed code into untouched bytes.
        for i in executed:
            if self.insn[i] and self.insn[i][0] == "op":
                work.extend(self.flow_targets(i, self.decode(i)))
        while work:
            start = work.pop()
            path = self.trace(start)
            for off, size in path:
                self.insn[off] = ("op", size)
                self.static[off] = True
                for k in range(1, size):
                    self.insn[off + k] = ("cont", off)
            for off, size in path:
                d = self.decode(off)
                if d[0] == "jsr" and d[3] in self.dispatchers:
                    self.no_return.add(off)
                    work.extend(t for t in self.find_table(off + 3) if t is not None)
                work.extend(self.flow_targets(off, d))

    def find_table(self, start):
        """Find the extent of an inline word table at `start`.

        An entry is accepted if it was read indirectly in the CDL, if its target
        was executed, or if its target plausibly is code (an official opcode
        not known to be data).  $0000 entries are accepted as null entries
        when followed by an accepted entry.  Returns the target offsets.
        """
        bank = start // BANK_SIZE
        end = (bank + 1) * BANK_SIZE
        entries = []  # (offset, target, observed)
        k = start
        while k + 1 < end:
            if self.insn[k] is not None or self.insn[k + 1] is not None or k in self.table_entry:
                break
            if self.cdl[k] & OPCODE or self.cdl[k + 1] & OPCODE:
                break
            value = self.prg[k] | self.prg[k + 1] << 8
            tb = FIXED_BANK if value >= 0xC000 else bank if value >= 0x8000 else None
            target = self.to_offset(tb, value) if tb is not None else None
            # The dispatcher reads its table indirectly; direct reads mean the
            # bytes belong to some other structure.
            observed = bool(self.cdl[k] & DATA_INDIRECT and self.cdl[k + 1] & DATA_INDIRECT)
            if not observed and (self.cdl[k] | self.cdl[k + 1]) & (DATA | DATA_INDIRECT):
                break
            executed = target is not None and bool(self.cdl[target] & OPCODE)
            plausible = (
                target is not None
                and self.prg[target] in OPCODES
                and not self.cdl[target] & (DATA | DATA_INDIRECT | OPERAND)
                and not (self.insn[target] and self.insn[target][0] == "cont")
            )
            if not (observed or executed or plausible or value == 0):
                break
            entries.append((k, value, target, observed or executed))
            k += 2
        # Drop trailing null entries.
        while entries and entries[-1][1] == 0:
            entries.pop()
        for k, _, target, confirmed in entries:
            self.table_entry[k] = target
            if not confirmed:
                self.unconfirmed.add(k)
        self.tables[start] = [t for _, _, t, _ in entries]
        return self.tables[start]

    def trace(self, start):
        """Linearly decode untouched bytes starting at `start`, until a
        terminator or existing code.  Returns the instructions decoded, or
        an empty list if the path hits something that isn't plausibly code."""
        path = []
        off = start
        while True:
            if not 0 <= off < len(self.prg):
                return []
            if self.insn[off] is not None:
                # Joined existing code: fine if it's an instruction start.
                return path if self.insn[off][0] == "op" else []
            if (off // BANK_SIZE) != (start // BANK_SIZE):
                return []
            d = self.decode(off)
            if d is None or d[0] == "brk":
                return []
            size = d[2]
            for k in range(size):
                if (
                    self.cdl[off + k] & (DATA | DATA_INDIRECT)
                    or self.insn[off + k] is not None
                    or off + k in self.table_entry
                    or off + k - 1 in self.table_entry
                ):
                    return []
            path.append((off, size))
            if d[0] in TERMINATORS or (d[0] == "jsr" and d[3] in self.dispatchers):
                return path
            off += size


class Disassembler:
    def __init__(self, rom):
        self.rom = rom
        self.labels = {}  # PRG offset -> name
        self.ram = {}  # cpu address -> name
        self.uses = {}  # name -> set of banks that reference it

    def label_for(self, target_off, kind):
        if target_off in self.labels:
            name = self.labels[target_off]
            # Upgrade loc_/dat_ to sub_ for JSR targets.
            if kind == "sub" and not name.startswith("sub_"):
                name = "sub_" + name.split("_", 1)[1]
                self.labels[target_off] = name
            return name
        bank, cpu = self.rom.to_cpu(target_off)
        if bank == FIXED_BANK:
            name = f"{kind}_{cpu:04X}"
        else:
            name = f"{kind}_{bank}_{cpu:04X}"
        self.labels[target_off] = name
        return name

    def collect_labels(self):
        rom = self.rom
        for off, f in enumerate(rom.cdl):
            if f & SUB_ENTRY:
                self.label_for(off, "sub")
            elif f & INDIRECT_TARGET:
                self.label_for(off, "loc")
        for off, kind in enumerate(rom.insn):
            if kind is None or kind[0] != "op":
                continue
            d = rom.decode(off)
            mnem, mode, size, operand = d
            bank, pc = rom.to_cpu(off)
            if mode == "rel":
                target = (pc + 2 + (operand - 256 if operand >= 128 else operand)) & 0xFFFF
                t = rom.to_offset(bank, target)
                if t is not None:
                    self.label_for(t, "loc")
            elif mode in ("abs", "absx", "absy", "ind") and operand >= 0x8000:
                if mnem in WRITES:
                    continue  # mapper register write
                tb = rom.target_bank(off, operand)
                if tb is None:
                    continue
                t = rom.to_offset(tb, operand)
                if t is None:
                    continue
                if mnem == "jsr":
                    kind = "sub"
                elif mnem == "jmp" and mode == "abs":
                    kind = "loc"
                else:
                    kind = "dat"
                self.label_for(t, kind)
        # Jump table targets.
        for t in rom.table_entry.values():
            if t is not None:
                self.label_for(t, "loc")
        # Interrupt vectors.
        for v in (0xFFFA, 0xFFFC, 0xFFFE):
            o = rom.to_offset(FIXED_BANK, v)
            target = rom.prg[o] | rom.prg[o + 1] << 8
            t = rom.to_offset(FIXED_BANK, target)
            if t is not None:
                self.label_for(t, "loc")

    def ram_name(self, addr):
        if addr < 0x0800:
            name = f"ram_{addr:04X}"
        elif 0x6000 <= addr < 0x8000:
            name = f"sram_{addr:04X}"
        else:
            return None
        self.ram[addr] = name
        return name

    def operand_text(self, off, d):
        """Format the operand of the instruction at `off`."""
        rom = self.rom
        mnem, mode, size, operand = d
        bank, pc = rom.to_cpu(off)
        if mode in ("imp",):
            return ""
        if mode == "acc":
            return "a"
        if mode == "imm":
            return f"#${operand:02X}"
        if mode == "rel":
            target = (pc + 2 + (operand - 256 if operand >= 128 else operand)) & 0xFFFF
            t = rom.to_offset(bank, target)
            return self.ref(t, bank) if t is not None else f"${target:04X}"

        if mode in ("zp", "zpx", "zpy", "indx", "indy"):
            sym = self.ram_name(operand)
        else:
            sym = self.addr_symbol(off, d)
        if sym is None:
            sym = f"${operand:04X}" if size == 3 else f"${operand:02X}"
        if size == 3 and operand < 0x100 and mode != "ind":
            sym = "a:" + sym
        return {
            "zp": "{}", "abs": "{}", "zpx": "{},x", "absx": "{},x", "zpy": "{},y",
            "absy": "{},y", "indx": "({},x)", "indy": "({}),y", "ind": "({})",
        }[mode].format(sym)

    def addr_symbol(self, off, d):
        rom = self.rom
        mnem, mode, size, operand = d
        bank = off // BANK_SIZE
        if operand in HARDWARE:
            return HARDWARE[operand]
        if operand < 0x0800 or 0x6000 <= operand < 0x8000:
            return self.ram_name(operand)
        if operand >= 0x8000:
            if mnem in WRITES:
                base, reg = [r for r in MMC1_REGS if r[0] <= operand][-1]
                return reg if operand == base else f"{reg}+${operand - base:04X}"
            tb = rom.target_bank(off, operand)
            if tb is None:
                return None
            t = rom.to_offset(tb, operand)
            return self.ref(t, bank) if t is not None else None
        return None

    def ref(self, target_off, from_bank):
        name = self.labels.get(target_off)
        if name is None:
            return None
        self.uses.setdefault(name, set()).add(from_bank)
        return name

    def emit_bank(self, bank):
        rom = self.rom
        lines = []
        base = base_addr(bank)
        lines.append(f"; Zelda II: The Adventure of Link - PRG bank {bank} (${base:04X}-${base + BANK_SIZE - 1:04X})")
        lines.append(";")
        lines.append("; Generated by tools/bootstrap.py, now maintained by hand.")
        lines.append("; See CONVENTIONS.md.")
        lines.append("")
        lines.append('.include "zelda2.inc"')
        lines.append("")
        lines.append(f'.segment "BANK{bank}"')
        lines.append("")

        start = bank * BANK_SIZE
        end = start + BANK_SIZE
        off = start
        prev_region = None
        while off < end:
            kind = rom.insn[off]
            cpu = rom.to_cpu(off)[1]
            if bank == FIXED_BANK and cpu == 0xFFFA:
                lines.append("")
                lines.append("; Interrupt vectors.")
                for name in ("NMI", "RESET", "IRQ"):
                    o = rom.to_offset(FIXED_BANK, cpu)
                    target = rom.prg[o] | rom.prg[o + 1] << 8
                    sym = self.ref(rom.to_offset(FIXED_BANK, target), bank) or f"${target:04X}"
                    lines.append(f"{'    .word   ' + sym:<36}; ${cpu:04X}  {name}")
                    cpu += 2
                break
            if off in rom.table_entry:
                if off in rom.tables:
                    lines.append("; (inline jump table)")
                prev_region = "table"
                self.emit_labels(lines, off)
                t = rom.table_entry[off]
                sym = self.ref(t, bank) if t is not None else None
                if sym is None:
                    sym = f"${rom.prg[off] | rom.prg[off + 1] << 8:04X}"
                note = "  (not observed)" if off in rom.unconfirmed else ""
                if off + 1 in self.labels:
                    lines.append(f"{self.labels[off + 1]} := * + 1")
                lines.append(f"{'    .word   ' + sym:<36}; ${cpu:04X}{note}")
                off += 2
                continue
            if kind is not None and kind[0] == "op":
                d = rom.decode(off)
                size = d[2]
                # Overlapping instruction (e.g. BIT-skip trick): another
                # instruction starts inside this one.
                inner = [k for k in range(1, size) if rom.insn[off + k] and rom.insn[off + k][0] == "op"]
                region = "static" if rom.static[off] else "code"
                if region != prev_region:
                    if region == "static":
                        lines.append("; (code below not executed in the test movies)")
                    elif prev_region == "static":
                        lines.append("; (end of code not executed in the test movies)")
                    prev_region = region
                self.emit_labels(lines, off)
                if inner:
                    n = inner[0]
                    raw = ", ".join(f"${b:02X}" for b in rom.prg[off : off + n])
                    lines.append(f"    .byte   {raw:<24}; ${cpu:04X}  {d[0]} (overlapping instruction)")
                    off += n
                    continue
                for k in range(1, size):
                    if off + k in self.labels:
                        lines.append(f"{self.labels[off + k]} := * + {k}")
                text = f"    {d[0]:<8}{self.operand_text(off, d)}"
                lines.append(f"{text:<36}; ${cpu:04X}")
                off += size
            else:
                # Data run: up to 16 bytes, split at labels, code and at
                # changes between referenced/unreferenced bytes.
                touched = bool(rom.cdl[off])
                region = "data" if touched else "unref"
                if region != prev_region:
                    if prev_region == "static":
                        lines.append("; (end of code not executed in the test movies)")
                    if region == "unref":
                        n = off
                        while n < end and not rom.cdl[n] and not (rom.insn[n] and rom.insn[n][0] == "op") and n not in rom.table_entry:
                            n += 1
                        lines.append(f"; (unreferenced: ${cpu:04X}-${rom.to_cpu(n - 1)[1]:04X}, {n - off} bytes)")
                    prev_region = region
                self.emit_labels(lines, off)
                n = off + 1
                while (
                    n < end
                    and n - off < 16
                    and n not in self.labels
                    and not (rom.insn[n] and rom.insn[n][0] == "op")
                    and n not in rom.table_entry
                    and bool(rom.cdl[n]) == touched
                ):
                    n += 1
                raw = ", ".join(f"${b:02X}" for b in rom.prg[off:n])
                lines.append(f"    .byte   {raw:<64}; ${cpu:04X}")
                off = n
        return "\n".join(lines) + "\n"

    def emit_labels(self, lines, off):
        if off in self.labels:
            lines.append("")
            lines.append(f"{self.labels[off]}:")

    def emit_ram(self):
        lines = [
            "; RAM, zero page and SRAM variables.",
            ";",
            "; Generated by tools/bootstrap.py, now maintained by hand.",
            "",
            ".ifndef RAM_INC",
            "RAM_INC = 1",
            "",
        ]
        for addr in sorted(self.ram):
            lines.append(f"{self.ram[addr]:<24}= ${addr:04X}")
        lines += ["", ".endif"]
        return "\n".join(lines) + "\n"

    def emit_globals(self):
        lines = [
            "; Labels referenced across banks.",
            ";",
            "; Generated by tools/bootstrap.py, now maintained by hand.",
            "",
            ".ifndef GLOBALS_INC",
            "GLOBALS_INC = 1",
            "",
        ]
        by_name = {name: off for off, name in self.labels.items()}
        names = []
        for name, banks in self.uses.items():
            home = by_name[name] // BANK_SIZE
            if banks - {home}:
                names.append((by_name[name], name))
        for _, name in sorted(names):
            lines.append(f".global {name}")
        lines += ["", ".endif"]
        return "\n".join(lines) + "\n"


ZELDA2_INC = """; Common definitions included by every bank.

.ifndef ZELDA2_INC
ZELDA2_INC = 1

.include "hardware.inc"
.include "ram.inc"
.include "constants.inc"
.include "interfaces.inc"
.include "globals.inc"

.endif
"""

CONSTANTS_INC = """; Game-wide constants and enumerations.

.ifndef CONSTANTS_INC
CONSTANTS_INC = 1

.endif
"""


INTERFACES_INC = """; Bank interfaces: routines and data that several switchable banks provide
; at a common address.  See "Bank Interfaces" in CONVENTIONS.md.

.ifndef INTERFACES_INC
INTERFACES_INC = 1

.endif
"""


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", required=True)
    p.add_argument("--cdl", help="CDL log prefix (omit to treat everything as data)")
    p.add_argument("--out", required=True)
    p.add_argument("--force", action="store_true", help="Overwrite existing bank files")
    args = p.parse_args()

    existing = [f"bank{b}.s" for b in range(NUM_BANKS) if os.path.exists(os.path.join(args.out, f"bank{b}.s"))]
    if existing and not args.force:
        print(f"Refusing to overwrite hand-maintained files: {existing}", file=sys.stderr)
        return 1

    rom = Rom(args.rom, args.cdl)
    rom.classify()
    dis = Disassembler(rom)
    dis.collect_labels()

    banks = {b: dis.emit_bank(b) for b in range(NUM_BANKS)}
    for b, text in banks.items():
        with open(os.path.join(args.out, f"bank{b}.s"), "w") as f:
            f.write(text)
    with open(os.path.join(args.out, "ram.inc"), "w") as f:
        f.write(dis.emit_ram())
    with open(os.path.join(args.out, "globals.inc"), "w") as f:
        f.write(dis.emit_globals())
    with open(os.path.join(args.out, "zelda2.inc"), "w") as f:
        f.write(ZELDA2_INC)
    for name, text in (("constants.inc", CONSTANTS_INC), ("interfaces.inc", INTERFACES_INC)):
        if not os.path.exists(os.path.join(args.out, name)) or args.force:
            with open(os.path.join(args.out, name), "w") as f:
                f.write(text)

    code = sum(1 for k in rom.insn if k)
    static = sum(1 for i, k in enumerate(rom.insn) if k and rom.static[k[1] if k[0] == "cont" else i])
    entries = sum(len(t) for t in rom.tables.values())
    print(
        f"code bytes: {code} ({static} found statically), jump tables: {len(rom.tables)} ({entries} entries), "
        f"labels: {len(dis.labels)}, ram symbols: {len(dis.ram)}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
