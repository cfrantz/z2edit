use crate::error::NesError;
use crate::peripheral::Peripheral;
use crate::NesFile;
use crate::{Address, AddressRange, Nes};
use anyhow::Result;
use std::sync::{Arc, Mutex};

mod uxrom;

pub fn new(rom: &NesFile) -> Result<Arc<Mutex<Box<dyn Peripheral + Send + Sync>>>> {
    match rom.mapper() {
        0 => Ok(Arc::new(Mutex::new(Box::new(uxrom::UxROM::new(rom)?)))),
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
