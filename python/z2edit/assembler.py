######################################################################
# A crummy 6502 assembler.
#
# This assembler is a python port of the assembler from Z2Edit 1.0.
# Features:
#   Standard 6502 opcodes
#   Labels and fixups
#   Data entry with .db / .dw / .dd pseudo ops
#   Some defacto 6502 syntax: $ for hex, ! to force 16-bit operands.
# 
# Not features:
#   No expressions: can't use "foo+1" as an operand.
#   Some defacto 6502 syntax not supported: < and > for lo/hi byte of a word.
#
# Bugs:
#   yes.
#
######################################################################
import re

try:
    import z2edit
    _address_adaptor = z2edit.Address.prg
except ImportError:
    _address_adaptor = lambda bank, addr: (bank, addr)

class OpcodeError(Exception):
    pass

class OperandError(Exception):
    pass

class FixupError(Exception):
    pass

class AddressError(Exception):
    pass

def nes_address_adaptor(bank, addr):
    if addr >= 0xC000 and addr < 0x10000:
        bank = -1
    elif addr >= 0x8000 and addr < 0xC000:
        pass
    else:
        raise AddressError("Invalid NES address", bank, addr)
    return _address_adaptor(bank, addr)

class Asm:
    ABSOLUTE = 0
    ABSOLUTEX = 1
    ABSOLUTEY = 2
    ACCUMULATOR = 3
    IMMEDIATE = 4
    IMPLIED = 5
    INDIRECTX = 6
    INDIRECT = 7
    INDIRECTY = 8
    RELATIVE = 9
    ZEROPAGE = 10
    ZEROPAGEX = 11
    ZEROPAGEY = 12
    FAKE = 13

    mnemonic = {
        #         Abs   AbX   AbY   Acc   Imm   Imp   InX   Ind   InY   Rel    Zp   ZpX   Zpy  Fake
        ".DB": [   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".DW": [   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".DD": [   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        "=":   [   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".ORG": [  -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".ASSERT_ORG": [  -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".BANK":[  -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],
        ".REPEATB":[  -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xFF, ],

        "ADC": [ 0x6d, 0x7d, 0x79,   -1, 0x69,   -1, 0x61,   -1, 0x71,   -1, 0x65, 0x75,   -1,   -1, ],
        "AND": [ 0x2d, 0x3d, 0x39,   -1, 0x29,   -1, 0x21,   -1, 0x31,   -1, 0x25, 0x35,   -1,   -1, ],
        "ASL": [ 0x0e, 0x1e,   -1, 0x0a,   -1,   -1,   -1,   -1,   -1,   -1, 0x06, 0x16,   -1,   -1, ],
        "BCC": [   -1,   -1,   -1,   -1, 0x90,   -1,   -1,   -1,   -1, 0x90,   -1,   -1,   -1,   -1, ],
        "BCS": [   -1,   -1,   -1,   -1, 0xb0,   -1,   -1,   -1,   -1, 0xb0,   -1,   -1,   -1,   -1, ],
        "BEQ": [   -1,   -1,   -1,   -1, 0xf0,   -1,   -1,   -1,   -1, 0xf0,   -1,   -1,   -1,   -1, ],
        "BIT": [ 0x2c,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0x24,   -1,   -1,   -1, ],
        "BMI": [   -1,   -1,   -1,   -1, 0x30,   -1,   -1,   -1,   -1, 0x30,   -1,   -1,   -1,   -1, ],
        "BNE": [   -1,   -1,   -1,   -1, 0xd0,   -1,   -1,   -1,   -1, 0xd0,   -1,   -1,   -1,   -1, ],
        "BPL": [   -1,   -1,   -1,   -1, 0x10,   -1,   -1,   -1,   -1, 0x10,   -1,   -1,   -1,   -1, ],
        "BRK": [   -1,   -1,   -1,   -1,   -1, 0x00,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "BVC": [   -1,   -1,   -1,   -1, 0x50,   -1,   -1,   -1,   -1, 0x50,   -1,   -1,   -1,   -1, ],
        "BVS": [   -1,   -1,   -1,   -1, 0x70,   -1,   -1,   -1,   -1, 0x70,   -1,   -1,   -1,   -1, ],
        "CLC": [   -1,   -1,   -1,   -1,   -1, 0x18,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "CLD": [   -1,   -1,   -1,   -1,   -1, 0xd8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "CLI": [   -1,   -1,   -1,   -1,   -1, 0x58,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "CLV": [   -1,   -1,   -1,   -1,   -1, 0xb8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "CMP": [ 0xcd, 0xdd, 0xd9,   -1, 0xc9,   -1, 0xc1,   -1, 0xd1,   -1, 0xc5, 0xd5,   -1,   -1, ],
        "CPX": [ 0xec,   -1,   -1,   -1, 0xe0,   -1,   -1,   -1,   -1,   -1, 0xe4,   -1,   -1,   -1, ],
        "CPY": [ 0xcc,   -1,   -1,   -1, 0xc0,   -1,   -1,   -1,   -1,   -1, 0xc4,   -1,   -1,   -1, ],
        "DEC": [ 0xce, 0xde,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xc6, 0xd6,   -1,   -1, ],
        "DEX": [   -1,   -1,   -1,   -1,   -1, 0xca,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "DEY": [   -1,   -1,   -1,   -1,   -1, 0x88,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "EOR": [ 0x4d, 0x5d, 0x59,   -1, 0x49,   -1, 0x41,   -1, 0x51,   -1, 0x45, 0x55,   -1,   -1, ],
        "INC": [ 0xee, 0xfe,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0xe6, 0xf6,   -1,   -1, ],
        "INX": [   -1,   -1,   -1,   -1,   -1, 0xe8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "INY": [   -1,   -1,   -1,   -1,   -1, 0xc8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "JMP": [ 0x4c,   -1,   -1,   -1,   -1,   -1,   -1, 0x6c,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "JSR": [ 0x20,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "LDA": [ 0xad, 0xbd, 0xb9,   -1, 0xa9,   -1, 0xa1,   -1, 0xb1,   -1, 0xa5, 0xb5,   -1,   -1, ],
        "LDX": [ 0xae,   -1, 0xbe,   -1, 0xa2,   -1,   -1,   -1,   -1,   -1, 0xa6,   -1, 0xb6,   -1, ],
        "LDY": [ 0xac, 0xbc,   -1,   -1, 0xa0,   -1,   -1,   -1,   -1,   -1, 0xa4, 0xb4,   -1,   -1, ],
        "LSR": [ 0x4e, 0x5e,   -1, 0x4a,   -1,   -1,   -1,   -1,   -1,   -1, 0x46, 0x56,   -1,   -1, ],
        "NOP": [   -1,   -1,   -1,   -1,   -1, 0xea,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "ORA": [ 0x0d, 0x1d, 0x19,   -1, 0x09,   -1, 0x01,   -1, 0x11,   -1, 0x05, 0x15,   -1,   -1, ],
        "PHA": [   -1,   -1,   -1,   -1,   -1, 0x48,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "PHP": [   -1,   -1,   -1,   -1,   -1, 0x08,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "PLA": [   -1,   -1,   -1,   -1,   -1, 0x68,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "PLP": [   -1,   -1,   -1,   -1,   -1, 0x28,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "ROL": [ 0x2e, 0x3e,   -1, 0x2a,   -1,   -1,   -1,   -1,   -1,   -1, 0x26, 0x36,   -1,   -1, ],
        "ROR": [ 0x6e, 0x7e,   -1, 0x6a,   -1,   -1,   -1,   -1,   -1,   -1, 0x66, 0x76,   -1,   -1, ],
        "RTI": [   -1,   -1,   -1,   -1,   -1, 0x40,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "RTS": [   -1,   -1,   -1,   -1,   -1, 0x60,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "SBC": [ 0xed, 0xfd, 0xf9,   -1, 0xe9,   -1, 0xe1,   -1, 0xf1,   -1, 0xe5, 0xf5,   -1,   -1, ],
        "SEC": [   -1,   -1,   -1,   -1,   -1, 0x38,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "SED": [   -1,   -1,   -1,   -1,   -1, 0xf8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "SEI": [   -1,   -1,   -1,   -1,   -1, 0x78,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "STA": [ 0x8d, 0x9d, 0x99,   -1,   -1,   -1, 0x81,   -1, 0x91,   -1, 0x85, 0x95,   -1,   -1, ],
        "STX": [ 0x8e,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0x86,   -1, 0x96,   -1, ],
        "STY": [ 0x8c,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, 0x84, 0x94,   -1,   -1, ],
        "TAX": [   -1,   -1,   -1,   -1,   -1, 0xaa,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "TAY": [   -1,   -1,   -1,   -1,   -1, 0xa8,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "TSX": [   -1,   -1,   -1,   -1,   -1, 0xba,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "TXA": [   -1,   -1,   -1,   -1,   -1, 0x8a,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "TXS": [   -1,   -1,   -1,   -1,   -1, 0x9a,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
        "TYA": [   -1,   -1,   -1,   -1,   -1, 0x98,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, ],
    }

    names = [
        "BRK",
        "ORA ($%02x,X)",
        "illop_02",
        "illop_03",
        "illop_04",
        "ORA $%02x",
        "ASL $%02x",
        "illop_07",
        "PHP",
        "ORA #$%02x",
        "ASL",
        "illop_0b",
        "illop_0c",
        "ORA $%04x",
        "ASL $%04x",
        "illop_0f",
        "BPL $%02x",
        "ORA ($%02x),Y",
        "illop_12",
        "illop_13",
        "illop_14",
        "ORA $%02x,X",
        "ASL $%02x,X",
        "illop_17",
        "CLC",
        "ORA $%04x,Y",
        "illop_1a",
        "illop_1b",
        "illop_1c",
        "ORA $%04x,X",
        "ASL $%04x,X",
        "illop_1f",
        "JSR $%04x",
        "AND ($%02x,X)",
        "illop_22",
        "illop_23",
        "BIT $%02x",
        "AND $%02x",
        "ROL $%02x",
        "illop_27",
        "PLP",
        "AND #$%02x",
        "ROL",
        "illop_2b",
        "BIT $%04x",
        "AND $%04x",
        "ROL $%04x",
        "illop_2f",
        "BMI $%02x",
        "AND ($%02x),Y",
        "illop_32",
        "illop_33",
        "illop_34",
        "AND $%02x,X",
        "ROL $%02x,X",
        "illop_37",
        "SEC",
        "AND $%04x,Y",
        "illop_3a",
        "illop_3b",
        "illop_3c",
        "AND $%04x,X",
        "ROL $%04x,X",
        "illop_3f",
        "RTI",
        "EOR ($%02x,X)",
        "illop_42",
        "illop_43",
        "illop_44",
        "EOR $%02x",
        "LSR $%02x",
        "illop_47",
        "PHA",
        "EOR #$%02x",
        "LSR",
        "illop_4b",
        "JMP $%04x",
        "EOR $%04x",
        "LSR $%04x",
        "illop_4f",
        "BVC $%02x",
        "EOR ($%02x),Y",
        "illop_52",
        "illop_53",
        "illop_54",
        "EOR $%02x,X",
        "LSR $%02x,X",
        "illop_57",
        "CLI",
        "EOR $%04x,Y",
        "illop_5a",
        "illop_5b",
        "illop_5c",
        "EOR $%04x,X",
        "LSR $%04x,X",
        "illop_5f",
        "RTS",
        "ADC ($%02x,X)",
        "illop_62",
        "illop_63",
        "illop_64",
        "ADC $%02x",
        "ROR $%02x",
        "illop_67",
        "PLA",
        "ADC #$%02x",
        "ROR",
        "illop_6b",
        "JMP ($%04x)",
        "ADC $%04x",
        "ROR $%04x",
        "illop_6f",
        "BVS $%02x",
        "ADC ($%02x),Y",
        "illop_72",
        "illop_73",
        "illop_74",
        "ADC $%02x,X",
        "ROR $%02x,X",
        "illop_77",
        "SEI",
        "ADC $%04x,Y",
        "illop_7a",
        "illop_7b",
        "illop_7c",
        "ADC $%04x,X",
        "ROR $%04x,X",
        "illop_7f",
        "illop_80",
        "STA ($%02x,X)",
        "illop_82",
        "illop_83",
        "STY $%02x",
        "STA $%02x",
        "STX $%02x",
        "illop_87",
        "DEY",
        "illop_89",
        "TXA",
        "illop_8b",
        "STY $%04x",
        "STA $%04x",
        "STX $%04x",
        "illop_8f",
        "BCC $%02x",
        "STA ($%02x),Y",
        "illop_92",
        "illop_93",
        "STY $%02x,X",
        "STA $%02x,X",
        "STX $%02x,Y",
        "illop_97",
        "TYA",
        "STA $%04x,Y",
        "TXS",
        "illop_9b",
        "illop_9c",
        "STA $%04x,X",
        "illop_9e",
        "illop_9f",
        "LDY #$%02x",
        "LDA ($%02x,X)",
        "LDX #$%02x",
        "illop_a3",
        "LDY $%02x",
        "LDA $%02x",
        "LDX $%02x",
        "illop_a7",
        "TAY",
        "LDA #$%02x",
        "TAX",
        "illop_ab",
        "LDY $%04x",
        "LDA $%04x",
        "LDX $%04x",
        "illop_af",
        "BCS $%02x",
        "LDA ($%02x),Y",
        "illop_b2",
        "illop_b3",
        "LDY $%02x,X",
        "LDA $%02x,X",
        "LDX $%02x,Y",
        "illop_b7",
        "CLV",
        "LDA $%04x,Y",
        "TSX",
        "illop_bb",
        "LDY $%04x,X",
        "LDA $%04x,X",
        "LDX $%04x,Y",
        "illop_bf",
        "CPY #$%02x",
        "CMP ($%02x,X)",
        "illop_c2",
        "illop_c3",
        "CPY $%02x",
        "CMP $%02x",
        "DEC $%02x",
        "illop_c7",
        "INY",
        "CMP #$%02x",
        "DEX",
        "illop_cb",
        "CPY $%04x",
        "CMP $%04x",
        "DEC $%04x",
        "illop_cf",
        "BNE $%02x",
        "CMP ($%02x),Y",
        "illop_d2",
        "illop_d3",
        "illop_d4",
        "CMP $%02x,X",
        "DEC $%02x,X",
        "illop_d7",
        "CLD",
        "CMP $%04x,Y",
        "illop_da",
        "illop_db",
        "illop_dc",
        "CMP $%04x,X",
        "DEC $%04x,X",
        "illop_df",
        "CPX #$%02x",
        "SBC ($%02x,X)",
        "illop_e2",
        "illop_e3",
        "CPX $%02x",
        "SBC $%02x",
        "INC $%02x",
        "illop_e7",
        "INX",
        "SBC #$%02x",
        "NOP",
        "illop_eb",
        "CPX $%04x",
        "SBC $%04x",
        "INC $%04x",
        "illop_ef",
        "BEQ $%02x",
        "SBC ($%02x),Y",
        "illop_f2",
        "illop_f3",
        "illop_f4",
        "SBC $%02x,X",
        "INC $%02x,X",
        "illop_f7",
        "SED",
        "SBC $%04x,Y",
        "illop_fa",
        "illop_fb",
        "illop_fc",
        "SBC $%04x,X",
        "INC $%04x,X",
        "illop_ff",
    ]

    xlate = [
        # No address parsed
        ACCUMULATOR, None, None, None,              # 0
        None, None, None, None,                     # 4
        None, None, None, None,                     # 8
        None, None, None, None,                     # 12
        None, None, None, None,                     # 16
        None, None, None, None,                     # 20
        None, None, None, None,                     # 24
        None, None, None, None,                     # 28

        # Parsed an address
        ZEROPAGE, ZEROPAGEX, ZEROPAGEY, None,       # 32
        INDIRECT, INDIRECTX, INDIRECTY, None,       # 36
        IMMEDIATE, None, None, None,                # 40
        None, None, None, None,                     # 44
        ABSOLUTE, ABSOLUTEX, ABSOLUTEY, None,       # 48
        INDIRECT, None, None, None,                 # 52
        None, None, None, None,                     # 56
        None, None, None, None,                     # 60
    ]

    def __init__(self, rom, org=0, bank=-1):
        self.rom = rom
        self.org = org
        self.last_org = self.org
        self.bank = bank
        self.reset()

    def reset(self):
        self.symtab = {}
        self.code_fixup = {}
        self.data_fixup = {}
        self.linenum = 0

    def asm(self, src, *, org=None, bank=None):
        self.reset()
        if org is not None:
            self.org = org
            self.last_org = self.org
        if bank is not None:
            self.bank = bank
        for line in src.split('\n'):
            self.linenum += 1
            line = line.replace('\r', '')
            self.parse_line(line)
        self.apply_fixups()

    def __call__(self, src, *, org=None, bank=None):
        self.asm(src, org=org, bank=bank)

    def disassemble_one(self, pc):
        opcode = self.read(pc)
        name = self.names[opcode]
        if '02x' in name:
            size = 2
            arg1 = self.read(pc + 1)
            r1 = '%04x: %02x%02x' % (pc, opcode, arg1)
            r2 = name % (arg1)
            if (opcode & 0x1F) == 0x10:
                dest = pc + 2 + -128 + (0x80 ^ arg1)
                r2 += '   ;[dest=%04x]' % dest
        elif '04x' in name:
            size = 3
            arg1 = self.read(pc + 1)
            arg2 = self.read(pc + 2)
            r1 = '%04x: %02x%02x%02x' % (pc, opcode, arg1, arg2)
            r2 = name % (arg1 | (arg2 << 8))
        else:
            size = 1
            r1 = '%04x: %02x' % (pc, opcode)
            r2 = name
        return (size, '%-20s%s' % (r1, r2))

    def dis(self, length, *, org=None, bank=None):
        if org is not None:
            self.org = org
            self.last_org = self.org
        if bank is not None:
            self.bank = bank

        for _ in range(length):
            (size, text) = self.disassemble_one(self.org)
            print(text)
            self.org += size
        return None

    def read(self, address):
        address = nes_address_adaptor(self.bank, address)
        return self.rom.read(address)

    def write(self, address, value):
        address = nes_address_adaptor(self.bank, address)
        self.rom.write(address, value & 0xFF)

    def write16(self, address, value):
        address = nes_address_adaptor(self.bank, address)
        self.rom.write_word(address, value & 0xFFFF)

    def parse_int(self, val):
        if re.fullmatch(r'-?\$[0-9a-fA-F]+', val):
            return int(val.replace('$', ''), 16)
        elif (re.fullmatch(r'-?[0-9]+', val)
              or re.fullmatch(r'-?0x[0-9a-fA-F]+', val)):
            return int(val, 0)
        return None

    def parse_data(self, opcode, operand):
        if opcode == '.DB':
            size = 1
        elif opcode == '.DW':
            size = 2
        elif opcode == '.DD':
            size = 4

        for data in operand.split(','):
            data = data.strip()
            if not data: continue
            val = 0xFFFFFFFF
            ival = self.parse_int(data)
            if ival is not None:
                val = ival
            else:
                self.data_fixup[self.org] = (size, data)
            for _ in range(size):
                self.write(self.org, val & 0xFF)
                self.org += 1
                val >>= 8
        return None

    def repeatb(self, operand):
        data = operand.split(',')
        if len(data) != 2:
            raise OperandError('Expecting exactly 2 operands')
        length = self.parse_int(data[0].strip())
        val = self.parse_int(data[1].strip())
        for _ in range(length):
            self.write(self.org, val & 0xFF)
            self.org += 1

    def parse_line(self, line):
        orig_line = line
        # Strip comments and transform to upper case
        (line, *_) = line.split(';', 1)
        line = line.upper()

        # Does this line have a label?
        if ':' in line:
            (label, *line) = line.split(':', 1)
            label = label.strip()
            self.symtab[label] = self.org
            line = line[0] if line else ''

        line = line.strip()
        if not line:
            return None

        # Is this an assignment?
        if '=' in line:
            (label, operand) = line.split('=')
            label = label.strip()
            operand = operand.strip()
            opcode = '='
        else:
            (opcode, *operand) = line.split(' ', 1)
            operand = operand[0].strip() if operand else ''

        # Handle data definitions
        if opcode in ('.DB', '.DW', '.DD'):
            return self.parse_data(opcode, operand)
        if opcode == '.REPEATB':
            return self.repeatb(operand)

        # Valid opcode?
        info = self.mnemonic.get(opcode)
        if info is None:
            raise OpcodeError("Unkown opcode", opcode, self.linenum, orig_line)

        # Parse the operand and determine the addressing mode
        addr = 0
        mode = 0
        ops = 0
        ope = len(operand)
        fixup_resolved = True

        p = operand.find(',X')
        if p != -1:
            mode |= 1; ope = p
        p = operand.find(',Y')
        if p != -1:
            mode |= 2; ope = p
        p = operand.find('(')
        if p != -1:
            mode |= 4; ops = p+1
            p = operand.find(')')
            if p != -1 and p < ope: ope = p

        p = operand.find('#')
        if p != -1:
            mode |= 8; ops = p + 1
        p = operand.find('!')
        if p != -1:
            mode |= 16; ops = p + 1

        operand = operand[ops:ope]
        ival = self.parse_int(operand)
        if ival is not None:
            addr = ival
            if addr >= 256: mode |= 16
            mode |= 32
        elif operand == '*':
            addr = self.org
            mode |= 32 | 16
        elif operand:
            try:
                addr = self.symtab[operand]
            except KeyError:
                fixup_resolved = False
                self.code_fixup[self.org] = operand
                addr = 0xffff
            if addr >= 256:
                mode |= 16
            mode |= 32

        # Translate the mode guess into the real mode
        omode = mode
        mode = self.xlate[mode]
        if mode is None:
            raise OperandError("Invalid Operand", opcode, operand, self.linenum, orig_line)
        if info[mode] == -1:
            if info[Asm.RELATIVE] != -1: mode = Asm.RELATIVE
            if info[Asm.IMPLIED] != -1: mode = Asm.IMPLIED
            if info[Asm.FAKE] != -1: mode = Asm.FAKE
        if info[mode] == -1:
            raise OperandError("Invalid addressing mode", opcode, operand, self.linenum, orig_line)

        # Write the assembled code into the ROM.
        if (mode == Asm.ABSOLUTE
            or mode == Asm.ABSOLUTEX
            or mode == Asm.ABSOLUTEY
            or mode == Asm.INDIRECT):
            self.write(self.org, info[mode])
            self.write16(self.org + 1, addr)
            if not fixup_resolved:
                self.code_fixup[self.org] = (mode, operand)
            self.org += 3
        elif (mode == Asm.ZEROPAGE
            or mode == Asm.ZEROPAGEX
            or mode == Asm.ZEROPAGEY
            or mode == Asm.IMMEDIATE
            or mode == Asm.INDIRECTX
            or mode == Asm.INDIRECTY):
            self.write(self.org, info[mode])
            self.write(self.org + 1, addr)
            if not fixup_resolved:
                self.code_fixup[self.org] = (mode, operand)
            self.org += 2
        elif mode == Asm.RELATIVE:
            self.write(self.org, info[mode])
            if fixup_resolved:
                delta = addr - (self.org + 2)
                if delta < -128 or delta > 127:
                    raise OperandError("Bad displacement", self.org, delta, self.linenum, orig_line)
            else:
                delta = 0xff
            if not fixup_resolved:
                self.code_fixup[self.org] = (mode, operand)
            self.write(self.org + 1, delta)
            self.org += 2
        elif (mode == Asm.ACCUMULATOR
            or mode == Asm.IMPLIED):
            self.write(self.org, info[mode])
            self.org += 1
        elif mode == Asm.FAKE:
            if not fixup_resolved:
                raise OperandError("Unresolved symbol", opcode, operand, self.linenum, orig_line)
            if opcode == '.ORG':
                print("Assembler: ORG changed %04x => %04x" % (self.org, addr))
                self.org = addr
                self.last_org = self.org
            elif opcode == ".ASSERT_ORG":
                if self.org != addr:
                    raise OperandError("ORG assertion failed on line %d (org=$%04x, expected=$%04x, delta since last org=%d)" % (
                        self.linenum, self.org, addr, self.org-self.last_org),
                        self.linenum, self.org, addr, self.org-self.last_org, orig_line)
            elif opcode == ".BANK":
                self.bank = addr
            elif opcode == "=":
                self.symtab[label] = addr
        else:
            raise OperandError("Invalid mode", mode, self.linenum, orig_line)
        return None

    def apply_fixups(self):
        for (addr, (mode, sym)) in self.code_fixup.items():
            value = self.symtab.get(sym)
            if value is None:
                raise FixupError("Unresolved symbol", sym, addr)

            if (mode == Asm.ABSOLUTE
                or mode == Asm.ABSOLUTEX
                or mode == Asm.ABSOLUTEY
                or mode == Asm.INDIRECT):
                self.write16(addr + 1, value)
            elif (mode == Asm.ZEROPAGE
                or mode == Asm.ZEROPAGEX
                or mode == Asm.ZEROPAGEY
                or mode == Asm.IMMEDIATE
                or mode == Asm.INDIRECTX
                or mode == Asm.INDIRECTY):
                self.write(addr + 1, value)
            elif mode == Asm.RELATIVE:
                delta = value - (addr + 2)
                if delta < -128 or delta > 127:
                    raise OperandError("Bad displacement", addr, delta)
                self.write(addr + 1, delta)
            else:
                raise FixupError("Invalid mode", mode, addr)

        for (addr, (size, sym)) in self.data_fixup.items():
            value = self.symtab.get(sym)
            if value is None:
                raise FixupError("Unresolved symbol", sym, addr)
            for _ in range(size):
                self.write(addr, value & 0xFF)
                value >>= 8
                addr += 1
        return None
