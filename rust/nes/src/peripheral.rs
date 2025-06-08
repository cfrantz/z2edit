use crate::{Address, AddressRange, Nes};

pub(crate) trait Peripheral {
    fn write(&mut self, nes: &Nes, address: Address, value: u8);
    fn read(&mut self, nes: &Nes, address: Address) -> u8; // PPU read can have side effects
    fn tick(&mut self, nes: &Nes);
    fn decode_address(&self) -> Vec<AddressRange> {
        Vec::default()
    }
}
