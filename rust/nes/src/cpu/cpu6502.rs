use pyo3::prelude::*;
use super::cpu6502_info::{AddressingMode, INFO, NAMES};
use serde::{Deserialize, Serialize};
use bitflags::bitflags;
use crate::Nes;
use crate::Address;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct CpuFlags: u8 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const U = 0b00100000;
        const V = 0b01000000;
        const N = 0b10000000;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: CpuFlags,
    pub sp: u8,
    pub pc: u16,
    pub reset_pending: bool,
    pub nmi_pending: bool,
    pub irq_pending: bool,
    pub halted: bool,
    pub cycles: u64,
}

impl Default for Cpu6502 {
    fn default() -> Self {
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            p: CpuFlags::default(),
            sp: 0,
            pc: 0,
            reset_pending: true,
            nmi_pending: false,
            irq_pending: false,
            halted: false,
            cycles: 0,
        }
    }
}


impl Cpu6502 {
    fn read16<'py>(nes: &Bound<'py, Nes>, address: u16) -> u16 {
        Nes::read(nes, Address::Cpu(address)) as u16 | (Nes::read(nes, Address::Cpu(address + 1)) as u16) << 8
    }

    fn read16bug<'py>(nes: &Bound<'py, Nes>, address: u16) -> u16 {
        // When reading the high byte of a word, the address increments,
        // but doesn't carry from low address byte to the high address byte.
        Nes::read(nes, Address::Cpu(address)) as u16
            | (Nes::read(nes, Address::Cpu((address & 0xFF00) | ((address + 1) & 0x00FF))) as u16) << 8
    }

    fn push<'py>(&mut self, nes: &Bound<'py, Nes>, value: u8) {
        Nes::write(nes, Address::Cpu(0x100u16 | self.sp as u16), value);
        self.sp = self.sp.wrapping_sub(1);
    }
    fn push16<'py>(&mut self, nes: &Bound<'py, Nes>, value: u16) {
        self.push(nes, (value >> 8) as u8);
        self.push(nes, (value & 255) as u8);
    }

    fn pull<'py>(&mut self, nes: &Bound<'py, Nes>) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        Nes::read(nes, Address::Cpu(0x100u16 | self.sp as u16))
    }
    fn pull16<'py>(&mut self, nes: &Bound<'py, Nes>) -> u16 {
        self.pull(nes) as u16 | (self.pull(nes) as u16) << 8
    }

    fn pages_differ(a: u16, b: u16, vtrue: u8, vfalse: u8) -> u64 {
        if (a & 0xFF00) != (b & 0xFF00) {
            vtrue as u64
        } else {
            vfalse as u64
        }
    }

    pub fn cpustate(&self) -> String {
        let c = if self.p.contains(CpuFlags::C) {
            "C"
        } else {
            "c"
        };
        let z = if self.p.contains(CpuFlags::Z) {
            "Z"
        } else {
            "z"
        };
        let i = if self.p.contains(CpuFlags::I) {
            "I"
        } else {
            "i"
        };
        let d = if self.p.contains(CpuFlags::D) {
            "D"
        } else {
            "d"
        };
        let b = if self.p.contains(CpuFlags::B) {
            "B"
        } else {
            "b"
        };
        let u = if self.p.contains(CpuFlags::U) {
            "U"
        } else {
            "u"
        };
        let v = if self.p.contains(CpuFlags::V) {
            "V"
        } else {
            "v"
        };
        let n = if self.p.contains(CpuFlags::N) {
            "N"
        } else {
            "n"
        };
        format!(
            "PC={:04x} A={:02x} X={:02x} Y={:02x} SP=1{:02x} {}{}{}{}{}{}{}{}",
            self.pc, self.a, self.x, self.y, self.sp, n, v, u, b, d, i, z, c
        )
    }

    pub fn disassemble<'py>(nes: &Bound<'py, Nes>, addr: u16) -> (String, u16) {
        let opcode = Nes::read(nes, Address::Cpu(addr));
        let info = INFO[opcode as usize];
        let name = NAMES[opcode as usize];
        match info.size {
            2 => {
                let operand = Nes::read(nes, Address::Cpu(addr.wrapping_add(1)));
                (
                    format!(
                        "{:04x}: {:02x}{:02x}          {}",
                        addr,
                        opcode,
                        operand,
                        name.replace("@", format!("{:02x}", operand).as_str())
                    ),
                    addr.wrapping_add(2),
                )
            }
            3 => {
                let op1 = Nes::read(nes, Address::Cpu(addr.wrapping_add(1)));
                let op2 = Nes::read(nes, Address::Cpu(addr.wrapping_add(2)));
                (
                    format!(
                        "{:04x}: {:02x}{:02x}{:02x}        {}",
                        addr,
                        opcode,
                        op1,
                        op2,
                        name.replace("@", format!("{:02x}{:02x}", op2, op1).as_str())
                    ),
                    addr.wrapping_add(3),
                )
            }
            0 | 1 | _ => (
                format!("{:04x}: {:02x}            {}", addr, opcode, name),
                addr.wrapping_add(1),
            ),
        }
    }

    fn branch(&mut self, target: u16, cond: bool) {
        if cond {
            self.cycles += Cpu6502::pages_differ(self.pc, target, 2, 1);
            self.pc = target;
        }
    }

    fn set_z(&mut self, val: u8) {
        if val == 0 {
            self.p.insert(CpuFlags::Z);
        } else {
            self.p.remove(CpuFlags::Z);
        }
    }
    fn set_n(&mut self, val: u8) {
        if val & 0x80 == 0x80 {
            self.p.insert(CpuFlags::N);
        } else {
            self.p.remove(CpuFlags::N);
        }
    }
    fn set_c(&mut self, val: bool) {
        if val {
            self.p.insert(CpuFlags::C);
        } else {
            self.p.remove(CpuFlags::C);
        }
    }
    fn set_v(&mut self, val: bool) {
        if val {
            self.p.insert(CpuFlags::V);
        } else {
            self.p.remove(CpuFlags::V);
        }
    }

    fn set_zn(&mut self, val: u8) {
        self.set_z(val);
        self.set_n(val);
    }
    fn compare(&mut self, a: u8, b: u8) {
        self.set_zn(a.wrapping_sub(b));
        self.set_c(a >= b);
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn signal_irq(&mut self) {
        self.irq_pending = true;
    }
    pub fn signal_nmi(&mut self) {
        self.nmi_pending = true;
    }
    pub fn reset(&mut self) {
        self.reset_pending = true;
        self.halted = false;
    }

    pub fn trace<'py>(&mut self, nes: &Bound<'py, Nes>) -> String {
        let (s, _next) = Cpu6502::disassemble(nes, self.pc);
        self.execute(nes);
        s
    }

    pub fn execute<'py>(&mut self, nes: &Bound<'py, Nes>) -> u64 {
        if self.halted {
            return 0;
        }
        let c = self.cycles;
        if self.reset_pending {
            self.reset_pending = false;
            self.p = CpuFlags::U | CpuFlags::B | CpuFlags::I;
            self.sp = 0xFD;
            self.a = 0;
            self.x = 0;
            self.y = 0;
            self.pc = Cpu6502::read16(nes, 0xFFFCu16);
        } else if self.nmi_pending {
            self.nmi_pending = false;
            self.push16(nes, self.pc);
            self.push(nes, (self.p | CpuFlags::B).bits);
            self.p.insert(CpuFlags::I);
            self.pc = Cpu6502::read16(nes, 0xFFFAu16);
            self.cycles += 7;
        } else if self.irq_pending && !self.p.contains(CpuFlags::I) {
            self.irq_pending = false;
            self.push16(nes, self.pc);
            self.push(nes, (self.p | CpuFlags::B).bits);
            self.p.insert(CpuFlags::I);
            self.pc = Cpu6502::read16(nes, 0xFFFEu16);
            self.cycles += 7;
        }

        // Read opcode from memory, then compute the opaddr address,
        // next PC and cycles consumed.
        let iaddr = self.pc;
        let opcode = Nes::read(nes, Address::Cpu(iaddr));
        let info = INFO[opcode as usize];
        let pc1 = self.pc.wrapping_add(1);
        self.pc = self.pc.wrapping_add(info.size as u16);
        self.cycles += info.cycles as u64;
        let opaddr = match info.mode {
            AddressingMode::Absolute => Cpu6502::read16(nes, pc1),
            AddressingMode::AbsoluteX => {
                let addr = Cpu6502::read16(nes, pc1);
                let newaddr = addr.wrapping_add(self.x as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::AbsoluteY => {
                let addr = Cpu6502::read16(nes, pc1);
                let newaddr = addr.wrapping_add(self.y as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::IndexedIndirect => {
                let operand = Nes::read(nes, Address::Cpu(pc1));
                Cpu6502::read16(nes, operand.wrapping_add(self.x) as u16)
            }
            AddressingMode::Indirect => {
                let operand = Cpu6502::read16(nes, pc1);
                Cpu6502::read16bug(nes, operand)
            }
            AddressingMode::IndirectIndexed => {
                let operand = Nes::read(nes, Address::Cpu(pc1));
                let addr = Cpu6502::read16(nes, operand as u16);
                let newaddr = addr.wrapping_add(self.y as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::ZeroPage => Nes::read(nes, Address::Cpu(pc1)) as u16,
            AddressingMode::ZeroPageX => (Nes::read(nes, Address::Cpu(pc1)).wrapping_add(self.x)) as u16,
            AddressingMode::ZeroPageY => (Nes::read(nes, Address::Cpu(pc1)).wrapping_add(self.y)) as u16,
            AddressingMode::Immediate => pc1,
            AddressingMode::Accumulator => 0,
            AddressingMode::Implied => 0,
            AddressingMode::Relative => {
                let mut disp = Nes::read(nes, Address::Cpu(pc1)) as u16;
                if disp & 0x80 == 0x80 {
                    disp |= 0xFF00;
                }
                self.pc.wrapping_add(disp)
            }
        };

        // Match the opcode and execute.
        match opcode {
            // BRK
            0x00 => {
                self.push16(nes, self.pc.wrapping_add(1));
                self.push(nes, (self.p | CpuFlags::B).bits);
                self.p |= CpuFlags::I;
                self.pc = Cpu6502::read16(nes, 0xFFFEu16);
            }

            // ORA <mem> opcodes
            0x01 | 0x05 | 0x09 | 0x0d | 0x11 | 0x15 | 0x19 | 0x1d => {
                self.a |= Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.a);
            }
            // ASL <mem>
            0x06 | 0x0e | 0x16 | 0x1e => {
                let val = Nes::read(nes, Address::Cpu(opaddr));
                self.set_c(val & 0x80 != 0);
                self.set_zn(val << 1);
                Nes::write(nes, Address::Cpu(opaddr), val << 1);
            }

            // ASL A
            0x0a => {
                self.set_c(self.a & 0x80 != 0);
                self.a <<= 1;
                self.set_zn(self.a);
            }

            // PHP
            0x08 => self.push(nes, (self.p | CpuFlags::B).bits),
            // BPL nn
            0x10 => self.branch(opaddr, !self.p.contains(CpuFlags::N)),
            // CLC
            0x18 => self.p.remove(CpuFlags::C),
            // JSR
            0x20 => {
                self.push16(nes, self.pc.wrapping_sub(1));
                self.pc = opaddr;
            }
            // AND <mem> opcodes
            0x21 | 0x25 | 0x29 | 0x2d | 0x31 | 0x35 | 0x39 | 0x3d => {
                self.a &= Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.a);
            }
            // BIT <mem> opcodes
            0x24 | 0x2c => {
                let val = Nes::read(nes, Address::Cpu(opaddr));
                self.set_v(val & 0x40 == 0x40);
                self.set_z(val & self.a);
                self.set_n(val);
            }
            // ROL <mem> opaddrs
            0x26 | 0x2e | 0x36 | 0x3e => {
                let r = Nes::read(nes, Address::Cpu(opaddr)) as u16;
                let carry = if self.p.contains(CpuFlags::C) {
                    1u16
                } else {
                    0u16
                };
                let r = (r << 1) | carry;
                self.set_c(r >= 0x100);
                self.set_zn(r as u8);
                Nes::write(nes, Address::Cpu(opaddr), r as u8);
            }
            // PLP
            0x28 => self.p.bits = (self.pull(nes) & 0xEF) | 0x20,
            // ROL A
            0x2a => {
                let r = self.a as u16;
                let carry = if self.p.contains(CpuFlags::C) {
                    1u16
                } else {
                    0u16
                };
                let r = (r << 1) | carry;
                self.set_c(r >= 0x100);
                self.set_zn(r as u8);
                self.a = r as u8;
            }
            // BMI nn
            0x30 => self.branch(opaddr, self.p.contains(CpuFlags::N)),
            // SEC
            0x38 => self.p.insert(CpuFlags::C),
            // RTI
            0x40 => {
                self.p.bits = (self.pull(nes) & 0xEF) | 0x20;
                self.pc = self.pull16(nes);
            }

            // EOR <mem> opcodes
            0x41 | 0x45 | 0x49 | 0x4d | 0x51 | 0x55 | 0x59 | 0x5d => {
                self.a ^= Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.a);
            }
            // LSR <mem>
            0x46 | 0x4e | 0x56 | 0x5e => {
                let val = Nes::read(nes, Address::Cpu(opaddr));
                self.set_c(val & 0x01 != 0);
                self.set_zn(val >> 1);
                Nes::write(nes, Address::Cpu(opaddr), val >> 1);
            }

            // PHA
            0x48 => self.push(nes, self.a),
            // BVC nn
            0x50 => self.branch(opaddr, !self.p.contains(CpuFlags::V)),
            // JMP nnnn, JMP (nnnn)
            0x4c | 0x6c => self.pc = opaddr,

            // LSR A
            0x4a => {
                self.set_c(self.a & 0x01 != 0);
                self.a >>= 1;
                self.set_zn(self.a);
            }
            // CLI
            0x58 => self.p.remove(CpuFlags::I),
            // RTS
            0x60 => self.pc = self.pull16(nes).wrapping_add(1),

            // ADC <mem> opcodes
            0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x75 | 0x79 | 0x7d => {
                let a = self.a;
                let b = Nes::read(nes, Address::Cpu(opaddr));
                let carry = if self.p.contains(CpuFlags::C) {
                    1u16
                } else {
                    0u16
                };
                let r = (a as u16) + (b as u16) + carry;
                self.a = r as u8;
                self.set_c(r >= 0x100);
                self.set_v(((a ^ b) & 0x80) == 0 && (a ^ self.a) & 0x80 != 0);
                self.set_zn(self.a);
            }

            // ROR <mem>
            0x66 | 0x6e | 0x76 | 0x7e => {
                let val = Nes::read(nes, Address::Cpu(opaddr));
                let carry = if self.p.contains(CpuFlags::C) {
                    1u8
                } else {
                    0u8
                };
                let a = (val >> 1) | (carry << 7);
                self.set_c(val & 0x01 != 0);
                self.set_zn(a);
                Nes::write(nes, Address::Cpu(opaddr), a);
            }
            // PLA
            0x68 => {
                self.a = self.pull(nes);
                self.set_zn(self.a);
            }
            // ROR A
            0x6a => {
                let val = self.a;
                let carry = if self.p.contains(CpuFlags::C) {
                    1u8
                } else {
                    0u8
                };
                self.a = (val >> 1) | (carry << 7);
                self.set_c(val & 0x01 != 0);
                self.set_zn(self.a);
            }
            // BVC nn
            0x70 => self.branch(opaddr, self.p.contains(CpuFlags::V)),
            // SEI
            0x78 => self.p.insert(CpuFlags::I),

            // STA <mem> opcodes
            0x81 | 0x85 | 0x8d | 0x91 | 0x95 | 0x99 | 0x9d => {
                Nes::write(nes, Address::Cpu(opaddr), self.a);
            }

            // STY <mem> opcodes
            0x84 | 0x8c | 0x94 => Nes::write(nes, Address::Cpu(opaddr), self.y),
            // STX <mem> opcodes
            0x86 | 0x8e | 0x96 => Nes::write(nes, Address::Cpu(opaddr), self.x),

            // DEY
            0x88 => {
                self.y = self.y.wrapping_sub(1);
                self.set_zn(self.y);
            }
            // TXA
            0x8a => {
                self.a = self.x;
                self.set_zn(self.a);
            }

            // BCC nn
            0x90 => self.branch(opaddr, !self.p.contains(CpuFlags::C)),

            // TYA
            0x98 => {
                self.a = self.y;
                self.set_zn(self.a);
            }
            // TXS
            0x9a => self.sp = self.x,

            // LDY <mem> opcodes
            0xa0 | 0xa4 | 0xac | 0xb4 | 0xbc => {
                self.y = Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.y);
            }
            // LDX <mem> opcodes
            0xa2 | 0xa6 | 0xae | 0xb6 | 0xbe => {
                self.x = Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.x);
            }
            // LDA <mem> opcodes
            0xA1 | 0xA5 | 0xA9 | 0xAd | 0xB1 | 0xB5 | 0xB9 | 0xBd => {
                self.a = Nes::read(nes, Address::Cpu(opaddr));
                self.set_zn(self.a);
            }

            // TAY
            0xa8 => {
                self.y = self.a;
                self.set_zn(self.y);
            }
            // TAX
            0xaa => {
                self.x = self.a;
                self.set_zn(self.x);
            }

            // BCS nn
            0xb0 => self.branch(opaddr, self.p.contains(CpuFlags::C)),
            // CLV
            0xB8 => self.p.remove(CpuFlags::V),
            // TSX
            0xba => {
                self.x = self.sp;
                self.set_zn(self.x);
            }

            // CPY opcodes
            0xc0 | 0xc4 | 0xcc => self.compare(self.y, Nes::read(nes, Address::Cpu(opaddr))),
            // CMP opcodes
            0xc1 | 0xc5 | 0xc9 | 0xcd | 0xd1 | 0xd5 | 0xd9 | 0xdd => {
                self.compare(self.a, Nes::read(nes, Address::Cpu(opaddr)))
            }
            // DEC <mem> opcodes
            0xc6 | 0xce | 0xd6 | 0xde => {
                let val = Nes::read(nes, Address::Cpu(opaddr)).wrapping_sub(1);
                Nes::write(nes, Address::Cpu(opaddr), val);
                self.set_zn(val);
            }
            // INY
            0xc8 => {
                self.y = self.y.wrapping_add(1);
                self.set_zn(self.y);
            }
            // DEX
            0xca => {
                self.x = self.x.wrapping_sub(1);
                self.set_zn(self.x);
            }
            // BNE nn
            0xd0 => self.branch(opaddr, !self.p.contains(CpuFlags::Z)),

            // CLD
            0xd8 => self.p.remove(CpuFlags::D),
            // CPX opcodes
            0xe0 | 0xe4 | 0xec => self.compare(self.x, Nes::read(nes, Address::Cpu(opaddr))),

            // SBC <mem> opcodes
            0xe1 | 0xe5 | 0xe9 | 0xed | 0xf1 | 0xf5 | 0xf9 | 0xfd => {
                let a = self.a;
                let b = Nes::read(nes, Address::Cpu(opaddr));
                let ncarry = if self.p.contains(CpuFlags::C) {
                    0i16
                } else {
                    1i16
                };
                let r = (a as i16) - (b as i16) - ncarry;
                self.a = r as u8;
                self.set_c(r >= 0);
                self.set_v(((a ^ b) & 0x80) != 0 && (a ^ self.a) & 0x80 != 0);
                self.set_zn(self.a);
            }

            // INC <mem> opcodes
            0xe6 | 0xee | 0xf6 | 0xfe => {
                let val = Nes::read(nes, Address::Cpu(opaddr)).wrapping_add(1);
                Nes::write(nes, Address::Cpu(opaddr), val);
                self.set_zn(val);
            }

            // INX
            0xe8 => {
                self.x = self.x.wrapping_add(1);
                self.set_zn(self.x);
            }
            // NOP
            0xea => {}
            // BEQ nn
            0xf0 => self.branch(opaddr, self.p.contains(CpuFlags::Z)),
            //SED
            0xf8 => self.p.insert(CpuFlags::D),

            _ => {
                // Illegal opcode.
                self.halted = true;
                log::error!("Illegal opcode at ${:4x} = {:2x}", iaddr, opcode);
            }
        };

        // Return the number of cycles actually used executing this instruction.
        self.cycles - c
    }
}
