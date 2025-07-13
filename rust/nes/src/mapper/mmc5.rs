use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::apu::{Apu, ApuPulse};
use crate::mapper::simple_mirror_address;
use crate::peripheral::{Mapper, Peripheral};
use crate::ram::{Ram, RamKind};
use crate::system::Nes;
use crate::NesFile;
use crate::{Address, AddressRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMC5 {
    prg_banks: usize,
    prg_mode: u8,
    chr_mode: u8,
    prg_ram_protect: [u8; 2],
    ext_ram_mode: u8,
    nt_map: u8,
    fill_tile: u8,
    fill_color: u8,
    prg_bank: [usize; 5],
    chr_bank: [usize; 12],
    chr_upper: usize,

    vsplit_mode: u8,
    vsplit_scroll: u8,
    vsplit_bank: u8,
    vsplit_region: bool,

    irq_scanline: u8,
    irq_enable: u8,
    irq_status: u8,
    scanline_counter: u8,

    multiplier: [u16; 2],

    timer: u16,
    timer_irq: u8,
    timer_running: bool,

    clock_divider: u8,
    bg_fetch: bool,
    ppu_rendering_enabled: bool,
    ppu_sprite_size: bool,
    ppu_scanline: isize,
    ppu_cycle: isize,
    cycle: usize,

    ext_ram: Ram,
    wram: Ram,

    pub(crate) pulse: [ApuPulse; 2],
}

impl MMC5 {
    pub fn new(rom: &NesFile) -> Result<Self> {
        Ok(Self {
            prg_banks: rom.prg_banks() * 2,
            prg_mode: 0,
            chr_mode: 0,
            prg_ram_protect: [0, 0],
            ext_ram_mode: 0,
            nt_map: 0,
            fill_tile: 0,
            fill_color: 0,
            prg_bank: [0, 0, 0, 0, 0xFF],
            chr_bank: [0; 12],
            chr_upper: 0,
            vsplit_mode: 0,
            vsplit_scroll: 0,
            vsplit_bank: 0,
            vsplit_region: false,
            irq_scanline: 0,
            irq_enable: 0,
            irq_status: 0,
            scanline_counter: 0,
            multiplier: [0xff; 2],
            timer: 0,
            timer_irq: 0,
            timer_running: false,

            clock_divider: 0,
            bg_fetch: false,
            ppu_rendering_enabled: false,
            ppu_sprite_size: false,
            ppu_scanline: 0,
            ppu_cycle: 0,
            cycle: 0,

            ext_ram: Ram::new(RamKind::WRam, 1024)?,
            // TODO: how much wram? 64K?
            wram: Ram::new(RamKind::WRam, 65536)?,

            pulse: [ApuPulse::new(0), ApuPulse::new(1)],
        })
    }

    fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            0x5100 => self.prg_mode,
            0x5101 => self.chr_mode,
            0x5102 => self.prg_ram_protect[0],
            0x5103 => self.prg_ram_protect[1],
            0x5104 => self.ext_ram_mode,
            0x5105 => self.nt_map,
            0x5106 => self.fill_tile,
            0x5107 => self.fill_color,
            0x5113..=0x5117 => self.prg_bank[(addr - 0x5113) as usize] as u8,
            0x5120..=0x512b => self.chr_bank[(addr - 0x5120) as usize] as u8,
            0x5130 => self.chr_upper as u8,
            0x5200 => self.vsplit_mode,
            0x5201 => self.vsplit_scroll,
            0x5202 => self.vsplit_bank,
            0x5203 => self.irq_scanline,
            0x5204 => {
                let val = self.irq_status;
                self.irq_status &= !0x80;
                val
            }
            0x5205 => (self.multiplier[0] * self.multiplier[1]) as u8,
            0x5206 => ((self.multiplier[0] * self.multiplier[1]) >> 8) as u8,
            0x5209 => {
                let val = self.timer_irq;
                self.timer_irq &= !0x80;
                val
            }
            _ => {
                log::warn!("MMC5: unhandled register read: {:x}", addr);
                0xff
            }
        }
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        match addr {
            0x5000 => self.pulse[0].set_control(val),
            0x5001 => { /* MMC5 pulse channels have no sweep unit. */ }
            0x5002 => self.pulse[0].set_timer_low(val),
            0x5003 => self.pulse[0].set_timer_high(val),
            0x5004 => self.pulse[1].set_control(val),
            0x5005 => { /* MMC5 pulse channels have no sweep unit. */ }
            0x5006 => self.pulse[1].set_timer_low(val),
            0x5007 => self.pulse[1].set_timer_high(val),
            0x5015 => {
                self.pulse[0].set_enabled(val & 0x01 != 0);
                self.pulse[1].set_enabled(val & 0x02 != 0);
            }

            0x5100 => self.prg_mode = val & 0x03,
            0x5101 => self.chr_mode = val & 0x03,
            0x5102 => self.prg_ram_protect[0] = val & 0x03,
            0x5103 => self.prg_ram_protect[1] = val & 0x03,
            0x5104 => self.ext_ram_mode = val & 0x03,
            0x5105 => self.nt_map = val,
            0x5106 => self.fill_tile = val,
            0x5107 => self.fill_color = val & 0x03,
            0x5113..=0x5117 => self.prg_bank[(addr - 0x5113) as usize] = val as usize,
            0x5120..=0x512b => self.chr_bank[(addr - 0x5120) as usize] = val as usize,
            0x5130 => self.chr_upper = val as usize & 0x03,
            0x5200 => {
                self.vsplit_mode = val & 0xDF;
                self.vsplit_region = false;
            }
            0x5201 => self.vsplit_scroll = val,
            0x5202 => self.vsplit_bank = val,
            0x5203 => self.irq_scanline = val,
            0x5204 => self.irq_enable = val & 0x80,
            0x5205 => self.multiplier[0] = val as u16,
            0x5206 => self.multiplier[1] = val as u16,
            0x5209 => {
                self.timer = (self.timer & 0xFF00) | (val as u16);
                self.timer_running = true;
            }
            0x520a => {
                self.timer = (self.timer & 0x00FF) | ((val as u16) << 8);
            }
            _ => {
                log::warn!("MMC5: unhandled register write: {:x} {:02x}", addr, val);
            }
        }
    }

    fn check_scanline(&mut self, nes: &Nes) {
        // According to the nesdev MMC5 document, the IRQ should happen at PPU cycle 4.
        if self.ppu_cycle == 4 {
            if self.ppu_rendering_enabled && self.ppu_scanline > 240 {
                // Clear irq_pending and in_frame.
                self.irq_status &= !0xC0;
                self.scanline_counter = 0;
            } else {
                if self.irq_status & 0x40 != 0 {
                    // If in_frame and the count is equal, signal an interrupt.
                    self.scanline_counter += 1;
                    if self.scanline_counter == self.irq_scanline {
                        self.irq_status |= 0x80;
                        if self.irq_enable & 0x80 != 0 {
                            nes.signal_irq();
                        }
                    }
                } else {
                    // Not in_frame; become in_frame.
                    self.irq_status = (self.irq_status | 0x40) & !0x80;
                    self.scanline_counter = 0;
                }
            }
        }
    }

    fn check_timer(&mut self, nes: &Nes) {
        if self.timer_running {
            self.timer = self.timer.wrapping_sub(1);
            if self.timer == 0 {
                self.timer_running = false;
                self.timer_irq |= 0x80;
                nes.signal_irq();
            }
        }
    }

    fn _prg_bank(&self, index: usize) -> usize {
        (self.prg_bank[index] & 0x7f) % self.prg_banks
    }

    fn translate_prg(&self, addr: u16) -> usize {
        let addr = addr as usize;
        match self.prg_mode {
            // Mode 0 is 1 x 32KiB mode
            0 => (self._prg_bank(4) & !3) * 8192 | (addr & 0x7fff),
            // Mode 1 is 2 x 16KiB mode
            1 => {
                let index = if addr & 0x4000 != 0 { 4 } else { 2 };
                (self._prg_bank(index) & !1) * 8192 | (addr & 0x3fff)
            }
            // Mode 2 is 16KiB + 2 x 8KiB mode
            2 => {
                if addr & 0x4000 != 0 {
                    let index = if addr & 0x2000 != 0 { 4 } else { 3 };
                    self._prg_bank(index) * 8192 | (addr & 0x1fff)
                } else {
                    (self._prg_bank(2) & !1) * 8192 | (addr & 0x3fff)
                }
            }
            // Mode 3 is 4 x 8KiB banks
            3 => {
                let index = (addr >> 13) & 3;
                self._prg_bank(1 + index) * 8192 | (addr & 0x1fff)
            }
            _ => unreachable!(),
        }
    }

    fn translate_chr(&self, addr: u16) -> usize {
        let (addr, mode, regofs) =
            if self.ppu_rendering_enabled && self.ppu_sprite_size && self.bg_fetch {
                let mode = if self.chr_mode == 0 { 1 } else { self.chr_mode };
                (addr as usize & 0x0FFF, mode, 8)
            } else {
                (addr as usize, self.chr_mode, 0)
            };

        let superbank = self.chr_upper << 8;
        match mode {
            // Mode 0 is 8KiB mode
            0 => {
                let bank = superbank | self.chr_bank[regofs + 7];
                (bank << 13) | (addr & 0x1FFF)
            }
            // Mode 1 is 4KiB mode
            1 => {
                let regofs = regofs + (addr >> 12) * 4;
                let bank = superbank | self.chr_bank[regofs + 3];
                (bank << 12) | (addr & 0x0FFF)
            }
            // Mode 2 is 2KiB mode
            2 => {
                let regofs = regofs + (addr >> 11) * 2;
                let bank = superbank | self.chr_bank[regofs + 1];
                (bank << 11) | (addr & 0x07FF)
            }
            // Mode 3 is 1KiB mode
            3 => {
                let regofs = regofs + (addr >> 10) * 1;
                let bank = superbank | self.chr_bank[regofs];
                (bank << 10) | (addr & 0x03FF)
            }
            _ => unreachable!(),
        }
    }

    fn _ram_readwrite(ram: &mut Ram, nes: &Nes, addr: Address, value: Option<u8>) -> u8 {
        if let Some(v) = value {
            ram.write(nes, addr, v);
            0xff
        } else {
            ram.read(nes, addr)
        }
    }

    fn vram_access(&mut self, nes: &Nes, addr: u16, value: Option<u8>) -> u8 {
        let offset = addr & 0x3ff;
        let table = (addr >> 10) & 3;
        let table = (self.nt_map >> (table * 2)) & 3;

        if self.vsplit_mode & 0x80 != 0 {
            let tile = self.vsplit_mode as isize & 0x1f;
            // Re-compute the PPU position from scanline/cycle data.
            let mut scanline = self.ppu_scanline;
            let mut pputile = self.ppu_cycle + 15;
            if pputile >= 336 {
                // PPU fetches the next line's first two tiles during HBlank.
                pputile -= 336;
                scanline += 1;
            }
            pputile /= 8;
            let mut sofs = ((scanline / 8) * 32 + pputile) as u16;
            if offset >= 0x3c0 {
                // Attribute fetch.
                sofs = 0x3c0 | ((sofs >> 4) & 0x38) | ((sofs >> 2) & 7);
            }
            if self.vsplit_mode & 0x40 != 0 {
                // The split is on the right side.
                if pputile >= tile {
                    self.vsplit_region = true;
                    return Self::_ram_readwrite(&mut self.ext_ram, nes, Address::Ppu(sofs), value);
                }
            } else {
                if pputile < tile {
                    self.vsplit_region = true;
                    return Self::_ram_readwrite(&mut self.ext_ram, nes, Address::Ppu(sofs), value);
                }
            }
            self.vsplit_region = false;
        }
        match table {
            0 => Self::_ram_readwrite(
                &mut *nes.vram.lock().expect("failed to lock vram"),
                nes,
                Address::Ppu(offset),
                value,
            ),
            1 => Self::_ram_readwrite(
                &mut *nes.vram.lock().expect("failed to lock vram"),
                nes,
                Address::Ppu(offset + 0x400),
                value,
            ),
            2 if self.ext_ram_mode <= 1 => {
                Self::_ram_readwrite(&mut self.ext_ram, nes, Address::Ppu(offset), value)
            }
            3 => {
                if offset < 0x3c0 {
                    self.fill_tile
                } else {
                    let mut color = self.fill_color;
                    color |= color << 2;
                    color | color << 4
                }
            }
            _ => 0xff,
        }
    }

    fn emulate_audio(&mut self, nes: &Nes) {
        let c1 = self.cycle as f64;
        self.cycle += 1;
        let c2 = self.cycle as f64;

        if self.cycle % 2 == 0 {
            // Pulse channels are clocked at half the CPU rate.
            self.pulse[0].step_timer();
            self.pulse[1].step_timer();
        }

        let f1 = (c1 / Apu::FRAME_COUNTER_RATE) as usize;
        let f2 = (c1 / Apu::FRAME_COUNTER_RATE) as usize;
        if f1 != f2 {
            // Pulse envelopes and lengths are clocked at 240 Hz.
            self.pulse[0].step_envelope();
            self.pulse[0].step_length();
            self.pulse[1].step_envelope();
            self.pulse[1].step_length();
        }

        let s1 = (c1 / Apu::SAMPLE_RATE) as usize;
        let s2 = (c2 / Apu::SAMPLE_RATE) as usize;
        if s1 != s2 {
            nes.audio_sample("MMC5: Pulse 0", self.pulse[0].output());
            nes.audio_sample("MMC5: Pulse 1", self.pulse[1].output());
        }
    }
}

