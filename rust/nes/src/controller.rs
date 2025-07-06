use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::peripheral::Peripheral;
use crate::system::Nes;
use crate::Address;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Controller {
    pub buttons: u8,
    index: u8,
    strobe: u8,
}

impl Controller {
    pub const BUTTON_A: u8 = 0x01;
    pub const BUTTON_B: u8 = 0x02;
    pub const BUTTON_SELECT: u8 = 0x04;
    pub const BUTTON_START: u8 = 0x08;
    pub const BUTTON_UP: u8 = 0x10;
    pub const BUTTON_DOWN: u8 = 0x20;
    pub const BUTTON_LEFT: u8 = 0x40;
    pub const BUTTON_RIGHT: u8 = 0x80;
}

impl Controller {
    fn read(&mut self) -> u8 {
        let ret = if self.index < 8 {
            (self.buttons >> self.index) & 1
        } else {
            0
        };
        self.index += 1;
        if (self.strobe & 1) != 0 {
            self.index = 0;
        }
        ret
    }

    fn write(&mut self, val: u8) {
        self.strobe = val;
        if (self.strobe & 1) != 0 {
            self.index = 0;
        }
    }

    #[inline]
    pub fn set(&mut self, val: u8) {
        self.buttons |= val;
    }

    #[inline]
    pub fn clear(&mut self, val: u8) {
        self.buttons &= !val;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Controllers {
    pub controller: Vec<Controller>,
}

impl Default for Controllers {
    fn default() -> Self {
        Controllers {
            controller: vec![Controller::default(), Controller::default()],
        }
    }
}

impl Controllers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Peripheral for Controllers {
    fn read(&mut self, _nes: &Nes, address: Address) -> u8 {
        match address {
            Address::Cpu(0x4016) => self.controller[0].read(),
            Address::Cpu(0x4017) => self.controller[1].read(),
            _ => 0xFF,
        }
    }

    fn write(&mut self, _nes: &Nes, address: Address, val: u8) {
        match address {
            Address::Cpu(0x4016) => {
                for c in self.controller.iter_mut() {
                    c.write(val);
                }
            }
            _ => {}
        }
    }

    fn tick(&mut self, _nes: &Nes) {
        // Controllers typically don't have a clock tick in the same way
        // other APU/PPU components do. Their state changes based on input.
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
