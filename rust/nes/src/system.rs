use anyhow::Result;
use pyo3::prelude::*;
use python_gui::{Image, Color};
//use serde::de;

use crate::{Address, NesFile, AddressRange};
use crate::stall::Stall;
use crate::cpu::{Cpu6502};
use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::ram::{Ram, RamKind};
use crate::controller::Controllers;
use crate::mapper;

#[pyclass]
pub struct Nes {
    pub rom: Py<NesFile>,
    pub cpu: Py<Cpu6502>,
    pub apu: Py<Apu>,
    pub ppu: Py<Ppu>,
    pub ram: Py<Ram>,
    pub vram: Py<Ram>,
    pub pram: Py<Ram>,
    pub mapper: Py<PyAny>,
    pub controllers: Py<Controllers>,
    pub image: Py<Image>,
    pub stall: Py<Stall>,
    peripherals: Vec<(AddressRange, Py<PyAny>)>,
}

impl Nes {
    pub const FREQUENCY: u32 = 1789773;
    pub const SAMPLE_RATE: u32 = 48000;
    pub const FPS: f64 = 60.0998;
}

#[pymethods]
impl Nes {
    #[new]
    pub fn new<'p>(py: Python<'p>, rom: Py<NesFile>) -> Result<Py<Self>> {
        let brom = rom.borrow(py);
        let mut nes = Nes {
            rom: rom.clone_ref(py),
            cpu: Py::new(py, Cpu6502::default())?,
            apu: Py::new(py, Apu::new())?,
            ppu: Py::new(py, Ppu::new())?,
            ram: Ram::py_new(py, RamKind::Ram, 2048)?,
            vram: Ram::py_new(py, RamKind::VRam, 2048)?,
            pram: Ram::py_new(py, RamKind::PaletteRam, 32)?,
            mapper: mapper::new(py, &brom)?,
            controllers: Py::new(py, Controllers::default())?,
            image: Py::new(py, Image::new(256, 240))?,
            stall: Py::new(py, Stall::default())?,
            peripherals: Vec::default(),
        };
        nes.register_peripherals(py)?;
        Ok(Py::new(py, nes)?)
    }

    pub fn register_peripheral(&mut self, addr_range: AddressRange, p: Py<PyAny>) {
        self.peripherals.push((addr_range, p));
    }

    pub fn register_peripherals<'p>(&mut self, py: Python<'p>) -> Result<()> {
        // Although RAM is only 2K, it decodes in the first 4K of address space.
        self.register_peripheral(AddressRange::cpu(0, 0x1000), self.ram.as_any().clone_ref(py));

        // The PPU has only 8 registers, but it's mirrored from 0x2000 to 0x3FFF.
        self.register_peripheral(AddressRange::cpu(0x2000, 0x2000), self.ppu.as_any().clone_ref(py));
        // The OAM DMA register is located at 0x4014.  Although this register is
        // part of the CPU/APU part, we model it as part of the PPU in this emulator.
        self.register_peripheral(AddressRange::cpu(0x4014, 1), self.ppu.as_any().clone_ref(py));
        // The PPU has a built-in RAM for mapping 2-bit color values into the
        // NES's total 64 possible colors.  Although this RAM is internal to the
        // PPU, we model it as a separate RAM here.
        self.register_peripheral(AddressRange::ppu(0x3F00, 256), self.pram.as_any().clone_ref(py));

        // The APU decodes from 0x4000-0x4013, 0x4015 and 0x4017.  On the NES, the
        // APU is integrated into the same chip as the CPU and has exact decodes
        // for these registers (ie: no mirroring).
        self.register_peripheral(AddressRange::cpu(0x4000, 0x14), self.apu.as_any().clone_ref(py));
        self.register_peripheral(AddressRange::cpu(0x4015, 0x01), self.apu.as_any().clone_ref(py));
        self.register_peripheral(AddressRange::cpu(0x4017, 0x01), self.apu.as_any().clone_ref(py));

        // The controllers decode at 0x4016 and 0x4017.
        self.register_peripheral(AddressRange::cpu(0x4016, 2), self.controllers.as_any().clone_ref(py));

        // Query the mapper's address ranges and register them.
        let mapper_ranges = self.mapper.call_method0(py, "decode_address")?;
        let mapper_ranges = mapper_ranges.extract::<Vec<AddressRange>>(py)?;
        for range in mapper_ranges {
            self.register_peripheral(range, self.mapper.as_any().clone_ref(py));
        } 
        Ok(())
    }

    pub fn emulate_frame<'py>(self_: &Bound<'py, Self>) {
        let py = self_.py();
        let this = self_.borrow();
        let frame = this.ppu.borrow(py).frame;
        while this.ppu.borrow(py).frame == frame {
            if !Self::tick(self_) {
                break;
            }
        }
    }

    pub fn tick<'py>(self_: &Bound<'py, Self>) -> bool {
        let py = self_.py();
        let this = self_.borrow();

        let n = {
            let mut cpu = this.cpu.borrow_mut(py);
            if this.stall.borrow(py).stalling() {
                let mut stall = this.stall.borrow_mut(py);
                let n = cpu.cycles;
                if n%2 == 1 && stall.odd_cycle {
                    stall.cycles += 1;
                }
                cpu.cycles += stall.cycles;
                stall.clear()
            } else {
                //let (op, _) = Cpu6502::disassemble(self_, cpu.pc);
                //log::info!("{op}");
                cpu.execute(self_)
            }
        };
        for _ in 0..n {
            this.apu.borrow_mut(py).tick(self_);
            for _ in 0..3 {
                this.ppu.borrow_mut(py).tick(self_);
                match this.mapper.call_method1(py, "tick", (self_,)) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Mapper tick failed: {e:?}");
                    }
                }
            }
        }
        n > 0
    }

    pub fn signal_nmi<'py>(self_: &Bound<'py, Self>) {
        self_.borrow().cpu.borrow_mut(self_.py()).signal_nmi();
    }
    pub fn signal_irq<'py>(self_: &Bound<'py, Self>) {
        self_.borrow().cpu.borrow_mut(self_.py()).signal_irq();
    }
    pub fn dma_stall<'py>(self_: &Bound<'py, Self>, cycles: u64, odd_cycle: bool) {
        self_.borrow().stall.borrow_mut(self_.py()).stall(cycles, odd_cycle);
    }
    pub fn set_pixel<'py>(self_: &Bound<'py, Self>, x: u32, y: u32, color: u32) {
        self_.borrow().image.borrow_mut(self_.py()).set_pixel(x, y, Color::new(color));
    }
    pub fn audio_sample<'py>(_self: &Bound<'py, Self>, _what: &str, _sample: f32) {
        //todo!();
    }

    pub fn read<'py>(self_: &Bound<'py, Self>, addr: Address) -> u8 {
        let py = self_.py();
        let mut result = 0xff;
        let mut decode = 0;
        for (range, peripheral) in self_.borrow().peripherals.iter() {
            if range.contains_addr(addr) {
                decode += 1;
                match peripheral.call_method1(py, "read", (self_, addr)) {
                    Ok(val_obj) => match val_obj.extract::<u8>(py) {
                        Ok(value) => result &= value,
                        Err(e) => log::error!("Extracting u8 from peripheral read failed for {addr:x?}: {e:?}"),
                    },
                    Err(e) => {
                        log::error!("Reading {addr:x?} from peripheral failed: {e:?}");
                    }
                }
            }
        }
        if decode == 0 {
            log::debug!("Reading {addr:x?} had no peripheral decode (open bus).");
        }
        result
    }

    pub fn write<'py>(self_: &Bound<'py, Self>, addr: Address, value: u8) {
        let py = self_.py();
        let mut decode = 0;
        for (range, peripheral) in self_.borrow().peripherals.iter() {
            if range.contains_addr(addr) {
                decode += 1;
                match peripheral.call_method1(py, "write", (self_, addr, value)) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Writing {addr:x?} to peripheral failed: {e:?}");
                    }
                }
            }
        }
        if decode == 0 {
            log::debug!("Writing {addr:x?} value {value:02x} had no peripheral decode.");
        }
    }
}