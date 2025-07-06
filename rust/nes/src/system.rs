use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use python_gui::{AudioOut, Color, Image};

use crate::apu::Apu;
use crate::controller::Controllers;
use crate::cpu::Cpu6502;
use crate::mapper;
use crate::peripheral::Peripheral;
use crate::ppu::Ppu;
use crate::ram::{Ram, RamKind};
use crate::stall::Stall;
use crate::{Address, AddressRange, NesFile};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct Nes {
    pub rom: Arc<Mutex<NesFile>>,
    pub cpu: Arc<Mutex<Cpu6502>>,
    pub apu: Arc<Mutex<Apu>>,
    pub ppu: Arc<Mutex<Ppu>>,
    pub ram: Arc<Mutex<Ram>>,
    pub vram: Arc<Mutex<Ram>>,
    pub pram: Arc<Mutex<Ram>>,
    pub mapper: Arc<Mutex<Box<dyn Peripheral + Send + Sync>>>,
    pub controllers: Arc<Mutex<Controllers>>,
    pub image: Arc<Mutex<Image>>,
    pub stall: Stall,
    pub audio: Arc<Mutex<IndexMap<String, Vec<f32>>>>,
    pub volume: AtomicI32,
    peripherals: Vec<(AddressRange, Arc<Mutex<dyn Peripheral + Send + Sync>>)>,
}

impl Nes {
    pub const FREQUENCY: u32 = 1789773;
    pub const SAMPLE_RATE: u32 = 48000;
    pub const FPS: f64 = 60.0998;

    fn register_peripheral(
        &mut self,
        addr_range: AddressRange,
        p: Arc<Mutex<dyn Peripheral + Send + Sync>>,
    ) {
        self.peripherals.push((addr_range, p));
    }

    fn register_peripherals(&mut self) -> Result<()> {
        // Although RAM is only 2K, it decodes in the first 4K of address space.
        self.register_peripheral(
            AddressRange::cpu(0, 0x1000),
            Arc::clone(&self.ram) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );

        // The PPU has only 8 registers, but it's mirrored from 0x2000 to 0x3FFF.
        self.register_peripheral(
            AddressRange::cpu(0x2000, 0x2000),
            Arc::clone(&self.ppu) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );
        // The OAM DMA register is located at 0x4014.  Although this register is
        // part of the CPU/APU part, we model it as part of the PPU in this emulator.
        self.register_peripheral(
            AddressRange::cpu(0x4014, 1),
            Arc::clone(&self.ppu) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );
        // The PPU has a built-in RAM for mapping 2-bit color values into the
        // NES's total 64 possible colors.  Although this RAM is internal to the
        // PPU, we model it as a separate RAM here.
        self.register_peripheral(
            AddressRange::ppu(0x3F00, 256),
            Arc::clone(&self.pram) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );

        // The APU decodes from 0x4000-0x4013, 0x4015 and 0x4017.  On the NES, the
        // APU is integrated into the same chip as the CPU and has exact decodes
        // for these registers (ie: no mirroring).
        self.register_peripheral(
            AddressRange::cpu(0x4000, 0x14),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );
        self.register_peripheral(
            AddressRange::cpu(0x4015, 0x01),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );
        self.register_peripheral(
            AddressRange::cpu(0x4017, 0x01),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );

        // The controllers decode at 0x4016 and 0x4017.
        self.register_peripheral(
            AddressRange::cpu(0x4016, 2),
            Arc::clone(&self.controllers) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
        );

        // Query the mapper's address ranges and register them.
        let mapper_ranges = self
            .mapper
            .lock()
            .expect("Failed to lock mapper for decode")
            .decode_address();
        for range in mapper_ranges {
            self.register_peripheral(
                range,
                Arc::clone(&self.mapper) as Arc<Mutex<dyn Peripheral + Send + Sync>>,
            );
        }
        Ok(())
    }

    fn audio_ready(&self) -> bool {
        let data = self.audio.lock().expect("Failed to lock audio");
        if let Some((_, samples)) = data.get_index(0) {
            samples.len() >= 1024
        } else {
            false
        }
    }

    fn audio_play(&self, audio: &AudioOut) -> Result<()> {
        let mut buf = vec![0.0; 1024];
        let mut data = self.audio.lock().expect("Failed to lock audio");
        let volume = self.get_volume();
        for channel in data.values_mut() {
            for (i, sample) in channel.drain(0..1024).enumerate() {
                buf[i] += sample * volume;
            }
        }
        audio.play(buf)?;
        Ok(())
    }
}

