use anyhow::{Context, Result};
use pyo3::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::nes::{Address, NesError};

#[pyclass]
pub struct NesFile {
    data: Vec<u8>,
}

impl NesFile {
    const HEADER_SZ: usize = 16;
    pub fn from_reader(r: &mut impl Read) -> Result<Self> {
        let mut data = Vec::new();
        r.read_to_end(&mut data)?;
        Ok(Self { data })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        Self::from_reader(&mut file)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        std::fs::write(path, &self.data)?;
        Ok(())
    }

    fn _bank(b: i16, banks: usize) -> usize {
        if b < 0 {
            banks - (-b) as usize
        } else {
            b as usize
        }
    }

    fn offset(&self, addr: Address) -> Result<usize> {
        match addr {
            Address::File(x) => Ok(x),
            Address::Cpu(_) => Err(NesError::InvalidAddress.into()),
            Address::Prg(b, x) => {
                // Prg banks are 16K.
                let bank = Self::_bank(b, self.prg_banks());
                Ok(Self::HEADER_SZ + bank * 16384 + (x & 0x3FFF) as usize)
            }
            Address::Prg8k(b, x) => {
                // Prg8k banks are 8K.  Since the iNES header advertises the
                // number of 16k PRG banks, we double it for this calculation.
                let bank = Self::_bank(b, self.prg_banks() * 2);
                Ok(Self::HEADER_SZ + bank * 8192 + (x & 0x1FFF) as usize)
            }
            Address::Chr(b, x) => {
                // Chr banks are 4K.  Since the iNES header advertises the
                // number of 8k CHR banks, we double it for this calculation.
                // The CHR section starts after the HEADER and PRG sections.
                let bank = Self::_bank(b, self.chr_banks() * 2);
                let chrstart = Self::HEADER_SZ + self.prg_banks() * 16384;
                Ok(chrstart + bank * 4096 + (x & 0x0FFF) as usize)
            }
            Address::Chr1k(b, x) => {
                // Chr1k banks are 1K.  Since the iNES header advertises the
                // number of 8k CHR banks, we multiply by 8 for this calculation.
                // The CHR section starts after the HEADER and PRG sections.
                let bank = Self::_bank(b, self.chr_banks() * 8);
                let chrstart = Self::HEADER_SZ + self.prg_banks() * 16384;
                Ok(chrstart + bank * 1024 + (x & 0x03FF) as usize)
            }
        }
    }
}

#[pymethods]
impl NesFile {
    #[staticmethod]
    #[pyo3(name = "load")]
    fn _load(path: &str) -> Result<Self> {
        Self::load(path)
    }

    #[pyo3(name = "save")]
    fn _save(&self, path: &str) -> Result<()> {
        self.save(path)
    }

    pub fn prg_banks(&self) -> usize {
        self.data[4] as usize
    }

    pub fn chr_banks(&self) -> usize {
        self.data[5] as usize
    }

    pub fn mapper(&self) -> u16 {
        (self.data[6] >> 4 | self.data[7] & 0xF0) as u16
    }

    pub fn read_bytes(&self, address: Address, length: usize) -> Result<&[u8]> {
        let start = self.offset(address)?;
        let end = start + length;
        Ok(self
            .data
            .get(start..end)
            .ok_or(NesError::InvalidAddress)
            .with_context(|| format!("reading {start}..{end}"))?)
    }

    pub fn read(&self, address: Address) -> Result<u8> {
        let byte = self.read_bytes(address, 1)?;
        Ok(byte[0])
    }
}
