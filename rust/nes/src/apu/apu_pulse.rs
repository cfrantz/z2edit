use crate::system::Nes;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registers {
    pub control: u8,
    pub sweep: u8,
    pub timer_lo: u8,
    pub timer_hi: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApuPulse {
    pub channel: u8,

    enabled: bool,
    length_enabled: bool,
    length_value: u8,

    timer_period: u16,
    timer_value: u16,

    duty_mode: usize,
    duty_value: usize,

    sweep_enable: bool,
    sweep_reload: bool,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_period: u8,
    sweep_value: u8,

    envelope_enable: bool,
    envelope_start: bool,
    envelope_loop: bool,
    envelope_period: u8,
    envelope_value: u8,
    envelope_volume: u8,

    constant_volume: u8,
    pub reg: Registers,
    pub channel_volume: f32,
    pub debug_buf: Vec<f32>,
    pub debug_idx: usize,
}

impl ApuPulse {
    const DEBUG_BUF_SZ: usize = Nes::SAMPLE_RATE as usize / (Nes::FPS as usize);
    pub fn new(channel: u8) -> Self {
        ApuPulse {
            channel: channel,
            channel_volume: 0.25,
            debug_buf: vec![0.0; Self::DEBUG_BUF_SZ],
            ..Default::default()
        }
    }
    pub fn active(&self) -> bool {
        self.length_value != 0
    }
    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
        if !self.enabled {
            self.length_value = 0;
        }
    }
    pub fn set_control(&mut self, val: u8) {
        self.reg.control = val;
        self.duty_mode = (val >> 6) as usize;
        self.length_enabled = (val & 0x20) == 0;
        self.envelope_loop = (val & 0x20) != 0;
        self.envelope_enable = (val & 0x10) == 0;
        self.envelope_period = val & 0x0F;
        self.constant_volume = val & 0x0F;
        self.envelope_start = true;
    }
    pub fn set_sweep(&mut self, val: u8) {
        self.reg.sweep = val;
        self.sweep_enable = (val & 0x80) != 0;
        self.sweep_period = (val >> 4) & 0x7;
        self.sweep_negate = (val & 0x08) != 0;
        self.sweep_shift = val & 0x07;
        self.sweep_reload = true;
    }
    pub fn set_timer_low(&mut self, val: u8) {
        self.reg.timer_lo = val;
        self.timer_period = (self.timer_period & 0xFF00) | (val as u16);
    }
    pub fn set_timer_high(&mut self, val: u8) {
        self.reg.timer_hi = val;
        self.length_value = LENGTH_TABLE[(val >> 3) as usize];
        self.timer_period = (self.timer_period & 0x00FF) | ((val & 0x07) as u16) << 8;
        self.envelope_start = true;
        self.duty_value = 0;
    }

    pub fn output(&mut self) -> f32 {
        let data = self.internal_output() as f32 / 15.0;
        self.debug_buf[self.debug_idx] = data;
        self.debug_idx = (self.debug_idx + 1) % self.debug_buf.len();
        data * self.channel_volume
    }

    fn internal_output(&self) -> u8 {
        if self.enabled == false
            || self.length_value == 0
            || DUTY_TABLE[self.duty_mode][self.duty_value] == 0
            || self.timer_period < 8
            || self.timer_period > 0x7ff
        {
            0
        } else if self.envelope_enable {
            self.envelope_volume
        } else {
            self.constant_volume
        }
    }

    pub fn sweep(&mut self) {
        let delta = self.timer_period >> self.sweep_shift;
        if self.sweep_negate {
            self.timer_period -= delta;
            if self.channel == 1 {
                self.timer_period -= 1;
            }
        } else {
            self.timer_period += delta;
        }
    }
    pub fn step_timer(&mut self) {
        if self.timer_value == 0 {
            self.timer_value = self.timer_period;
            self.duty_value = (self.duty_value + 1) % 8;
        } else {
            self.timer_value -= 1;
        }
    }
    pub fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_volume = 15;
            self.envelope_value = self.envelope_period;
            self.envelope_start = false;
        } else if self.envelope_value > 0 {
            self.envelope_value -= 1;
        } else {
            if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            } else if self.envelope_loop {
                self.envelope_volume = 15;
            }
            self.envelope_value = self.envelope_period;
        }
    }
    pub fn step_sweep(&mut self) {
        if self.sweep_reload {
            if self.sweep_enable && self.sweep_value == 0 {
                self.sweep();
            }
            self.sweep_value = self.sweep_period;
            self.sweep_reload = false;
        } else if self.sweep_value > 0 {
            self.sweep_value -= 1;
        } else {
            if self.sweep_enable {
                self.sweep();
            }
            self.sweep_value = self.sweep_period;
        }
    }
    pub fn step_length(&mut self) {
        if self.length_enabled && self.length_value > 0 {
            self.length_value -= 1;
        }
    }

    pub fn write<'py>(&mut self, _nes: &Nes, addr: u16, val: u8) {
        match addr & 3 {
            0 => self.set_control(val),
            1 => self.set_sweep(val),
            2 => self.set_timer_low(val),
            3 => self.set_timer_high(val),
            _ => {}
        }
    }
}
