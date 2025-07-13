use std::any::Any;

use crate::{Address, AddressRange, Nes};

pub trait Peripheral: Send + Sync {
    fn write(&mut self, nes: &Nes, address: Address, value: u8);
    fn read(&mut self, nes: &Nes, address: Address) -> u8; // PPU read can have side effects
    fn tick(&mut self, nes: &Nes);
    fn decode_address(&self) -> Vec<AddressRange> {
        Vec::default()
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Mapper: Peripheral {
    fn clone(&self) -> Box<dyn Mapper>;
}
