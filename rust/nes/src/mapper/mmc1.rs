use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::mapper::simple_mirror_address;
use crate::peripheral::Peripheral;
use crate::ram::{Ram, RamKind};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Debug, Serialize, Deserialize)]
pub struct MMC1 {
    shift_register: u8,
    control: u8,
    prg_mode: u8,
    chr_mode: u8,
    prg_bank: u8,
    chr_bank: [u8; 2],
    prg_offset: [Address; 2],
    chr_offset: [Address; 2],
    wram: Ram,
}

impl MMC1 {
    pub fn new(rom: &NesFile) -> Result<Self> {
        let last_prg = Address::Prg(-1, 0);
        Ok(Self {
            shift_register: 0x10,
            control: 0,
            prg_mode: 0,
            chr_mode: 0,
            prg_bank: 0,
            chr_bank: [0, 0],
            prg_offset: [Address::Prg(0, 0), Address::Prg(-1, 0)],
            chr_offset: [Address::Chr(0, 0), Address::Chr(1, 0)],
            wram: Ram::new(RamKind::WRam, 8192)?,
        })
    }
}

impl MMC1 {
    fn load_register(&mut self, addr: u16, val: u8) {
        if val & 0x80 != 0 {
            self.shift_register = 0x10;
            self.write_control(self.control | 0x0C);
            self.update_offsets();
        } else {
            let complete = (self.shift_register & 1) != 0;
            self.shift_register = (self.shift_register >> 1) | ((val & 1) << 4);
            if complete {
                self.write_register(addr, self.shift_register);
                self.shift_register = 0x10;
            }
        }
    }
    fn write_register(&mut self, addr: u16, val: u8) {
        if addr < 0xA000 {
            self.write_control(val);
        } else if addr < 0xC000 {
            self.chr_bank[0] = val;
        } else if addr < 0xE000 {
            self.chr_bank[1] = val;
        } else {
            self.prg_bank = val & 0x0F;
        }
        self.update_offsets();
    }
    fn write_control(&mut self, val: u8) {
        self.control = val;
        self.chr_mode = (val >> 4) & 1;
        self.prg_mode = (val >> 2) & 3;
    }
    fn update_offsets(&mut self) {
        self.prg_offset = match self.prg_mode {
            0 | 1 => [
                Address::Prg(self.prg_bank as i16 & 0xFE, 0),
                Address::Prg(self.prg_bank as i16 | 0x01, 0),
            ],
            2 => [Address::Prg(0, 0), Address::Prg(self.prg_bank as i16, 0)],
            3 => [Address::Prg(self.prg_bank as i16, 0), Address::Prg(-1, 0)],
            _ => {
                panic!("MMC1: Bad prg_mode {}", self.prg_mode);
            }
        };
        self.chr_offset = match self.chr_mode {
            0 => [
                Address::Chr(self.chr_bank[0] as i16 & 0xFE, 0),
                Address::Chr(self.chr_bank[0] as i16 | 0x01, 0),
            ],
            1 => [
                Address::Chr(self.chr_bank[0] as i16, 0),
                Address::Chr(self.chr_bank[1] as i16, 0),
            ],
            _ => {
                panic!("MMC1: Bad chr_mode {}", self.chr_mode);
            }
        }
    }

    fn mirror_address(&self, rom: &NesFile, address: u16) -> u16 {
        let mode = if rom.fourscreen() {
            4
        } else {
            !self.control & 3
        };
        simple_mirror_address(mode, address)
    }
}

impl Peripheral for MMC1 {
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let rom = nes.rom.lock().expect("Failed to lock ROM for read");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.read(nes, Address::Cpu(addr & 0x1FFF)),
                0x8000..=0xFFFF => {
                    let address = (addr & 0x7FFF) as usize;
                    let bank = address / 0x4000;
                    let offset = address % 0x4000;
                    let a = self.prg_offset[bank] + offset;
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC1 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                _ => {
                    log::warn!("MMC1: unhandled CPU read: address={:04x}", addr);
                    0
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    let bank = (addr / 0x1000) as usize;
                    let offset = (addr % 0x1000) as usize;
                    let a = self.chr_offset[bank] + offset;
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC1 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                0x2000..=0x3FFF => nes
                    .vram
                    .lock()
                    .expect("Failed to lock VRAM for read")
                    .read(nes, Address::Ppu(self.mirror_address(&rom, addr))),
                _ => {
                    log::warn!("MMC1: unhandled PPU read: address={:04x}", addr);
                    0
                }
            },
            _ => {
                log::warn!("MMC1: unhandled read for address space: {:?}", address);
                0
            }
        }
    }

    fn write(&mut self, nes: &Nes, address: Address, val: u8) {
        let mut rom = nes.rom.lock().expect("Failed to lock ROM for write");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.write(nes, Address::Cpu(addr & 0x1FFF), val),
                0x8000..=0xFFFF => {
                    self.load_register(addr, val);
                }
                _ => {
                    log::warn!(
                        "MMC1: unhandled CPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For MMC1 with CHR RAM, this would write to CHR RAM.

                    let bank = (addr / 0x1000) as usize;
                    let offset = (addr % 0x1000) as usize;
                    rom.write(self.chr_offset[bank] + offset, val).ok();
                }
                0x2000..=0x3FFF => nes
                    .vram
                    .lock()
                    .expect("Failed to lock VRAM for write")
                    .write(nes, Address::Ppu(self.mirror_address(&rom, addr)), val),
                _ => {
                    log::warn!(
                        "MMC1: unhandled PPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            _ => {
                log::warn!(
                    "MMC1: unhandled write for address space: {:?} value {:02x}",
                    address,
                    val
                );
            }
        }
    }

    fn tick(&mut self, _nes: &Nes) {
        // MMC1 typically doesn't have complex clock-based logic
    }

    fn decode_address(&self) -> Vec<AddressRange> {
        vec![
            AddressRange::cpu(0x6000, 0xA000),
            AddressRange::ppu(0x0000, 0x3f00),
        ]
    }
}
