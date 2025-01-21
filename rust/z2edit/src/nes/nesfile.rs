use anyhow::{Context, Result};
use pyo3::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::nes::{Address, NesError};

#[pyclass]
#[derive(Default, Clone)]
pub struct NesFile {
    data: Vec<u8>,
}

impl std::fmt::Debug for NesFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NesFile({})", self.sha256())
    }
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
    /// Load a NES ROM.
    fn _load(path: &str) -> Result<Self> {
        Self::load(path)
    }

    #[pyo3(name = "save")]
    /// Save the NES ROM.
    fn _save(&self, path: &str) -> Result<()> {
        self.save(path)
    }

    /// Return the number of 16K PRG banks in the NES ROM.
    pub fn prg_banks(&self) -> usize {
        self.data[4] as usize
    }

    /// Return the number of 8K CHR banks in the NES ROM.
    pub fn chr_banks(&self) -> usize {
        self.data[5] as usize
    }

    /// Return the mapper used by the NES ROM.
    pub fn mapper(&self) -> u16 {
        (self.data[6] >> 4 | self.data[7] & 0xF0) as u16
    }

    /// Insert data into the NES ROM.
    /// This function will cause the ROM to grow.  Be sure to adjust
    /// the header appropriately.
    pub fn insert(&mut self, address: Address, data: &[u8]) -> Result<()> {
        let offset = self.offset(address)?;
        for (i, v) in data.iter().enumerate() {
            self.data.insert(offset + i, *v);
        }
        Ok(())
    }

    /// Read a slice of bytes from the NES ROM.
    pub fn read_bytes(&self, address: Address, length: usize) -> Result<&[u8]> {
        let start = self.offset(address)?;
        let end = start + length;
        self.data
            .get(start..end)
            .ok_or(NesError::InvalidAddress)
            .with_context(|| format!("reading {start}..{end}"))
    }

    /// Write a slice of bytes into the NES ROM.
    pub fn write_bytes(&mut self, address: Address, value: &[u8]) -> Result<()> {
        let offset = self.offset(address)?;
        for (i, v) in value.iter().enumerate() {
            self.data[offset + i] = *v;
        }
        Ok(())
    }

    /// Read a byte from the NES ROM.
    pub fn read(&self, address: Address) -> Result<u8> {
        let byte = self.read_bytes(address, 1)?;
        Ok(byte[0])
    }

    /// Read a word (u16) from the NES ROM.
    pub fn read_word(&self, address: Address) -> Result<u16> {
        let byte = self.read_bytes(address, 2)?;
        Ok(byte[0] as u16 | (byte[1] as u16) << 8)
    }

    /// Read a pointer (u16) from the NES ROM.
    /// It is assumed the pointer points into the same bank as the source address.
    pub fn read_pointer(&self, address: Address) -> Result<Address> {
        let val = self.read_word(address)?;
        Ok(address.with_offset(val as usize))
    }

    /// Read a variable sized slice terminted by the given terminator.
    /// The slice will not include the terminator.
    pub fn read_terminated(&self, address: Address, terminator: u8) -> Result<&[u8]> {
        let start = self.offset(address)?;
        let mut len = 0;
        while self.data[start + len] != terminator {
            len += 1;
        }
        self.read_bytes(address, len)
    }

    /// Write a byte into the NES ROM.
    pub fn write(&mut self, address: Address, value: u8) -> Result<()> {
        self.write_bytes(address, &[value])
    }

    /// Write a word (u16) into the NES ROM.
    pub fn write_word(&mut self, address: Address, value: u16) -> Result<()> {
        let value = value.to_le_bytes();
        self.write_bytes(address, &value)
    }

    /// Write a word (u16) into the NES ROM.
    /// It is assumed the pointer points into the same bank as the source address.
    pub fn write_pointer(&mut self, address: Address, value: Address) -> Result<()> {
        self.write_word(address, value.offset() as u16)
    }

    /// Write a variable sized slice terminted by the given terminator.
    /// The slice does not include the terminator.
    pub fn write_terminated(
        &mut self,
        address: Address,
        value: &[u8],
        terminator: u8,
    ) -> Result<()> {
        self.write_bytes(address, value)?;
        self.write(address + value.len(), terminator)?;
        Ok(())
    }

    pub fn sha256(&self) -> String {
        hex::encode(Sha256::digest(&self.data))
    }
}
