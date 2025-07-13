use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::mapper::simple_mirror_address;
use crate::mapper::vrc7_audio::Emu2413;
use crate::peripheral::{Mapper, Peripheral};
use crate::ram::{Ram, RamKind};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Clone, Serialize, Deserialize)]
pub struct OplDebug {
    pub volume: u8,
    pub flo: u8,
    pub fhi: u8,
    pub debug_buf: Vec<f32>,
    pub debug_idx: usize,
    pub channel_volume: f32,
}

impl Default for OplDebug {
    fn default() -> Self {
        OplDebug {
            volume: 0,
            flo: 0,
            fhi: 0,
            debug_buf: vec![0f32; 800],
            debug_idx: 0,
            channel_volume: 1.00,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vrc7 {
    cycles: u64,
    prg_banks: u8,
    chr_banks: u8,
    mirror: u8,
    prg_bank: [Address; 4],
    chr_bank: [Address; 8],
    irq_latch: u8,
    irq_control: u8,
    irq_counter: u16,
    cycle_counter: u16,
    opl_index: usize,
    pub(crate) opl_debug: [OplDebug; Self::VRC7_AUDIO_CHANNELS],
    #[serde(skip)]
    opll: Emu2413,

    wram: Ram,
}

impl Vrc7 {
    const VRC7_AUDIO_CHANNELS: usize = Emu2413::CHANNELS;
    const SAMPLE_RATE: f64 = ((3.0 * Nes::FREQUENCY as f64) / (Nes::FPS as f64)) / (48000.0 / 59.9);
    pub fn new(rom: &NesFile) -> Result<Self> {
        let banks = u8::try_from(rom.prg_banks())?;
        Ok(Self {
            cycles: 0,
            prg_banks: u8::try_from(rom.prg_banks())?,
            chr_banks: u8::try_from(rom.chr_banks())?,
            mirror: 0,
            prg_bank: [
                Address::Prg8k(0, 0),
                Address::Prg8k(0, 0),
                Address::Prg8k(0, 0),
                Address::Prg8k(-1, 0),
            ],
            chr_bank: [
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
                Address::Chr1k(0, 0),
            ],
            irq_latch: 0,
            irq_control: 0,
            irq_counter: 0,
            cycle_counter: 0,
            opl_index: 0,
            opl_debug: std::array::from_fn(|_| OplDebug::default()),
            opll: Emu2413::new(3579545, 48000),
            wram: Ram::new(RamKind::WRam, 8192)?,
        })
    }

    fn mirror_address(&self, rom: &NesFile, address: u16) -> u16 {
        let mode = if rom.fourscreen() {
            4
        } else {
            match self.mirror & 3 {
                0 => 1,
                1 => 0,
                2 => 2,
                3 => 3,
                _ => unreachable!(),
            }
        };
        simple_mirror_address(mode, address)
    }

    fn write_opl_register(&mut self, val: u8) {
        self.opll.write_reg(self.opl_index as u32, val);
        // Update some local copies of register values for creating debug displays.
        match self.opl_index {
            0x10..=0x1F => self.opl_debug[self.opl_index & 0x0F].flo = val,
            0x20..=0x2F => self.opl_debug[self.opl_index & 0x0F].fhi = val,
            0x30..=0x3F => self.opl_debug[self.opl_index & 0x0F].volume = val,
            _ => {}
        }
    }
}

impl Mapper for Vrc7 {
    fn clone(&self) -> Box<dyn Mapper> {
        Box::new(Clone::clone(self))
    }
}
impl Peripheral for Vrc7 {
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let rom = nes.rom.lock().expect("Failed to lock ROM for read");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => {
                    if self.mirror & 0x80 != 0 {
                        self.wram.read(nes, Address::Cpu(addr & 0x1FFF))
                    } else {
                        0xFF
                    }
                }
                0x8000..=0xFFFF => {
                    let bank = (addr as usize & 0x7FFF) / 0x2000;
                    let a = self.prg_bank[bank] + (addr & 0x1FFF);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Vrc7 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                _ => {
                    log::warn!("Vrc7: unhandled CPU read: address={:04x}", addr);
                    0
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    let bank = (addr as usize) / 0x0400;
                    let a = self.chr_bank[bank] + (addr & 0x03FF);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Vrc7 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                0x2000..=0x3FFF => {
                    let mirrored_addr = self.mirror_address(&*rom, addr);
                    nes.vram
                        .lock()
                        .expect("Failed to lock VRAM for read")
                        .read(nes, Address::Ppu(mirrored_addr))
                }
                _ => {
                    log::warn!("Vrc7: unhandled PPU read: address={:04x}", addr);
                    0
                }
            },
            _ => {
                log::warn!("Vrc7: unhandled read for address space: {:?}", address);
                0
            }
        }
    }

    fn write(&mut self, nes: &Nes, address: Address, val: u8) {
        let mut rom = nes.rom.lock().expect("Failed to lock ROM for write");
        match address {
            Address::Cpu(addr) => match addr {
                0x6000..=0x7FFF => self.wram.write(nes, Address::Cpu(addr & 0x1FFF), val),
                0x8000 => self.prg_bank[0] = Address::Prg8k(val as i16, 0),
                0x8010 => self.prg_bank[1] = Address::Prg8k(val as i16, 0),
                0x9000 => self.prg_bank[2] = Address::Prg8k(val as i16, 0),
                0x9010 => self.opl_index = val as usize,
                0x9030 => self.write_opl_register(val),
                0xA000 => self.chr_bank[0] = Address::Chr1k(val as i16, 0),
                0xA010 => self.chr_bank[1] = Address::Chr1k(val as i16, 0),
                0xB000 => self.chr_bank[2] = Address::Chr1k(val as i16, 0),
                0xB010 => self.chr_bank[3] = Address::Chr1k(val as i16, 0),
                0xC000 => self.chr_bank[4] = Address::Chr1k(val as i16, 0),
                0xC010 => self.chr_bank[5] = Address::Chr1k(val as i16, 0),
                0xD000 => self.chr_bank[6] = Address::Chr1k(val as i16, 0),
                0xD010 => self.chr_bank[7] = Address::Chr1k(val as i16, 0),
                0xE000 => self.mirror = val & 0xC3,
                0xE010 => self.irq_latch = val,
                0xF000 => {
                    self.irq_control = val;
                    self.cycle_counter = 0;
                    if val & 2 != 0 {
                        self.irq_counter = self.irq_latch as u16;
                    }
                }
                0xF010 => {
                    let eaa = (self.irq_control & 1) << 1;
                    self.irq_control &= !0x02;
                    self.irq_control |= eaa;
                }
                _ => {
                    log::warn!(
                        "Vrc7: unhandled CPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For Vrc7 with CHR RAM, this would write to CHR RAM.
                    let bank = (addr as usize) / 0x0400;
                    let a = self.chr_bank[bank] + (addr & 0x03FF);
                    rom.write(a, val).ok();
                }
                0x2000..=0x3FFF => {
                    let mirrored_addr = self.mirror_address(&*rom, addr);
                    nes.vram
                        .lock()
                        .expect("Failed to lock VRAM for write")
                        .write(nes, Address::Ppu(mirrored_addr), val);
                }
                _ => {
                    log::warn!(
                        "Vrc7: unhandled PPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            _ => {
                log::warn!(
                    "Vrc7: unhandled write for address space: {:?} value {:02x}",
                    address,
                    val
                );
            }
        }
    }

    fn tick(&mut self, nes: &Nes) {
        self.wram.tick(nes);
        let c = self.cycles;
        self.cycles += 1;
        let s1 = ((c as f64) / Self::SAMPLE_RATE) as u64;
        let s2 = ((self.cycles as f64) / Self::SAMPLE_RATE) as u64;
        if s1 != s2 {
            let output = self.opll.output();
            let mut sum = 0.0;
            for (&v, dbg) in output.iter().zip(self.opl_debug.iter_mut()) {
                dbg.debug_buf[dbg.debug_idx] = v;
                dbg.debug_idx = (dbg.debug_idx + 1) % dbg.debug_buf.len();
                sum += v * dbg.channel_volume;
            }
            sum = sum / (output.len() as f32);
            nes.audio_sample("vrc7", sum);
        }
        if self.irq_control & 2 != 0 {
            self.cycle_counter += 1;
            if self.cycle_counter >= 341 {
                self.cycle_counter -= 341;
                self.irq_counter += 1;
                if self.irq_counter == 0x100 {
                    self.irq_counter = self.irq_latch as u16;
                    nes.signal_irq();
                }
            }
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn decode_address(&self) -> Vec<AddressRange> {
        vec![
            AddressRange::cpu(0x6000, 0xA000),
            AddressRange::ppu(0x0000, 0x3f00),
        ]
    }
}
