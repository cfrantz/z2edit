use anyhow::Result;
use indexmap::IndexMap;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use python_gui::{AudioOut, Color, Image};
use serde::{Deserialize, Serialize};

use crate::apu::Apu;
use crate::controller::Controllers;
use crate::cpu::Cpu6502;
use crate::mapper;
use crate::peripheral::{Mapper, Peripheral};
use crate::ppu::Ppu;
use crate::ram::{Ram, RamKind};
use crate::stall::Stall;
use crate::{Address, AddressRange, NesFile};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct NesState {
    pub cpu: Cpu6502,
    pub apu: Apu,
    pub ppu: Ppu,
    pub ram: Ram,
    pub vram: Ram,
    pub pram: Ram,
    pub mapper: Box<dyn Mapper>,
    pub controllers: Controllers,
    pub stall: Stall,
}

#[pyclass(sequence)]
pub struct Nes {
    pub rom: Arc<Mutex<NesFile>>,
    pub cpu: Arc<Mutex<Cpu6502>>,
    pub apu: Arc<Mutex<Apu>>,
    pub ppu: Arc<Mutex<Ppu>>,
    pub ram: Arc<Mutex<Ram>>,
    pub vram: Arc<Mutex<Ram>>,
    pub pram: Arc<Mutex<Ram>>,
    pub mapper: Arc<Mutex<Box<dyn Mapper>>>,
    pub controllers: Arc<Mutex<Controllers>>,
    pub image: Arc<Mutex<Image>>,
    pub stall: Stall,
    pub audio: Arc<Mutex<IndexMap<String, Vec<f32>>>>,
    pub volume: AtomicI32,
    pub name: Arc<Mutex<String>>,
    pub pause: AtomicBool,
    pub frame_step: AtomicBool,
    pub frame_lock: AtomicBool,
    pub frame: AtomicI64,
    pub remainder: AtomicI64,
    peripherals: Vec<(AddressRange, Arc<Mutex<dyn Peripheral>>)>,

    pub(crate) read_cb: Arc<Mutex<IndexMap<u16, PyObject>>>,
    pub(crate) write_cb: Arc<Mutex<IndexMap<u16, PyObject>>>,
    pub(crate) exec_cb: Arc<Mutex<IndexMap<Address, PyObject>>>,

    pub trace: AtomicBool,
    pub tracebuf: Arc<Mutex<IndexMap<Address, u64>>>,
}

impl Nes {
    pub const FREQUENCY: u32 = 1789773;
    pub const SAMPLE_RATE: u32 = 48000;
    pub const FPS: f64 = 60.0998;

    fn register_peripheral(&mut self, addr_range: AddressRange, p: Arc<Mutex<dyn Peripheral>>) {
        self.peripherals.push((addr_range, p));
    }

