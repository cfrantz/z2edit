use anyhow::{anyhow, ensure, Result};
use python_gui::Directories;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

use crate::peripheral::Peripheral;
use crate::system::Nes;
use crate::Address;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RamKind {
    Ram,
    PaletteRam,
    VRam,
    WRam,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Ram {
    kind: RamKind,
    cycle: u64,
    data: Vec<u8>,
}

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ram")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

impl Ram {
    pub const SAVE_FREQUENCY: u64 = 3 * Nes::FREQUENCY as u64;
    fn validate_address(&self, addr: Address) -> Result<usize> {
        match addr {
            Address::Cpu(x) => Ok(x as usize),
            Address::Ppu(x) => Ok(x as usize),
            _ => Err(anyhow!(
                "Address {addr:x?} not supported for RamKind::{:?}",
                self.kind
            )),
        }
    }

    pub fn new(kind: RamKind, size: usize) -> Result<Self> {
        ensure!(size.is_power_of_two(), "RAM size must be a power of two");
        Ok(Ram {
            kind,
            cycle: 0,
            data: vec![0u8; size],
        })
    }
}

impl Peripheral for Ram {
    fn read(&mut self, _nes: &Nes, address: Address) -> u8 {
        match self.validate_address(address) {
            Ok(addr) => {
                let mask = self.data.len() - 1;
                let mut addr = addr & mask;
                if self.kind == RamKind::PaletteRam && addr >= 16 && addr % 4 == 0 {
                    addr -= 16;
                }
                self.data[addr]
            }
            Err(e) => {
                log::error!("{e}");
                0xFF
            }
        }
    }

    fn write(&mut self, _nes: &Nes, address: Address, val: u8) {
        match self.validate_address(address) {
            Ok(addr) => {
                let mask = self.data.len() - 1;
                let mut addr = addr & mask;
                if self.kind == RamKind::PaletteRam && addr >= 16 && addr % 4 == 0 {
                    addr -= 16;
                }
                self.data[addr] = val;
            }
            Err(e) => {
                log::error!("{e}");
            }
        }
    }

    fn tick(&mut self, nes: &Nes) {
        if self.kind == RamKind::WRam && self.cycle % Self::SAVE_FREQUENCY == 0 {
            let rom = nes.rom.lock().expect("failed to lock rom");
            if rom.battery() {
                let name = nes.name.lock().expect("failed to lock filename");
                let datadir = Directories::get().data_dir.display();
                let name = format!("{datadir}/{name}.sram");
                if self.cycle == 0 {
                    if let Err(e) = self.load(&name) {
                        log::error!("Failed to load SRAM file {name}: {e}");
                    }
                } else {
                    if let Err(e) = self.save(&name) {
                        log::error!("Failed to save SRAM file {name}: {e}");
                    }
                }
            }
        }
        self.cycle += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Ram {
    pub fn load(&mut self, path: &str) -> Result<()> {
        let mut f = File::open(path)?;
        f.read_exact(&mut self.data)?;
        Ok(())
    }

    fn save(&mut self, path: &str) -> Result<()> {
        let mut f = File::create(path)?;
        f.write_all(&self.data)?;
        Ok(())
    }
}
