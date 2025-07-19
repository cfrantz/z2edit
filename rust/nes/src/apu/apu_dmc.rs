use serde::{Deserialize, Serialize};

use crate::system::Nes;
use crate::Address;

const DMC_TABLE: [u8; 16] = [
    214, 190, 170, 160, 143, 127, 113, 107, 95, 80, 71, 64, 53, 42, 36, 27,
];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registers {
    pub control: u8,
    pub value: u8,
    pub address: u8,
    pub length: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApuDmc {
    enabled: bool,
    value: u8,

    sample_address: u16,
    sample_length: u16,
    current_address: u16,
    current_length: u16,

    shift_register: u8,
    bit_count: u8,
    tick_value: u8,
    tick_period: u8,

    loopit: bool,
    irq: bool,

    pub reg: Registers,
    pub channel_volume: f32,
    pub debug_buf: Vec<f32>,
    pub debug_idx: usize,
}

impl ApuDmc {
    const DEBUG_BUF_SZ: usize = Nes::SAMPLE_RATE as usize / (Nes::FPS as usize);
    pub fn new() -> Self {
        Self {
            channel_volume: 0.25,
            debug_buf: vec![0.0; Self::DEBUG_BUF_SZ],
            ..Default::default()
        }
    }
    pub fn active(&self) -> bool {
        self.current_length != 0
    }
    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
        if !self.enabled {
            self.current_length = 0;
        } else if self.current_length == 0 {
            self.restart();
        }
    }
    pub fn set_control(&mut self, val: u8) {
        self.reg.control = val;
        self.irq = (val & 0x80) != 0;
        self.loopit = (val & 0x40) != 0;
        self.tick_period = DMC_TABLE[(val & 0x0f) as usize];
    }
    pub fn set_value(&mut self, val: u8) {
        self.reg.value = val;
        self.value = val & 0x7f;
    }
    pub fn set_address(&mut self, val: u8) {
        self.reg.address = val;
        self.sample_address = 0xC000u16 | (val as u16) << 6;
    }
    pub fn set_length(&mut self, val: u8) {
        self.reg.length = val;
        self.sample_length = 1u16 | (val as u16) << 4;
    }

    pub fn output(&mut self) -> f32 {
        let data = self.value as f32 / 127.0;
        self.debug_buf[self.debug_idx] = data;
        self.debug_idx = (self.debug_idx + 1) % self.debug_buf.len();
        data * self.channel_volume
    }

    pub fn step_timer<'py>(&mut self, nes: &Nes) {
        if self.enabled {
            self.step_reader(nes);
            if self.tick_value == 0 {
                self.tick_value = self.tick_period;
                self.step_shifter();
            } else {
                self.tick_value -= 1;
            }
        }
    }
    pub fn step_reader<'py>(&mut self, nes: &Nes) {
        if self.current_length > 0 && self.bit_count == 0 {
            nes.dma_stall(4, false);
            self.shift_register = nes.read(Address::Cpu(self.current_address));
            self.bit_count = 8;
            self.current_address = self.current_address.wrapping_add(1);
            if self.current_address == 0 {
                self.current_address = 0x8000;
            }
            self.current_length -= 1;
            if self.current_length == 0 && self.loopit {
                self.restart();
            }
        }
    }
    pub fn step_shifter(&mut self) {
        if self.bit_count != 0 {
            if (self.shift_register & 1) == 1 {
                if self.value <= 125 {
                    self.value += 2;
                }
            } else if self.value >= 2 {
                self.value -= 2;
            }
            self.shift_register >>= 1;
            self.bit_count -= 1;
        }
    }
    fn restart(&mut self) {
        self.current_address = self.sample_address;
        self.current_length = self.sample_length;
    }

    pub fn write<'py>(&mut self, _nes: &Nes, addr: u16, val: u8) {
        match addr & 3 {
            0 => self.set_control(val),
            1 => self.set_value(val),
            2 => self.set_address(val),
            3 => self.set_length(val),
            _ => {}
        }
    }
}