    fn register_peripherals(&mut self) -> Result<()> {
        // Although RAM is only 2K, it decodes in the first 4K of address space.
        self.register_peripheral(
            AddressRange::cpu(0, 0x1000),
            Arc::clone(&self.ram) as Arc<Mutex<dyn Peripheral>>,
        );

        // The PPU has only 8 registers, but it's mirrored from 0x2000 to 0x3FFF.
        self.register_peripheral(
            AddressRange::cpu(0x2000, 0x2000),
            Arc::clone(&self.ppu) as Arc<Mutex<dyn Peripheral>>,
        );
        // The OAM DMA register is located at 0x4014.  Although this register is
        // part of the CPU/APU part, we model it as part of the PPU in this emulator.
        self.register_peripheral(
            AddressRange::cpu(0x4014, 1),
            Arc::clone(&self.ppu) as Arc<Mutex<dyn Peripheral>>,
        );
        // The PPU has a built-in RAM for mapping 2-bit color values into the
        // NES's total 64 possible colors.  Although this RAM is internal to the
        // PPU, we model it as a separate RAM here.
        self.register_peripheral(
            AddressRange::ppu(0x3F00, 256),
            Arc::clone(&self.pram) as Arc<Mutex<dyn Peripheral>>,
        );

        // The APU decodes from 0x4000-0x4013, 0x4015 and 0x4017.  On the NES, the
        // APU is integrated into the same chip as the CPU and has exact decodes
        // for these registers (ie: no mirroring).
        self.register_peripheral(
            AddressRange::cpu(0x4000, 0x14),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral>>,
        );
        self.register_peripheral(
            AddressRange::cpu(0x4015, 0x01),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral>>,
        );
        self.register_peripheral(
            AddressRange::cpu(0x4017, 0x01),
            Arc::clone(&self.apu) as Arc<Mutex<dyn Peripheral>>,
        );

        // The controllers decode at 0x4016 and 0x4017.
        self.register_peripheral(
            AddressRange::cpu(0x4016, 2),
            Arc::clone(&self.controllers) as Arc<Mutex<dyn Peripheral>>,
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
                Arc::clone(&self.mapper) as Arc<Mutex<dyn Peripheral>>,
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
        if self.frame_lock.load(Ordering::Relaxed) {
            audio.play(buf)?;
        } else {
            audio.try_play(buf)?;
        }
        Ok(())
    }
    pub fn read(&self, addr: Address) -> u8 {
        match addr {
            Address::Cpu(_) | Address::Ppu(_) => {
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
            Address::File(_)
            | Address::Prg(_, _)
            | Address::Prg8k(_, _)
            | Address::Chr(_, _)
            | Address::Chr1k(_, _) => self
                .rom
                .lock()
                .expect("Failed to lock rom for read")
                .read(addr)
                .unwrap_or(0xff),
            Address::NullPtr() => 0xff,
        }
    }

    pub fn write(&self, addr: Address, value: u8) {
        match addr {
            Address::Cpu(_) | Address::Ppu(_) => {
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
            Address::File(_)
            | Address::Prg(_, _)
            | Address::Prg8k(_, _)
            | Address::Chr(_, _)
            | Address::Chr1k(_, _) => {
                if self
                    .rom
                    .lock()
                    .expect("Failed to lock rom for write")
                    .write(addr, value)
                    .is_err()
                {
                    log::debug!("Writing {addr:x?} value {value:02x} failed.");
                }
            }
            Address::NullPtr() => {
                log::debug!("Writing {addr:x?} value {value:02x} is nonsense.");
            }
        }
    }

    fn extract_address(py: Python<'_>, addr: PyObject) -> PyResult<Address> {
        if let Ok(a) = addr.extract::<Address>(py) {
            Ok(a)
        } else if let Ok(a) = addr.extract::<u16>(py) {
            Ok(Address::Cpu(a))
        } else {
            Err(PyTypeError::new_err("unknown address type"))
        }
    }

    fn trace_cycles(&self, pc: Address, cycles: u64) {
        let mut tracebuf = self.tracebuf.lock().unwrap();
        if let Some(time) = tracebuf.get_mut(&pc) {
            *time += cycles;
        } else {
            tracebuf.insert(pc, cycles);
        }
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
            mapper: Arc::new(Mutex::new(mapper)),
            controllers: Arc::new(Mutex::new(Controllers::new())),
            image: Arc::new(Mutex::new(Image::with_color(
                256,
                240,
                Color::new(0xFF999999),
            ))),
            stall: Stall::default(),
            audio: Arc::default(),
            volume: AtomicI32::new(1 << 24),
            pause: AtomicBool::default(),
            frame_step: AtomicBool::default(),
            frame_lock: AtomicBool::new(true),
            frame: AtomicI64::default(),
            remainder: AtomicI64::default(),
            name: Arc::new(Mutex::new(String::default())),
            peripherals: Vec::default(),
            read_cb: Arc::default(),
            write_cb: Arc::default(),
            exec_cb: Arc::default(),
            trace: AtomicBool::default(),
            tracebuf: Arc::default(),
        };
        nes.register_peripherals()?;
        Ok(Py::new(py, nes)?)
    }

    pub fn reset(&self) {
        *self.cpu.lock().unwrap() = Cpu6502::default();
        *self.apu.lock().unwrap() = Apu::new();
        *self.ppu.lock().unwrap() = Ppu::new();
        *self.controllers.lock().unwrap() = Controllers::new();
        *self.mapper.lock().unwrap() =
            mapper::new(&self.rom.lock().unwrap()).expect("failed to create mapper");
        *self.image.lock().unwrap() = Image::with_color(256, 240, Color::new(0xFF999999));
        let _ = self.stall.clear(false);
        self.frame.store(0, Ordering::Relaxed);
        self.remainder.store(0, Ordering::Relaxed);
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

    #[setter]
    pub fn set_name(&self, name: &str) {
        let mut n = self.name.lock().unwrap();
        *n = name.to_string();
    }

    #[getter]
    pub fn get_name(&self) -> String {
        let n = self.name.lock().unwrap();
        n.clone()
    }

    #[setter]
    pub fn set_pause(&self, pause: bool) {
        self.pause.store(pause, Ordering::Relaxed);
    }

    #[getter]
    pub fn get_pause(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    #[getter]
    pub fn get_frame(&self) -> i64 {
        self.frame.load(Ordering::Relaxed)
    }

    #[setter]
    pub fn set_frame_step(&self, frame_step: bool) {
        self.frame_step.store(frame_step, Ordering::Relaxed);
    }

    #[getter]
    pub fn get_frame_step(&self) -> bool {
        self.frame_step.load(Ordering::Relaxed)
    }

    #[setter]
    pub fn set_frame_lock(&self, frame_lock: bool) {
        self.frame_lock.store(frame_lock, Ordering::Relaxed);
    }

    #[getter]
    pub fn get_frame_lock(&self) -> bool {
        self.frame_lock.load(Ordering::Relaxed)
    }

    #[setter]
    pub fn set_trace(&self, trace: bool) {
        self.trace.store(trace, Ordering::Relaxed);
    }

    #[getter]
    pub fn get_trace(&self) -> bool {
        self.trace.load(Ordering::Relaxed)
    }

    #[setter]
    pub fn set_tracebuf(&self, tracebuf: IndexMap<Address, u64>) {
        *self.tracebuf.lock().unwrap() = tracebuf;
    }

    #[getter]
    pub fn get_tracebuf(&self) -> IndexMap<Address, u64> {
        self.tracebuf.lock().unwrap().clone()
    }

    pub fn save_state(&self) -> NesState {
        NesState {
            cpu: self.cpu.lock().unwrap().clone(),
            apu: self.apu.lock().unwrap().clone(),
            ppu: self.ppu.lock().unwrap().clone(),
            ram: self.ram.lock().unwrap().clone(),
            vram: self.vram.lock().unwrap().clone(),
            pram: self.pram.lock().unwrap().clone(),
            mapper: self.mapper.lock().unwrap().clone(),
            controllers: self.controllers.lock().unwrap().clone(),
            stall: self.stall.clone(),
        }
    }

    pub fn restore_state(&self, state: &NesState) {
        self.cpu.lock().unwrap().clone_from(&state.cpu);
        self.apu.lock().unwrap().clone_from(&state.apu);
        self.ppu.lock().unwrap().clone_from(&state.ppu);
        self.ram.lock().unwrap().clone_from(&state.ram);
        self.vram.lock().unwrap().clone_from(&state.vram);
        self.pram.lock().unwrap().clone_from(&state.pram);
        *self.mapper.lock().unwrap() = state.mapper.clone();
        self.controllers
            .lock()
            .unwrap()
            .clone_from(&state.controllers);
        self.stall.clone_from(&state.stall);
    }

    pub fn emulate_frame(&self, audio: &AudioOut) -> bool {
        let pause = self.pause.load(Ordering::Relaxed);
        let frame_step = self.frame_step.load(Ordering::Relaxed);
        if pause {
            if !frame_step {
                return false;
            }
            self.frame_step.store(false, Ordering::Relaxed);
        }

        if self.audio_ready() {
            let _ = self.audio_play(audio);
        }

        let start = self.cpu.lock().unwrap().cycles as i64 * 2;
        let eof = start + 59561 + self.remainder.load(Ordering::Relaxed);
        loop {
            let cycles = self.cpu.lock().unwrap().cycles as i64 * 2;
            if cycles >= eof {
                let _f = self.frame.fetch_add(1, Ordering::Relaxed);
                //log::info!(
                //    "Frame {_f} ended on cycle {} ({}, {})",
                //    cycles / 2,
                //    (cycles - start) / 2,
                //    eof - cycles
                //);
                self.remainder.store(eof - cycles, Ordering::Relaxed);
                break;
            }
            if !self.tick() {
                return false;
            }
        }
        true
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
                //log::info!("          {}", cpu.cpustate());
                //log::info!("{:<10}{op:<30}", cpu.cycles);

                let pc = self.mapper.lock().unwrap().cpu_to_address(cpu.pc);
                let exec_cb = self.exec_cb.lock().expect("failed to lock exec_cb");
                if let Some(callback) = exec_cb.get(&pc) {
                    Python::with_gil(|py| {
                        match callback
                            .call1(py, (cpu.clone(),))
                            .and_then(|val| val.extract::<Cpu6502>(py))
                        {
                            Ok(val) => *cpu = val,
                            Err(e) => {
                                log::error!("Exec callback for {pc:x?} failed: {e}");
                            }
                        }
                    });
                }
                let n = cpu.execute(self);
                if self.trace.load(Ordering::Relaxed) {
                    self.trace_cycles(pc, n);
                }
                n
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

    pub fn controller_set(&self, index: usize, value: u8) {
        let mut controllers = self.controllers.lock().unwrap();
        if let Some(ctrl) = controllers.controller.get_mut(index) {
            ctrl.set(value);
        }
    }

    pub fn controller_clear(&self, index: usize, value: u8) {
        let mut controllers = self.controllers.lock().unwrap();
        if let Some(ctrl) = controllers.controller.get_mut(index) {
            ctrl.clear(value);
        }
    }

    pub fn controller_value(&self, index: usize, value: u8) {
        let mut controllers = self.controllers.lock().unwrap();
        if let Some(ctrl) = controllers.controller.get_mut(index) {
            ctrl.buttons = value;
        }
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

    #[pyo3(name = "read")]
    pub fn _read<'p>(&self, py: Python<'p>, addr: PyObject) -> PyResult<u8> {
        let addr = Self::extract_address(py, addr)?;
        Ok(self.read(addr))
    }

    pub fn read_i8<'p>(&self, py: Python<'p>, addr: PyObject) -> PyResult<i8> {
        let addr = Self::extract_address(py, addr)?;
        Ok(self.read(addr) as i8)
    }

    pub fn read_u16<'p>(&self, py: Python<'p>, addr: PyObject, hiaddr: PyObject) -> PyResult<u16> {
        let addr = Self::extract_address(py, addr)?;
        let hiaddr = if hiaddr.is_none(py) {
            addr + 1
        } else {
            Self::extract_address(py, hiaddr)?
        };
        Ok(u16::from_le_bytes([self.read(addr), self.read(hiaddr)]))
    }

    #[pyo3(name = "write")]
    pub fn _write<'p>(&self, py: Python<'p>, addr: PyObject, val: PyObject) -> PyResult<()> {
        let addr = Self::extract_address(py, addr)?;
        if let Ok(val) = val.extract::<u8>(py) {
            self.write(addr, val);
            Ok(())
        } else if let Ok(val) = val.extract::<Vec<u8>>(py) {
            for (i, &v) in val.iter().enumerate() {
                self.write(addr + i, v);
            }
            Ok(())
        } else {
            Err(PyTypeError::new_err("bad data type"))
        }
    }

    pub fn write_u16<'p>(
        &self,
        py: Python<'p>,
        addr: PyObject,
        hiaddr: PyObject,
        val: u16,
    ) -> PyResult<()> {
        let addr = Self::extract_address(py, addr)?;
        let hiaddr = if hiaddr.is_none(py) {
            addr + 1
        } else {
            Self::extract_address(py, hiaddr)?
        };
        let [lo, hi] = val.to_le_bytes();
        self.write(addr, lo);
        self.write(hiaddr, hi);
        Ok(())
    }

    fn __len__(&self) -> usize {
        65536
    }
    fn __getitem__<'p>(&self, py: Python<'p>, addr: PyObject) -> PyResult<u8> {
        self._read(py, addr)
    }
    fn __setitem__<'p>(&self, py: Python<'p>, addr: PyObject, val: u8) -> PyResult<()> {
        let addr = Self::extract_address(py, addr)?;
        self.write(addr, val);
        Ok(())
    }

    pub fn set_read_callback<'p>(
        &self,
        py: Python<'p>,
        addr: u16,
        callback: PyObject,
    ) -> PyResult<()> {
        //let addr = Self::extract_address(py, addr)?;
        let mut read_cb = self.read_cb.lock().expect("failed to lock read_cb");
        if callback.is_none(py) {
            read_cb.shift_remove(&addr);
        } else {
            read_cb.insert(addr, callback);
        }
        Ok(())
    }

    pub fn set_write_callback<'p>(
        &self,
        py: Python<'p>,
        addr: u16,
        callback: PyObject,
    ) -> PyResult<()> {
        //let addr = Self::extract_address(py, addr)?;
        let mut write_cb = self.write_cb.lock().expect("failed to lock write_cb");
        if callback.is_none(py) {
            write_cb.shift_remove(&addr);
        } else {
            write_cb.insert(addr, callback);
        }
        Ok(())
    }

    pub fn set_exec_callback<'p>(
        &self,
        py: Python<'p>,
        addr: Address,
        callback: PyObject,
    ) -> PyResult<()> {
        //let addr = Self::extract_address(py, addr)?;
        let mut exec_cb = self.exec_cb.lock().expect("failed to lock exec_cb");
        if callback.is_none(py) {
            exec_cb.shift_remove(&addr);
        } else {
            exec_cb.insert(addr, callback);
        }
        Ok(())
    }

    /// Return the number of 16K PRG banks in the NES ROM.
    #[getter]
    pub fn rom_prg_banks(&self) -> usize {
        self.rom.lock().unwrap().prg_banks()
    }

    /// Return the number of 8K CHR banks in the NES ROM.
    #[getter]
    pub fn rom_chr_banks(&self) -> usize {
        self.rom.lock().unwrap().chr_banks()
    }

    /// Return the mapper used by the NES ROM.
    #[getter]
    pub fn rom_mapper(&self) -> u16 {
        self.rom.lock().unwrap().mapper()
    }

    /// Return the mirror mode in the header.
    /// - false: vertical arrangement (mirrored horizontally) or mapper-controlled.
    /// - true: horizontal arrangement (mirrored vertically).
    #[getter]
    pub fn rom_mirror(&self) -> bool {
        self.rom.lock().unwrap().mirror()
    }

    /// Return whether the cart uses four-screen arrangement.
    #[getter]
    pub fn rom_fourscreen(&self) -> bool {
        self.rom.lock().unwrap().fourscreen()
    }

    /// Return whether the cart has a battery or other non-volatile memory.
    #[getter]
    pub fn rom_battery(&self) -> bool {
        self.rom.lock().unwrap().battery()
    }

    /// Get the PRG address of a raw CPU address.
    fn cpu_to_address(&self, cpu: u16) -> Address {
        self.mapper.lock().unwrap().cpu_to_address(cpu)
    }
}
