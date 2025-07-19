use std::fmt::Write;

use crate::system::Nes;
use crate::Address;

#[derive(Clone, Debug, Default)]
pub struct MemoryDebug {
    pub visible: bool,
}

impl MemoryDebug {
    pub fn hexdump(nes: &Nes, address: Address, length: usize) -> String {
        let mut data = String::with_capacity(length as usize * 80 / 16);

        let mut i = 0;
        write!(&mut data, "\n{:04x}: ", address.offset() + i).unwrap();
        while i < length {
            if i > 0 && i % 16 == 0 {
                write!(&mut data, "\n{:04x}: ", address.offset() + i).unwrap();
            }
            let v = nes.read(address + i);
            write!(&mut data, " {v:02x}").unwrap();
            i = i + 1;
        }
        data
    }

    pub fn draw(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if !self.visible {
            return;
        }
        let mut visible = self.visible;
        ui.window("Memory").opened(&mut visible).build(|| {
            ui.text("Main NES RAM:");
            ui.text(Self::hexdump(nes, Address::Cpu(0), 2048));
            ui.text("Cartridge WRAM:");
            ui.text(Self::hexdump(nes, Address::Cpu(0x6000), 8192));
        });
        self.visible = visible;
    }
}
