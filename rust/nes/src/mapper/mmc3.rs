use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::mapper::simple_mirror_address;
use crate::peripheral::{Mapper, Peripheral};
use crate::ram::{Ram, RamKind};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMC3 {
    irq_enable: bool,
    register: u8,
    reload: u8,
    counter: u8,
    prg_mode: u8,
    chr_mode: u8,
    mirror_mode: u8,
    registers: [u8; 8],
    prg_offset: [Address; 4],
    chr_offset: [Address; 8],
    wram: Ram,
}

impl MMC3 {
    pub fn new(_rom: &NesFile) -> Result<Self> {
        Ok(Self {
            irq_enable: false,
            register: 0,
            reload: 0,
            counter: 0,
            prg_mode: 0,
            chr_mode: 0,
            mirror_mode: 0,
            registers: [0u8; 8],
            prg_offset: [
                Address::Prg8k(0, 0),
                Address::Prg8k(1, 0),
                Address::Prg8k(-2, 0),
                Address::Prg8k(-1, 0),
            ],
            chr_offset: [
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
            ],
            wram: Ram::new(RamKind::WRam, 8192)?,
        })
    }
}

#[typetag::serde(name = "MMC3")]
impl Mapper for MMC3 {
    fn clone(&self) -> Box<dyn Mapper> {
        Box::new(Clone::clone(self))
    }
}
impl MMC3 {
    fn write_bank_select(&mut self, val: u8) {
        self.prg_mode = (val >> 6) & 1;
        self.chr_mode = (val >> 7) & 1;
        self.register = val & 7;
    }

    fn write_mirror_mode(&mut self, val: u8) {
        self.mirror_mode = 1 - (val & 1);
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        if addr < 0xA000 {
            if (addr & 1) == 0 {
                self.write_bank_select(val);
            } else {
                self.registers[self.register as usize] = val;
            }
            self.update_offsets();
        } else if addr < 0xC000 {
            if (addr & 1) == 0 {
                self.write_mirror_mode(val);
            }
        } else if addr < 0xE000 {
            if (addr & 1) == 0 {
                self.reload = val;
            } else {
                self.counter = 0;
            }
        } else {
            self.irq_enable = (addr & 1) != 0;
        }
    }

    fn update_offsets(&mut self) {
        self.prg_offset = match self.prg_mode {
            0 => [
                Address::Prg8k(self.registers[6] as i16, 0),
                Address::Prg8k(self.registers[7] as i16, 0),
                Address::Prg8k(-2, 0),
                Address::Prg8k(-1, 0),
            ],
            1 => [
                Address::Prg8k(-2, 0),
                Address::Prg8k(self.registers[7] as i16, 0),
                Address::Prg8k(self.registers[6] as i16, 0),
                Address::Prg8k(-1, 0),
            ],
            _ => {
                panic!("MMC3: Bad prg_mode {}", self.prg_mode);
            }
        };
        self.chr_offset = match self.chr_mode {
            0 => [
                Address::Chr1k(self.registers[0] as i16 & 0xFE, 0),
                Address::Chr1k(self.registers[0] as i16 | 0x01, 0),
                Address::Chr1k(self.registers[1] as i16 & 0xFE, 0),
                Address::Chr1k(self.registers[1] as i16 | 0x01, 0),
                Address::Chr1k(self.registers[2] as i16, 0),
                Address::Chr1k(self.registers[3] as i16, 0),
                Address::Chr1k(self.registers[4] as i16, 0),
                Address::Chr1k(self.registers[5] as i16, 0),
            ],
            1 => [
                Address::Chr1k(self.registers[2] as i16, 0),
                Address::Chr1k(self.registers[3] as i16, 0),
                Address::Chr1k(self.registers[4] as i16, 0),
                Address::Chr1k(self.registers[5] as i16, 0),
                Address::Chr1k(self.registers[0] as i16 & 0xFE, 0),
                Address::Chr1k(self.registers[0] as i16 | 0x01, 0),
                Address::Chr1k(self.registers[1] as i16 & 0xFE, 0),
                Address::Chr1k(self.registers[1] as i16 | 0x01, 0),
            ],
            _ => {
                panic!("MMC3: Bad chr_mode {}", self.chr_mode);
            }
        }
    }

    fn mirror_address(&self, rom: &NesFile, address: u16) -> u16 {
        let mode = if rom.fourscreen() {
            4
        } else {
            self.mirror_mode
        };
        simple_mirror_address(mode, address)
    }
}

impl Peripheral for MMC3 {
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let rom = nes.rom.lock().expect("Failed to lock ROM for read");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.read(nes, Address::Cpu(addr & 0x1FFF)),
                0x8000..=0xFFFF => {
                    let address = (addr & 0x7FFF) as usize;
                    let bank = address / 0x2000;
                    let offset = address % 0x2000;
                    let a = self.prg_offset[bank] + offset;
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC3 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                _ => {
                    log::warn!("MMC3: unhandled CPU read: address={:04x}", addr);
                    0
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    let bank = (addr / 0x0400) as usize;
                    let offset = (addr % 0x0400) as usize;
                    let a = self.chr_offset[bank] + offset;
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC3 failed to read rom {a:x?}: {e}");
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
                    log::warn!("MMC3: unhandled PPU read: address={:04x}", addr);
                    0
                }
            },
            _ => {
                log::warn!("MMC3: unhandled read for address space: {:?}", address);
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
                    self.write_register(addr, val);
                }
                _ => {
                    log::warn!(
                        "MMC3: unhandled CPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For MMC3 with CHR RAM, this would write to CHR RAM.

                    let bank = (addr / 0x0400) as usize;
                    let offset = (addr % 0x0400) as usize;
                    rom.write(self.chr_offset[bank] + offset, val).ok();
                }
                0x2000..=0x3FFF => nes
                    .vram
                    .lock()
                    .expect("Failed to lock VRAM for write")
                    .write(nes, Address::Ppu(self.mirror_address(&rom, addr)), val),
                _ => {
                    log::warn!(
                        "MMC3: unhandled PPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            _ => {
                log::warn!(
                    "MMC3: unhandled write for address space: {:?} value {:02x}",
                    address,
                    val
                );
            }
        }
    }

    fn tick(&mut self, nes: &Nes) {
        self.wram.tick(nes);
        use crate::ppu::{MASK_SHOWBG, MASK_SHOWSPRITES};
        let ppu = nes.ppu.lock().expect("mmc3 failed to lock ppu");
        if ppu.cycle == 280
            && (ppu.scanline < 240 || ppu.scanline == 260)
            && (ppu.mask & (MASK_SHOWBG | MASK_SHOWSPRITES)) != 0
        {
            if self.counter == 0 {
                self.counter = self.reload;
            } else {
                self.counter -= 1;
                if self.counter == 0 && self.irq_enable {
                    nes.signal_irq();
                }
            }
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn decode_address(&self) -> Vec<AddressRange> {
        vec![
            AddressRange::cpu(0x6000, 0xA000),
            AddressRange::ppu(0x0000, 0x3f00),
        ]
    }
}
