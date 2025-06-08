use crate::error::NesError;
use crate::peripheral::Peripheral;
use crate::NesFile;
use crate::{Address, AddressRange, Nes};
use anyhow::Result;
use std::sync::{Arc, Mutex};

mod cnrom;
mod mmc1;
mod mmc3;
mod uxrom;

pub fn new(rom: &NesFile) -> Result<Arc<Mutex<Box<dyn Peripheral + Send + Sync>>>> {
    match rom.mapper() {
        0 | 2 => Ok(Arc::new(Mutex::new(Box::new(uxrom::UxROM::new(rom)?)))),
        1 => Ok(Arc::new(Mutex::new(Box::new(mmc1::MMC1::new(rom)?)))),
        3 => Ok(Arc::new(Mutex::new(Box::new(cnrom::CNROM::new(rom)?)))),
        4 => Ok(Arc::new(Mutex::new(Box::new(mmc3::MMC3::new(rom)?)))),
        _ => Err(NesError::UnsupportedMapper(rom.mapper()).into()),
    }
}

impl Peripheral for Box<dyn Peripheral + Send + Sync> {
    fn write(&mut self, nes: &Nes, address: Address, value: u8) {
        (**self).write(nes, address, value)
    }
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        (**self).read(nes, address)
    }
    fn tick(&mut self, nes: &Nes) {
        (**self).tick(nes)
    }
    fn decode_address(&self) -> Vec<AddressRange> {
        (**self).decode_address()
    }
}

pub(crate) fn simple_mirror_address(mode: u8, address: u16) -> u16 {
    let address = address & 0xFFF;
    match mode {
        0 => {
            let a11 = address & 0x800;
            (address & !0xC00) | (a11 >> 1)
        }
        1 => address & !0x800,
        2 => address & !0xC00,
        3 => (address & !0xC00) | 0x400,
        4 => address,
        _ => {
            panic!("Unknown mirror mode {}", mode);
        }
    }
}
