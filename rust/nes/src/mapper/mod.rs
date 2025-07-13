use anyhow::Result;
use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::error::NesError;
use crate::peripheral::{Mapper, Peripheral};
use crate::NesFile;
use crate::{Address, AddressRange, Nes};

pub mod cnrom;
pub mod mmc1;
pub mod mmc3;
pub mod mmc5;
pub mod uxrom;
pub mod vrc7;
mod vrc7_audio;

pub fn new(rom: &NesFile) -> Result<Arc<Mutex<Box<dyn Mapper>>>> {
    match rom.mapper() {
        0 | 2 => Ok(Arc::new(Mutex::new(Box::new(uxrom::UxROM::new(rom)?)))),
        1 => Ok(Arc::new(Mutex::new(Box::new(mmc1::MMC1::new(rom)?)))),
        3 => Ok(Arc::new(Mutex::new(Box::new(cnrom::CNROM::new(rom)?)))),
        4 => Ok(Arc::new(Mutex::new(Box::new(mmc3::MMC3::new(rom)?)))),
        5 => Ok(Arc::new(Mutex::new(Box::new(mmc5::MMC5::new(rom)?)))),
        85 => Ok(Arc::new(Mutex::new(Box::new(vrc7::Vrc7::new(rom)?)))),
        _ => Err(NesError::UnsupportedMapper(rom.mapper()).into()),
    }
}

// We need this forwarding impl so that Box<dyn Mapper> can be coerced
// to dyn Peripheral for the peripheral memory map.
impl Peripheral for Box<dyn Mapper> {
    fn write(&mut self, nes: &Nes, address: Address, value: u8) {
        (**self).write(nes, address, value)
    }
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        (**self).read(nes, address)
    }
    fn tick(&mut self, nes: &Nes) {
        (**self).tick(nes)
    }
    fn as_any(&self) -> &dyn Any {
        (**self).as_any()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_mut()
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
