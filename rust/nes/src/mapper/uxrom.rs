use serde::{Serialize, Deserialize};
use anyhow::Result;
use pyo3::prelude::*;

use crate::ram::{RamKind, Ram};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Debug, Serialize, Deserialize)]
#[pyclass]
pub struct UxROM {
    prg_banks: u8,
    prg_bank1: u8,
    prg_bank2: u8,
    wram: Ram,
}

#[pymethods]
impl UxROM {
    #[new]
    pub fn new<'py>(py: Python<'py>, rom: &NesFile) -> Result<Py<Self>> {
        let banks = u8::try_from(rom.prg_banks())?;
        Ok(Py::new(py, Self {
            prg_banks: banks,
            prg_bank1: 0,
            prg_bank2: banks - 1,
            wram: Ram::new(RamKind::WRam, 8192)?,
        })?)
    }

    pub fn tick<'py>(&mut self, _nes: &Bound<'py, Nes>) {
    }

    pub fn read<'py>(&mut self, nes_bound: &Bound<'py, Nes>, address: Address) -> u8 {
        let py = nes_bound.py();
        let nes = nes_bound.borrow();
        let rom = nes.rom.borrow(py);
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.read(nes_bound, Address::Cpu(addr & 0x1FFF)),
                0x8000..=0xBFFF => {
                    let a = Address::Prg(self.prg_bank1 as i16, addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("UxROM failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                },
                0xC000..=0xFFFF => {
                    let a = Address::Prg(self.prg_bank2 as i16, addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("UxROM failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                },
                _ => {
                    log::warn!("UxROM: unhandled CPU read: address={:04x}", addr);
                    0
                }
            }
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
                },
                0x2000..=0x3FFF => {
                    let addr = {
                        rom.mirror_address(addr)
                    };
                    nes.vram.borrow_mut(py).read(nes_bound, Address::Ppu(addr))
                }
                _ => {
                    log::warn!("UxROM: unhandled PPU read: address={:04x}", addr);
                    0
                }
            }
            _ => { log::warn!("UxROM: unhandled read for address space: {:?}", address); 0 }
        }
    }

    fn write<'py>(&mut self, nes_bound: &Bound<'py, Nes>, address: Address, val: u8) {
        let py = nes_bound.py();
        let nes = nes_bound.borrow();
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.write(nes_bound, Address::Cpu(addr & 0x1FFF), val),
                0x8000..=0xFFFF => {
                    self.prg_bank1 = val;
                },
                _ => {
                    log::warn!("UxROM: unhandled CPU write address={:04x} value={:02x}", addr, val);
                }
            }
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For UxROM with CHR RAM, this would write to CHR RAM.
                    // Assuming NesFile's write handles CHR RAM if present.
                    nes.rom.borrow_mut(py).write(Address::Chr(0, addr), val).ok();
                },
                0x2000..=0x3FFF => {
                    let addr = {
                        let rom = nes.rom.borrow(py);
                        rom.mirror_address(addr)
                    };
                    nes.vram.borrow_mut(py).write(nes_bound, Address::Ppu(addr), val);
                }
                _ => {
                    log::warn!("UxROM: unhandled PPU write address={:04x} value={:02x}", addr, val);
                }
            }
            _ => { log::warn!("UxROM: unhandled write for address space: {:?} value {:02x}", address, val); }
        }
    }

    fn decode_address(&self) -> Vec<AddressRange> {
        vec![
            AddressRange::cpu(0x6000, 0xA000),
            AddressRange::ppu(0x0000, 0x3f00),
        ]
    }
}
