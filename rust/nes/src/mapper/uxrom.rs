use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::peripheral::Peripheral;
use crate::ram::{Ram, RamKind};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Debug, Serialize, Deserialize)]
pub struct UxROM {
    prg_banks: u8,
    prg_bank1: u8,
    prg_bank2: u8,
    wram: Ram,
}

impl UxROM {
    pub fn new(rom: &NesFile) -> Result<Self> {
        let banks = u8::try_from(rom.prg_banks())?;
        Ok(Self {
            prg_banks: banks,
            prg_bank1: 0,
            prg_bank2: banks - 1,
            wram: Ram::new(RamKind::WRam, 8192)?,
        })
    }
}

impl Peripheral for UxROM {
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let rom = nes.rom.lock().expect("Failed to lock ROM for read");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.read(nes, Address::Cpu(addr & 0x1FFF)),
                0x8000..=0xBFFF => {
                    let a = Address::Prg(self.prg_bank1 as i16, addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("UxROM failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                0xC000..=0xFFFF => {
                    let a = Address::Prg(self.prg_bank2 as i16, addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("UxROM failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                _ => {
                    log::warn!("UxROM: unhandled CPU read: address={:04x}", addr);
                    0
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    let bank = addr / 0x1000;
                    let a = Address::Chr(bank as i16, addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("UxROM failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                0x2000..=0x3FFF => {
                    let mirrored_addr = rom.mirror_address(addr);
                    nes.vram
                        .lock()
                        .expect("Failed to lock VRAM for read")
                        .read(nes, Address::Ppu(mirrored_addr))
                }
                _ => {
                    log::warn!("UxROM: unhandled PPU read: address={:04x}", addr);
                    0
                }
            },
            _ => {
                log::warn!("UxROM: unhandled read for address space: {:?}", address);
                0
            }
        }
    }

    fn write(&mut self, nes: &Nes, address: Address, val: u8) {
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.write(nes, Address::Cpu(addr & 0x1FFF), val),
                0x8000..=0xFFFF => {
                    self.prg_bank1 = val;
                }
                _ => {
                    log::warn!(
                        "UxROM: unhandled CPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For UxROM with CHR RAM, this would write to CHR RAM.
                    nes.rom
                        .lock()
                        .expect("Failed to lock ROM for CHR write")
                        .write(Address::Chr(0, addr), val)
                        .ok();
                }
                0x2000..=0x3FFF => {
                    let mirrored_addr = nes
                        .rom
                        .lock()
                        .expect("Failed to lock ROM for mirror_address")
                        .mirror_address(addr);
                    nes.vram
                        .lock()
                        .expect("Failed to lock VRAM for write")
                        .write(nes, Address::Ppu(mirrored_addr), val);
                }
                _ => {
                    log::warn!(
                        "UxROM: unhandled PPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            _ => {
                log::warn!(
                    "UxROM: unhandled write for address space: {:?} value {:02x}",
                    address,
                    val
                );
            }
        }
    }

    fn tick(&mut self, nes: &Nes) {
        // UxROM typically doesn't have complex clock-based logic
        self.wram.tick(nes);
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