impl Mapper for MMC5 {
    fn clone(&self) -> Box<dyn Mapper> {
        Box::new(Clone::clone(self))
    }
}
impl Peripheral for MMC5 {
    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let rom = nes.rom.lock().expect("Failed to lock ROM for read");
        match address {
            Address::Cpu(addr) => match addr {
                0x5000..=0x57FF => self.read_register(addr),
                0x5800..=0x5BFF => {
                    log::warn!("MMC5: unknown register read: address={:04x}", addr);
                    0xFF
                }
                0x5C00..=0x5FFF => {
                    // TODO: should depend on the ext_ram_mode.
                    self.ext_ram.read(nes, Address::Cpu(addr - 0x5c00))
                }
                0x6000..=0x7FFF => {
                    // TODO: Technically, MMC5 can address up to 128K of WRAM,
                    // but I don't think any game ever used that much.  This
                    // implementation addresses up to 64K of WRAM.
                    let addr = (self.prg_bank[0] & 7) * 8192 + (addr as usize & 0x1fff);
                    self.wram.read(nes, Address::Cpu(addr as u16))
                }
                0x8000..=0xFFFF => {
                    // Because of the complexity of MMC5 PRG modes, we just
                    // calculate the file offset rather than using the Prg variants.
                    let addr = self.translate_prg(addr);
                    let a = Address::File(16 + addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC5 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                _ => {
                    log::warn!("MMC5: unhandled CPU read: address={:04x}", addr);
                    0
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Because of the complexity of MMC5 CHR modes, we just
                    // calculate the file offset rather than using the Chr variants.
                    let addr = if self.vsplit_region && self.bg_fetch {
                        // If we're in the vsplit region, backgrounds are fetched from
                        // a contiguous 4K bank encoded in vsplit_bank.
                        (self.vsplit_bank as usize * 4096) + (addr as usize & 0xFFF)
                    } else {
                        // Normal background and sprite fetches go through the banking
                        // registers.
                        self.translate_chr(addr)
                    };
                    let a = Address::File(16 + 8192 * self.prg_banks + addr);
                    match rom.read(a) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("MMC5 failed to read rom {a:x?}: {e}");
                            0xff
                        }
                    }
                }
                0x2000..=0x3FFF => self.vram_access(nes, addr, None),
                _ => {
                    log::warn!("MMC5: unhandled PPU read: address={:04x}", addr);
                    0
                }
            },
            _ => {
                log::warn!("MMC5: unhandled read for address space: {:?}", address);
                0
            }
        }
    }

    fn write(&mut self, nes: &Nes, address: Address, val: u8) {
        match address {
            Address::Cpu(addr) => match addr {
                0x5000..=0x57FF => self.write_register(addr, val),
                0x5800..=0x5BFF => {
                    log::warn!(
                        "MMC5: unknown register write: address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
                0x5C00..=0x5FFF => {
                    // TODO: should depend on the ext_ram_mode.
                    self.ext_ram.write(nes, Address::Cpu(addr - 0x5c00), val)
                }
                0x6000..=0x7FFF => {
                    // TODO: Technically, MMC5 can address up to 128K of WRAM,
                    // but I don't think any game ever used that much.  This
                    // implementation addresses up to 64K of WRAM.
                    let enabled = self.prg_ram_protect[0] == 2 && self.prg_ram_protect[1] == 1;
                    if enabled {
                        let addr = (self.prg_bank[0] & 7) * 8192 + (addr as usize & 0x1fff);
                        self.wram.write(nes, Address::Cpu(addr as u16), val)
                    }
                }
                0x8000..=0xFFFF => {
                    // TODO: depends on PRG mapping and WP values.  It is possible
                    // to address RAM in this region.
                    log::warn!("MMC5: unhandled MMC5 PRG write at {:04x}", addr);
                }
                _ => {
                    log::warn!(
                        "MMC5: unhandled CPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            Address::Ppu(addr) => match addr {
                0x0000..=0x1FFF => {
                    // Attempting to write to CHR ROM, typically no-op or mapper specific.
                    // For MMC5 with CHR RAM, this would write to CHR RAM.
                }
                0x2000..=0x3FFF => {
                    self.vram_access(nes, addr, Some(val));
                }
                _ => {
                    log::warn!(
                        "MMC5: unhandled PPU write address={:04x} value={:02x}",
                        addr,
                        val
                    );
                }
            },
            _ => {
                log::warn!(
                    "MMC5: unhandled write for address space: {:?} value {:02x}",
                    address,
                    val
                );
            }
        }
    }

    fn tick(&mut self, nes: &Nes) {
        self.wram.tick(nes);
        {
            // Remember some PPU state so the rest of the mapper
            // can function.
            let ppu = nes.ppu.lock().expect("mmc5 failed to lock ppu");
            self.ppu_rendering_enabled = ppu.rendering_enabled();
            self.ppu_sprite_size = ppu.sprite_size();
            self.ppu_scanline = ppu.scanline;
            self.ppu_cycle = ppu.cycle;
            self.bg_fetch = ppu.cycle < 256 || ppu.cycle >= 321 && ppu.cycle < 336;
        }
        self.check_scanline(nes);

        self.clock_divider += 1;
        if self.clock_divider == 3 {
            self.clock_divider = 0;
            self.check_timer(nes);
            self.emulate_audio(nes);
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
            AddressRange::cpu(0x5000, 0xB000),
            AddressRange::ppu(0x0000, 0x3f00),
        ]
    }
}
