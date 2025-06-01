use anyhow::{anyhow, ensure, Result};
use std::fmt;
use std::io::{Read, Write};
use std::fs::File;
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;

use crate::{Address};
use crate::system::Nes;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[pyclass(eq, eq_int)]
pub enum RamKind {
    Ram,
    PaletteRam,
    VRam,
    WRam,
}

#[derive(Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Ram {
    kind: RamKind,
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
    fn validate_address(&self, addr: Address) -> Result<usize> {
        match addr {
            Address::Cpu(x) => Ok(x as usize),
            Address::Ppu(x) => Ok(x as usize),
            _ => Err(anyhow!("Address {addr:x?} not supporte for RamKind::{:?}", self.kind)),
        }
    }

    pub fn new(kind: RamKind, size: usize) -> Result<Self> {
        ensure!(size.is_power_of_two(), "RAM size must be a power of two");
        Ok(Ram {
            kind,
            data: vec![0u8; size],
        })
    }
}

#[pymethods]
impl Ram {
    #[new]
    pub fn py_new<'p>(py: Python<'p>, kind: RamKind, size: usize) -> Result<Py<Self>> {
        Ok(Py::new(py, Ram::new(kind, size)?)?)
    }

    pub fn read<'py>(&mut self, _nes: &Bound<'py, Nes>, address: Address) -> u8 {
        match self.validate_address(address) {
            Ok(addr) => {
                let mask  = self.data.len() - 1;
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

    pub fn write<'py>(&mut self, _nes: &Bound<'py, Nes>, address: Address, val: u8) {
        match self.validate_address(address) {
            Ok(addr) => {
                let mask  = self.data.len() - 1;
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

    pub fn tick<'py>(&mut self, _nes: &Bound<'py, Nes>) {
        // If the RamKind is WRAM and its battery backed, then every N
        // ticks, save it to disk.
    }

    fn load(&mut self, path: &str) -> Result<()> {
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
