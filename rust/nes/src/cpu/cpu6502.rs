use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::cpu6502_info::{AddressingMode, INFO, NAMES};
use crate::Address;
use crate::Nes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
#[pyo3(get_all, set_all)]
pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
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
            p: Self::FLAG_U, // Default flags often include U
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
    // Flag constants
    const FLAG_C: u8 = 0b00000001;
    const FLAG_Z: u8 = 0b00000010;
    const FLAG_I: u8 = 0b00000100;
    const FLAG_D: u8 = 0b00001000;
    const FLAG_B: u8 = 0b00010000; // Break command
    const FLAG_U: u8 = 0b00100000; // Unused, always 1
    const FLAG_V: u8 = 0b01000000; // Overflow
    const FLAG_N: u8 = 0b10000000; // Negative

    fn read(&self, nes: &Nes, address: u16) -> u8 {
        let mut result = nes.read(Address::Cpu(address));
        let read_cb = nes.read_cb.lock().expect("failed to lock read_cb");
        if let Some(callback) = read_cb.get(&address) {
            result = Python::with_gil(|py| {
                match callback
                    .call1(py, (address, result))
                    .and_then(|val| val.extract::<u8>(py))
                {
                    Ok(val) => val,
                    Err(e) => {
                        log::error!("Read callback for {address:x?} failed: {e}");
                        result
                    }
                }
            });
        }
        result
    }

    fn write(&self, nes: &Nes, address: u16, value: u8) {
        let mut value = value;
        let write_cb = nes.write_cb.lock().expect("failed to lock write_cb");
        if let Some(callback) = write_cb.get(&address) {
            value = Python::with_gil(|py| {
                match callback
                    .call1(py, (address, value))
                    .and_then(|val| val.extract::<u8>(py))
                {
                    Ok(val) => val,
                    Err(e) => {
                        log::error!("Write callback for {address:x?} failed: {e}");
                        value
                    }
                }
            });
        }
        nes.write(Address::Cpu(address), value);
    }

    fn read16(&self, nes: &Nes, address: u16) -> u16 {
        u16::from_le_bytes([self.read(nes, address), self.read(nes, address + 1)])
    }

    fn read16bug(&self, nes: &Nes, address: u16) -> u16 {
        // When reading the high byte of a word, the address increments,
        // but doesn't carry from low address byte to the high address byte.
        u16::from_le_bytes([
            self.read(nes, address),
            self.read(nes, (address & 0xFF00) | ((address + 1) & 0x00FF)),
        ])
    }

    fn push(&mut self, nes: &Nes, value: u8) {
        self.write(nes, 0x100u16 | self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    fn push16(&mut self, nes: &Nes, value: u16) {
        self.push(nes, (value >> 8) as u8);
        self.push(nes, (value & 255) as u8);
    }
    fn pull(&mut self, nes: &Nes) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read(nes, 0x100u16 | self.sp as u16)
    }
    fn pull16(&mut self, nes: &Nes) -> u16 {
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
        let c = if (self.p & Self::FLAG_C) != 0 {
            "C"
        } else {
            "c"
        };
        let z = if (self.p & Self::FLAG_Z) != 0 {
            "Z"
        } else {
            "z"
        };
        let i = if (self.p & Self::FLAG_I) != 0 {
            "I"
        } else {
            "i"
        };
        let d = if (self.p & Self::FLAG_D) != 0 {
            "D"
        } else {
            "d"
        };
        let b = if (self.p & Self::FLAG_B) != 0 {
            "B"
        } else {
            "b"
        };
        let u = if (self.p & Self::FLAG_U) != 0 {
            "U"
        } else {
            "u"
        };
        let v = if (self.p & Self::FLAG_V) != 0 {
            "V"
        } else {
            "v"
        };
        let n = if (self.p & Self::FLAG_N) != 0 {
            "N"
        } else {
            "n"
        };
        format!(
            "PC={:04x} A={:02x} X={:02x} Y={:02x} SP=1{:02x} {}{}{}{}{}{}{}{}",
            self.pc, self.a, self.x, self.y, self.sp, n, v, u, b, d, i, z, c
        )
    }

    pub fn disassemble(nes: &Nes, addr: u16) -> (String, u16) {
        let opcode = nes.read(Address::Cpu(addr));
        let info = INFO[opcode as usize];
        let name = NAMES[opcode as usize];
        match info.size {
            2 => {
                let operand = nes.read(Address::Cpu(addr.wrapping_add(1)));
                (
                    format!(
                        "{:04x}: {:02x}{:02x}      {}",
                        addr,
                        opcode,
                        operand,
                        name.replace("@", format!("{:02x}", operand).as_str())
                    ),
                    addr.wrapping_add(2),
                )
            }
            3 => {
                let op1 = nes.read(Address::Cpu(addr.wrapping_add(1)));
                let op2 = nes.read(Address::Cpu(addr.wrapping_add(2)));
                (
                    format!(
                        "{:04x}: {:02x}{:02x}{:02x}    {}",
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
                format!("{:04x}: {:02x}        {}", addr, opcode, name),
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
            self.p |= Self::FLAG_Z;
        } else {
            self.p &= !Self::FLAG_Z;
        }
    }
    fn set_n(&mut self, val: u8) {
        if val & 0x80 == 0x80 {
            self.p |= Self::FLAG_N;
        } else {
            self.p &= !Self::FLAG_N;
        }
    }
    fn set_c(&mut self, val: bool) {
        if val {
            self.p |= Self::FLAG_C;
        } else {
            self.p &= !Self::FLAG_C;
        }
    }
    fn set_v(&mut self, val: bool) {
        if val {
            self.p |= Self::FLAG_V;
        } else {
            self.p &= !Self::FLAG_V;
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

    pub fn trace(&mut self, nes: &Nes) -> String {
        let (s, _next) = Cpu6502::disassemble(nes, self.pc);
        self.execute(nes);
        s
    }

    pub fn execute(&mut self, nes: &Nes) -> u64 {
        if self.halted {
            return 0;
        }
        let c = self.cycles;
        if self.reset_pending {
            self.reset_pending = false;
            self.p = Self::FLAG_U | Self::FLAG_B | Self::FLAG_I;
            self.sp = 0xFD;
            self.a = 0;
            self.x = 0;
            self.y = 0;
            self.pc = self.read16(nes, 0xFFFCu16);
        } else if self.nmi_pending {
            self.nmi_pending = false;
            self.push16(nes, self.pc);
            self.push(nes, self.p | Self::FLAG_B);
            self.p |= Self::FLAG_I;
            self.pc = self.read16(nes, 0xFFFAu16);
            self.cycles += 7;
        } else if self.irq_pending && (self.p & Self::FLAG_I) == 0 {
            self.irq_pending = false;
            self.push16(nes, self.pc);
            self.push(nes, self.p | Self::FLAG_B);
            self.p |= Self::FLAG_I;
            self.pc = self.read16(nes, 0xFFFEu16);
            self.cycles += 7;
        }

        // Read opcode from memory, then compute the opaddr address,
        // next PC and cycles consumed.
        let iaddr = self.pc;
        let opcode = self.read(nes, iaddr);
        let info = INFO[opcode as usize];
        let pc1 = self.pc.wrapping_add(1);
        self.pc = self.pc.wrapping_add(info.size as u16);
        self.cycles += info.cycles as u64;
        let opaddr = match info.mode {
            AddressingMode::Absolute => self.read16(nes, pc1),
            AddressingMode::AbsoluteX => {
                let addr = self.read16(nes, pc1);
                let newaddr = addr.wrapping_add(self.x as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::AbsoluteY => {
                let addr = self.read16(nes, pc1);
                let newaddr = addr.wrapping_add(self.y as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::IndexedIndirect => {
                let operand = self.read(nes, pc1);
                self.read16(nes, operand.wrapping_add(self.x) as u16)
            }
            AddressingMode::Indirect => {
                let operand = self.read16(nes, pc1);
                self.read16bug(nes, operand)
            }
            AddressingMode::IndirectIndexed => {
                let operand = self.read(nes, pc1);
                let addr = self.read16(nes, operand as u16);
                let newaddr = addr.wrapping_add(self.y as u16);
                self.cycles += Cpu6502::pages_differ(addr, newaddr, info.page, 0);
                newaddr
            }
            AddressingMode::ZeroPage => self.read(nes, pc1) as u16,
            AddressingMode::ZeroPageX => self.read(nes, pc1).wrapping_add(self.x) as u16,
            AddressingMode::ZeroPageY => self.read(nes, pc1).wrapping_add(self.y) as u16,
            AddressingMode::Immediate => pc1,
            AddressingMode::Accumulator => 0,
            AddressingMode::Implied => 0,
            AddressingMode::Relative => {
                let mut disp = self.read(nes, pc1) as u16;
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
                self.push(nes, self.p | Self::FLAG_B);
                self.p |= Self::FLAG_I;
                self.pc = self.read16(nes, 0xFFFEu16);
            }

            // ORA <mem> opcodes
            0x01 | 0x05 | 0x09 | 0x0d | 0x11 | 0x15 | 0x19 | 0x1d => {
                self.a |= self.read(nes, opaddr);
                self.set_zn(self.a);
            }
            // ASL <mem>
            0x06 | 0x0e | 0x16 | 0x1e => {
                let val = self.read(nes, opaddr);
                self.set_c(val & 0x80 != 0);
                self.set_zn(val << 1);
                self.write(nes, opaddr, val << 1);
            }

            // ASL A
            0x0a => {
                self.set_c(self.a & 0x80 != 0);
                self.a <<= 1;
                self.set_zn(self.a);
            }

            // PHP
            0x08 => self.push(nes, self.p | Self::FLAG_B | Self::FLAG_U), // B and U are set on stack
            // BPL nn
            0x10 => self.branch(opaddr, (self.p & Self::FLAG_N) == 0),
            // CLC
            0x18 => self.p &= !Self::FLAG_C,
            // JSR
            0x20 => {
                self.push16(nes, self.pc.wrapping_sub(1));
                self.pc = opaddr;
            }
            // AND <mem> opcodes
            0x21 | 0x25 | 0x29 | 0x2d | 0x31 | 0x35 | 0x39 | 0x3d => {
                self.a &= self.read(nes, opaddr);
                self.set_zn(self.a);
            }
            // BIT <mem> opcodes
            0x24 | 0x2c => {
                let val = self.read(nes, opaddr);
                self.set_v(val & 0x40 == 0x40);
                self.set_z(val & self.a);
                self.set_n(val);
            }
            // ROL <mem> opaddrs
            0x26 | 0x2e | 0x36 | 0x3e => {
                let r = self.read(nes, opaddr) as u16;
                let carry = if self.p & Self::FLAG_C != 0 {
                    1u16
                } else {
                    0u16
                };
                let r = (r << 1) | carry;
                self.set_c(r >= 0x100);
                self.set_zn(r as u8);
                self.write(nes, opaddr, r as u8);
            }
            // PLP
            0x28 => self.p = (self.pull(nes) & !Self::FLAG_B) | Self::FLAG_U,
            // ROL A
            0x2a => {
                let r = self.a as u16;
                let carry = if self.p & Self::FLAG_C != 0 {
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
            0x30 => self.branch(opaddr, (self.p & Self::FLAG_N) != 0),
            // SEC
            0x38 => self.p |= Self::FLAG_C,
            // RTI
            0x40 => {
                self.p = (self.pull(nes) & !Self::FLAG_B) | Self::FLAG_U;
                self.pc = self.pull16(nes);
            }

            // EOR <mem> opcodes
            0x41 | 0x45 | 0x49 | 0x4d | 0x51 | 0x55 | 0x59 | 0x5d => {
                self.a ^= self.read(nes, opaddr);
                self.set_zn(self.a);
            }
            // LSR <mem>
            0x46 | 0x4e | 0x56 | 0x5e => {
                let val = self.read(nes, opaddr);
                self.set_c(val & 0x01 != 0);
                self.set_zn(val >> 1);
                self.write(nes, opaddr, val >> 1);
            }

            // PHA
            0x48 => self.push(nes, self.a),
            // BVC nn
            0x50 => self.branch(opaddr, (self.p & Self::FLAG_V) == 0),
            // JMP nnnn, JMP (nnnn)
            0x4c | 0x6c => self.pc = opaddr,

            // LSR A
            0x4a => {
                self.set_c(self.a & 0x01 != 0);
                self.a >>= 1;
                self.set_zn(self.a);
            }
            // CLI
            0x58 => self.p &= !Self::FLAG_I,
            // RTS
            0x60 => self.pc = self.pull16(nes).wrapping_add(1),

            // ADC <mem> opcodes
            0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x75 | 0x79 | 0x7d => {
                let a = self.a;
                let b = self.read(nes, opaddr);
                let carry = if (self.p & Self::FLAG_C) != 0 {
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
                let val = self.read(nes, opaddr);
                let carry = if (self.p & Self::FLAG_C) != 0 {
                    1u8
                } else {
                    0u8
                };
                let a = (val >> 1) | (carry << 7);
                self.set_c(val & 0x01 != 0);
                self.set_zn(a);
                self.write(nes, opaddr, a);
            }
            // PLA
            0x68 => {
                self.a = self.pull(nes);
                self.set_zn(self.a);
            }
            // ROR A
            0x6a => {
                let val = self.a;
                let carry = if (self.p & Self::FLAG_C) != 0 {
                    1u8
                } else {
                    0u8
                };
                self.a = (val >> 1) | (carry << 7);
                self.set_c(val & 0x01 != 0);
                self.set_zn(self.a);
            }
            // BVC nn
            0x70 => self.branch(opaddr, (self.p & Self::FLAG_V) != 0),
            // SEI
            0x78 => self.p |= Self::FLAG_I,

            // STA <mem> opcodes
            0x81 | 0x85 | 0x8d | 0x91 | 0x95 | 0x99 | 0x9d => {
                self.write(nes, opaddr, self.a);
            }

            // STY <mem> opcodes
            0x84 | 0x8c | 0x94 => self.write(nes, opaddr, self.y),
            // STX <mem> opcodes
            0x86 | 0x8e | 0x96 => self.write(nes, opaddr, self.x),

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
            0x90 => self.branch(opaddr, (self.p & Self::FLAG_C) == 0),

            // TYA
            0x98 => {
                self.a = self.y;
                self.set_zn(self.a);
            }
            // TXS
            0x9a => self.sp = self.x,

            // LDY <mem> opcodes
            0xa0 | 0xa4 | 0xac | 0xb4 | 0xbc => {
                self.y = self.read(nes, opaddr);
                self.set_zn(self.y);
            }
            // LDX <mem> opcodes
            0xa2 | 0xa6 | 0xae | 0xb6 | 0xbe => {
                self.x = self.read(nes, opaddr);
                self.set_zn(self.x);
            }
            // LDA <mem> opcodes
            0xA1 | 0xA5 | 0xA9 | 0xAd | 0xB1 | 0xB5 | 0xB9 | 0xBd => {
                self.a = self.read(nes, opaddr);
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
            0xb0 => self.branch(opaddr, (self.p & Self::FLAG_C) != 0),
            // CLV
            0xB8 => self.p &= !Self::FLAG_V,
            // TSX
            0xba => {
                self.x = self.sp;
                self.set_zn(self.x);
            }

            // CPY opcodes
            0xc0 | 0xc4 | 0xcc => self.compare(self.y, self.read(nes, opaddr)),
            // CMP opcodes
            0xc1 | 0xc5 | 0xc9 | 0xcd | 0xd1 | 0xd5 | 0xd9 | 0xdd => {
                self.compare(self.a, self.read(nes, opaddr))
            }
            // DEC <mem> opcodes
            0xc6 | 0xce | 0xd6 | 0xde => {
                let val = self.read(nes, opaddr).wrapping_sub(1);
                self.write(nes, opaddr, val);
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
            0xd0 => self.branch(opaddr, (self.p & Self::FLAG_Z) == 0),

            // CLD
            0xd8 => self.p &= !Self::FLAG_D,
            // CPX opcodes
            0xe0 | 0xe4 | 0xec => self.compare(self.x, self.read(nes, opaddr)),

            // SBC <mem> opcodes
            0xe1 | 0xe5 | 0xe9 | 0xed | 0xf1 | 0xf5 | 0xf9 | 0xfd => {
                let a = self.a;
                let b = self.read(nes, opaddr);
                let ncarry = if (self.p & Self::FLAG_C) != 0 {
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
                let val = self.read(nes, opaddr).wrapping_add(1);
                self.write(nes, opaddr, val);
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
            0xf0 => self.branch(opaddr, (self.p & Self::FLAG_Z) != 0),
            //SED
            0xf8 => self.p |= Self::FLAG_D,

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