#[pymethods]
impl Nes {
    #[new]
    pub fn new(py: Python<'_>, rom: NesFile) -> Result<Py<Self>> {
        rom.log_header();
        let mapper = mapper::new(&rom)?;
        let mut nes = Nes {
            rom: Arc::new(Mutex::new(rom)),
            cpu: Arc::new(Mutex::new(Cpu6502::default())),
            apu: Arc::new(Mutex::new(Apu::new())),
            ppu: Arc::new(Mutex::new(Ppu::new())),
            ram: Arc::new(Mutex::new(Ram::new(RamKind::Ram, 2048)?)),
            vram: Arc::new(Mutex::new(Ram::new(RamKind::VRam, 2048)?)),
            pram: Arc::new(Mutex::new(Ram::new(RamKind::PaletteRam, 32)?)),
            mapper,
            controllers: Arc::new(Mutex::new(Controllers::new())),
            image: Arc::new(Mutex::new(Image::new(256, 240))),
            stall: Stall::default(),
            audio: Arc::default(),
            volume: AtomicI32::new(1 << 24),
            peripherals: Vec::default(),
        };
        nes.register_peripherals()?;
        Ok(Py::new(py, nes)?)
    }

    #[setter]
    pub fn set_volume(&self, volume: f32) {
        let volume = (volume * 16777216.0) as i32;
        self.volume.store(volume, Ordering::Relaxed);
    }

    #[getter]
    pub fn get_volume(&self) -> f32 {
        let volume = self.volume.load(Ordering::Relaxed);
        volume as f32 / 16777216.0
    }

    pub fn emulate_frame(&self, audio: &AudioOut) {
        let initial_frame = self.ppu.lock().expect("Failed to lock PPU").frame;
        while self.ppu.lock().expect("Failed to lock PPU").frame == initial_frame {
            if !self.tick() {
                break;
            }
        }
        if self.audio_ready() {
            let _ = self.audio_play(audio);
        }
    }

    pub fn tick(&self) -> bool {
        let n = {
            let mut cpu = self.cpu.lock().expect("Failed to lock CPU");
            if self.stall.stalling() {
                let n = self.stall.clear(cpu.cycles % 2 == 1);
                cpu.cycles += n;
                n
            } else {
                //let (op, _) = Cpu6502::disassemble(self, cpu.pc);
                //log::info!("{op:<30} {}", cpu.cpustate());
                cpu.execute(self)
            }
        };
        for _ in 0..n {
            self.apu.lock().expect("Failed to lock APU").tick(self);
            for _ in 0..3 {
                self.ppu.lock().expect("Failed to lock PPU").tick(self);
                self.mapper
                    .lock()
                    .expect("Failed to lock mapper")
                    .tick(self);
            }
        }
        n > 0
    }

    pub fn signal_nmi(&self) {
        self.cpu.lock().expect("Failed to lock CPU").signal_nmi();
    }
    pub fn signal_irq(&self) {
        self.cpu.lock().expect("Failed to lock CPU").signal_irq();
    }
    pub fn dma_stall(&self, cycles: u64, odd_cycle: bool) {
        self.stall.stall(cycles, odd_cycle);
    }
    pub fn set_pixel(&self, x: u32, y: u32, color: u32) {
        self.image
            .lock()
            .expect("Failed to lock image")
            .set_pixel(x, y, Color::new(color));
    }
    pub fn audio_sample(&self, what: &str, sample: f32) {
        let mut data = self.audio.lock().expect("Failed to lock audio");
        if let Some(samples) = data.get_mut(what) {
            samples.push(sample);
        } else {
            let mut samples = Vec::with_capacity(1024);
            samples.push(sample);
            data.insert(what.into(), Vec::with_capacity(1024));
        }
    }

    pub fn read(&self, addr: Address) -> u8 {
        let mut result = 0xff;
        let mut decode = 0;
        //log::debug!("read {addr:x?}.");
        for (range, peripheral_arc) in self.peripherals.iter() {
            if range.contains_addr(addr) {
                decode += 1;
                let mut peripheral = peripheral_arc
                    .lock()
                    .expect("Failed to lock peripheral for read");
                result &= peripheral.read(self, addr);
            }
        }
        if decode == 0 {
            log::debug!("Reading {addr:x?} had no peripheral decode (open bus).");
        }
        result
    }

    pub fn write(&self, addr: Address, value: u8) {
        let mut decode = 0;
        //log::debug!("write {addr:x?} {value:02x}.");
        for (range, peripheral_arc) in self.peripherals.iter() {
            if range.contains_addr(addr) {
                decode += 1;
                let mut peripheral = peripheral_arc
                    .lock()
                    .expect("Failed to lock peripheral for write");
                peripheral.write(self, addr, value);
            }
        }
        if decode == 0 {
            log::debug!("Writing {addr:x?} value {value:02x} had no peripheral decode.");
        }
    }
}
