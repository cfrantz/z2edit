mod apu_dmc;
mod apu_noise;
mod apu_pulse;
mod apu_triangle;

use crate::Address;
use crate::apu::apu_dmc::ApuDmc;
use crate::apu::apu_noise::ApuNoise;
use crate::apu::apu_pulse::ApuPulse;
use crate::apu::apu_triangle::ApuTriangle;
use crate::system::Nes;
use serde::{Deserialize, Serialize};
use std::default::Default;
use pyo3::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass]
pub struct Apu {
    cycle: u64,
    frame_period: u8,
    frame_value: u8,
    frame_irq: bool,

    pub pulse0: ApuPulse,
    pub pulse1: ApuPulse,
    pub triangle: ApuTriangle,
    pub noise: ApuNoise,
    pub dmc: ApuDmc,
}

impl Apu {
    const FRAME_COUNTER_RATE: f64 = (Nes::FREQUENCY as f64) / 240.0;
    //const SAMPLE_RATE: f64 = (Nes::FREQUENCY as f64) / (Nes::SAMPLE_RATE as f64);
    const SAMPLE_RATE: f64 = ((Nes::FREQUENCY as f64) / (Nes::FPS as f64)) / (48000.0/59.9);
    pub fn new() -> Self {
        Apu {
            pulse0: ApuPulse::new(0),
            pulse1: ApuPulse::new(1),
            triangle: ApuTriangle::new(),
            noise: ApuNoise::new(),
            dmc: ApuDmc::new(),
            ..Default::default()
        }
    }
    fn step_timer<'py>(&mut self, nes: &Bound<'py, Nes>) {
        if self.cycle % 2 == 0 {
            self.pulse0.step_timer();
            self.pulse1.step_timer();
            self.noise.step_timer();
            self.dmc.step_timer(nes);
        }
        self.triangle.step_timer();
    }
    fn step_envelope(&mut self) {
        self.pulse0.step_envelope();
        self.pulse1.step_envelope();
        self.triangle.step_counter();
        self.noise.step_envelope();
    }
    fn step_sweep(&mut self) {
        self.pulse0.step_sweep();
        self.pulse1.step_sweep();
    }
    fn step_length(&mut self) {
        self.pulse0.step_length();
        self.pulse1.step_length();
        self.triangle.step_length();
        self.noise.step_length();
    }
    fn step_frame_counter<'py>(&mut self, nes: &Bound<'py, Nes>) {
        if self.frame_period == 4 {
            self.frame_value = (self.frame_value + 1) % 4;
            self.step_envelope();
            if (self.frame_value & 1) == 1 {
                self.step_sweep();
                self.step_length();
                if self.frame_value == 3 && self.frame_irq {
                    Nes::signal_irq(nes);
                }
            }
        } else {
            self.frame_value = (self.frame_value + 1) % 5;
            if self.frame_value != 4 {
                self.step_envelope();
                if (self.frame_value & 1) == 0 {
                    self.step_sweep();
                    self.step_length();
                }
            }
        }
    }


    fn set_frame_counter(&mut self, val: u8) {
        self.frame_period = 4 + (val >> 7);
        self.frame_irq = (val & 0x40) == 0;
    }
    fn set_control(&mut self, val: u8) {
        self.pulse0.set_enabled((val & 0x01) != 0);
        self.pulse1.set_enabled((val & 0x02) != 0);
        self.triangle.set_enabled((val & 0x04) != 0);
        self.noise.set_enabled((val & 0x08) != 0);
        self.dmc.set_enabled((val & 0x10) != 0);
    }
}

#[pymethods]
impl Apu {
    pub fn write<'py>(&mut self, nes: &Bound<'py, Nes>, address: Address, val: u8) {
        let Address::Cpu(address) = address else {
            return;
        };
        match address {
            0x4000..=0x4003 => self.pulse0.write(nes, address, val),
            0x4004..=0x4007 => self.pulse1.write(nes, address, val),
            0x4008..=0x400b => self.triangle.write(nes, address, val),
            0x400c..=0x400f => self.noise.write(nes, address, val),
            0x4010..=0x4013 => self.dmc.write(nes, address, val),
            0x4015 => self.set_control(val),
            0x4017 => self.set_frame_counter(val),
            _ => {}
        }
    }

    pub fn read<'py>(&mut self, _nes: &Bound<'py, Nes>, address: Address) -> u8 {
        let Address::Cpu(address) = address else {
            return 0xFF;
        };
        match address {
            0x4015 => {
                0x00 | if self.pulse0.active() { 0x01 } else { 0x00 }
                    | if self.pulse1.active() { 0x02 } else { 0x00 }
                    | if self.triangle.active() { 0x04 } else { 0x00 }
                    | if self.noise.active() { 0x08 } else { 0x00 }
                    | if self.dmc.active() { 0x10 } else { 0x00 }
            }
            _ => 0,
        }
    }

    pub fn tick<'py>(&mut self, nes: &Bound<'py, Nes>) {
        let c1 = self.cycle as f64;
        self.cycle += 1;
        let c2 = self.cycle as f64;

        self.step_timer(nes);

        let f1 = (c1 / Self::FRAME_COUNTER_RATE) as usize;
        let f2 = (c2 / Self::FRAME_COUNTER_RATE) as usize;
        if f1 != f2 {
            self.step_frame_counter(nes);
        }

        let s1 = (c1 / Self::SAMPLE_RATE) as usize;
        let s2 = (c2 / Self::SAMPLE_RATE) as usize;
        if s1 != s2 {
            Nes::audio_sample(nes, "APU: Pulse 0", self.pulse0.output());
            Nes::audio_sample(nes, "APU: Pulse 1", self.pulse1.output());
            Nes::audio_sample(nes, "APU: Triangle", self.triangle.output());
            Nes::audio_sample(nes, "APU: Noise", self.noise.output());
            Nes::audio_sample(nes, "APU: DMC", self.dmc.output());
        }
    }
}
