use serde::{Deserialize, Serialize};

use crate::hwpalette;
use crate::peripheral::Peripheral;
use crate::{Address, Nes};

// The `EXPAND` tables expand the 8 bits of a byte into 8 zero-or-1 nybbles in
// a 32-bit word.  The byte-to-word expansions are used to quickly calculate
// color-index values at once for an 8-bit block of tile-data.
const fn expand<const N: usize>(left: bool) -> [u32; N] {
    let mut a = [0; N];
    let mut i = 0usize;
    while i < N {
        let mut data = 0;
        let n = i as u32;
        let mut j = 0;
        while j < 8 {
            let x = if left { 7 - j } else { j };
            data = (data << 4) | (n >> x) & 1;
            j += 1;
        }
        a[i] = data;
        i += 1;
    }
    a
}

const EXPAND_L: [u32; 256] = expand(true);
const EXPAND_R: [u32; 256] = expand(false);

pub const CTRL_NAMETABLE: u8 = 0x03u8;
pub const CTRL_INCREMENT: u8 = 0x04u8;
pub const CTRL_SPRITETABLE: u8 = 0x08u8;
pub const CTRL_BGTABLE: u8 = 0x10u8;
pub const CTRL_SPRITESIZE: u8 = 0x20u8;
pub const CTRL_MASTER: u8 = 0x40u8;
pub const CTRL_NMI: u8 = 0x80u8;

pub const MASK_GRAYSCALE: u8 = 0x01u8;
pub const MASK_SHOWLEFTBG: u8 = 0x02u8;
pub const MASK_SHOWLEFTSPRITE: u8 = 0x04u8;
pub const MASK_SHOWBG: u8 = 0x08u8;
pub const MASK_SHOWSPRITES: u8 = 0x10u8;
pub const MASK_REDTINT: u8 = 0x20u8;
pub const MASK_GREENTINT: u8 = 0x40u8;
pub const MASK_BLUETINT: u8 = 0x80u8;

pub const SPRITE_OVERFLOW: u8 = 0x20u8;
pub const SPRITE_ZEROHIT: u8 = 0x40u8;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Nmi {
    delay: u32,
    occurred: bool,
    output: bool,
    previous: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Sprite {
    pattern: u32,
    position: isize,
    priority: bool,
    index: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScrollPosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ppu {
    pub cycle: isize,
    pub scanline: isize,
    pub frame: u64,
    dead: u32,

    v: u16,
    t: u16,
    x: u8,
    w: bool,
    f: bool,

    pub control: u8,
    pub mask: u8,
    status: u8,
    nmi: Nmi,
    last_regval: u8,
    last_data: u8,

    nametable: u8,
    attrtable: u8,
    low_tile: u8,
    high_tile: u8,
    tiledata: u64,

    pub oam: Vec<u8>,
    oam_addr: u8,
    sprite: Vec<Sprite>,
    sprite_count: usize,
    pub scroll: Vec<ScrollPosition>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            oam: vec![0u8; 256],
            sprite: vec![Sprite::default(); 8],
            scroll: vec![ScrollPosition::default(); 262],
            ..Default::default()
        }
    }
    pub fn reset(&mut self) {
        self.dead = 2 * 262 * 341;
        self.cycle = 341 - 18;
        self.scanline = 239;
        self.frame = 0;
        self.oam_addr = 0;
        self.set_control(0);
        self.mask = 0;
    }

    fn set_control(&mut self, val: u8) {
        self.control = val;
        self.nmi.output = (val & CTRL_NMI) != 0;
        self.nmi_change();
        let nt = (val & CTRL_NAMETABLE) as u16;
        self.t = (self.t & 0xF3FF) | (nt << 10);
    }

    fn read_status_impl(&mut self) -> u8 {
        let result =
            (self.last_regval & 0x1F) | self.status | if self.nmi.occurred { 0x80 } else { 0x00 };
        self.nmi.occurred = false;
        self.nmi_change();
        self.w = false;
        result
    }

    fn set_scroll_impl(&mut self, val: u8) {
        let val = val as u16;
        if self.w == false {
            self.t = (self.t & 0xFFE0) | (val >> 3);
            self.x = (val & 7) as u8;
            self.w = true;
        } else {
            self.t = (self.t & 0x8FFF) | ((val & 0x07) << 12);
            self.t = (self.t & 0xFC1F) | ((val & 0xF8) << 2);
            self.w = false;
        }
    }

    fn set_address_impl(&mut self, val: u8) {
        let val = val as u16;
        if self.w == false {
            self.t = (self.t & 0x80FF) | (val & 0x3F) << 8;
            self.w = true;
        } else {
            self.t = (self.t & 0xFF00) | val;
            self.v = self.t;
            self.w = false;
        }
    }

    fn set_data_impl(&mut self, nes: &Nes, val: u8) {
        nes.write(Address::Ppu(self.v), val);
        self.v += if (self.control & CTRL_INCREMENT) != 0 {
            32
        } else {
            1
        };
    }

    fn read_data_impl(&mut self, nes: &Nes) -> u8 {
        let mut val = nes.read(Address::Ppu(self.v));
        if self.v % 0x4000 < 0x3F00 {
            std::mem::swap(&mut self.last_data, &mut val);
        } else {
            self.last_data = nes.read(Address::Ppu(self.v - 0x1000));
        }
        self.v += if (self.control & CTRL_INCREMENT) != 0 {
            32
        } else {
            1
        };
        val
    }

    fn set_dma_impl(&mut self, nes: &Nes, val: u8) {
        let mut addr = (val as u16) << 8;
        for _ in 0..256 {
            self.oam[self.oam_addr as usize] = nes.read(Address::Cpu(addr));
            self.oam_addr = self.oam_addr.wrapping_add(1);
            addr = addr.wrapping_add(1);
        }
        nes.dma_stall(513, true);
    }

    fn nmi_change(&mut self) {
        let nmi = self.nmi.output && self.nmi.occurred;
        if nmi && !self.nmi.previous {
            self.nmi.delay = 15;
        }
        self.nmi.previous = nmi;
    }
    fn set_vertical_blank(&mut self, value: bool) {
        self.nmi.occurred = value;
        self.nmi_change();
    }

    fn copy_x(&mut self) {
        self.v = (self.v & 0xFBE0) | (self.t & 0x041F);

        // Compute the scroll offset for the debug display
        let line = (self.scanline as usize + 1) % 262;
        let coarse_x = ((self.v & 0x001F) << 3) + ((self.v & 0x400) >> 2);
        let coarse_y = ((self.v & 0x03E0) >> 2) + if (self.v & 0x800) == 0 { 0 } else { 240 };
        let fine_y = (self.v & 0x7000) >> 12;
        self.scroll[line] = ScrollPosition {
            x: coarse_x + self.x as u16,
            y: coarse_y + fine_y,
        };
    }
    fn copy_y(&mut self) {
        self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
    }
    fn increment_x(&mut self) {
        if (self.v & 0x001F) == 0x001F {
            self.v = (self.v & 0xFFE0) ^ 0x0400;
        } else {
            self.v += 1;
        }
    }
    fn increment_y(&mut self) {
        if (self.v & 0x7000) != 0x7000 {
            self.v += 0x1000;
        } else {
            self.v &= 0x8FFF;
            let mut y = (self.v & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.v = (self.v & 0xFC1F) | (y << 5);
        }
    }

    fn store_tile_data(&mut self) {
        let aa = 0x44444444u32 * self.attrtable as u32;
        let data = EXPAND_L[self.low_tile as usize] | EXPAND_L[self.high_tile as usize] << 1;
        self.tiledata |= (data | aa) as u64;
    }
    fn background_pixel(&self) -> u8 {
        if (self.mask & MASK_SHOWBG) != 0 {
            let data = (self.tiledata >> 32) as u32;
            let data = (data >> ((7 - self.x) * 4)) & 0x0F;
            data as u8
        } else {
            0
        }
    }

    fn fetch_nametable_byte_impl(&mut self, nes: &Nes) {
        self.nametable = nes.read(Address::Ppu(0x2000 | (self.v & 0x0FFF)));
    }
    fn fetch_attribute_byte_impl(&mut self, nes: &Nes) {
        let a = 0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 7);
        let shift = ((self.v >> 4) & 4) | (self.v & 2);
        self.attrtable = (nes.read(Address::Ppu(a)) >> shift) & 3;
    }
    fn fetch_low_tile_byte_impl(&mut self, nes: &Nes) {
        let bgtable = if (self.control & CTRL_BGTABLE) != 0 {
            0x1000u16
        } else {
            0u16
        };
        let a = bgtable + (16 * self.nametable as u16) + ((self.v >> 12) & 7);
        self.low_tile = nes.read(Address::Ppu(a));
    }
    fn fetch_high_tile_byte_impl(&mut self, nes: &Nes) {
        let bgtable = if (self.control & CTRL_BGTABLE) != 0 {
            0x1000u16
        } else {
            0u16
        };
        let a = bgtable + (16 * self.nametable as u16) + ((self.v >> 12) & 7);
        self.high_tile = nes.read(Address::Ppu(a + 8));
    }

    fn sprite_pixel(&self) -> (usize, u8) {
        if (self.mask & MASK_SHOWSPRITES) != 0 {
            for i in 0..self.sprite_count {
                let offset = self.cycle - 1 - self.sprite[i].position;
                if offset < 0 || offset > 7 {
                    continue;
                }
                let offset = (7 - offset) * 4;
                let color = (self.sprite[i].pattern >> offset) & 0xF;
                if color % 4 == 0 {
                    continue;
                }
                return (i, color as u8);
            }
        }
        (0, 0)
    }

    fn render_pixel_impl(&mut self, nes: &Nes) {
        let x = (self.cycle - 1) as u32;
        let y = self.scanline as u32;
        let mut background = self.background_pixel();
        let (i, mut sprite) = self.sprite_pixel();
        let color;

        if x < 8 {
            if (self.mask & MASK_SHOWLEFTBG) == 0 {
                background = 0;
            }
            if (self.mask & MASK_SHOWLEFTSPRITE) == 0 {
                sprite = 0;
            }
        }

        let b = background % 4 != 0;
        let s = sprite % 4 != 0;

        if !b {
            color = if s { sprite | 0x10 } else { 0 };
        } else if !s {
            color = background;
        } else {
            if self.sprite[i].index == 0 && x < 255 {
                self.status |= SPRITE_ZEROHIT;
            }
            if self.sprite[i].priority {
                color = background;
            } else {
                color = sprite | 0x10;
            }
        }
        let color = nes.read(Address::Ppu(0x3F00 | color as u16));
        nes.set_pixel(x, y, hwpalette::PALETTE[color as usize]);
    }

    fn fetch_sprite_pattern_impl(&mut self, nes: &Nes, n: usize, row: isize) -> u32 {
        let mut tile = self.oam[n * 4 + 1] as u16;
        let attr = self.oam[n * 4 + 2];
        let table;
        let mut row = row;

        if (self.control & CTRL_SPRITESIZE) == 0 {
            if (attr & 0x80) != 0 {
                row = 7 - row;
            }
            table = if (self.control & CTRL_SPRITETABLE) == 0 {
                0
            } else {
                0x1000
            };
        } else {
            if (attr & 0x80) != 0 {
                row = 15 - row;
            }
            table = if (tile & 1) == 0 { 0 } else { 0x1000 };
            tile &= 0xFE;
            if row > 7 {
                tile += 1;
                row -= 8;
            }
        }

        let addr = table + tile * 16 + (row as u16);
        let a = (attr & 3) as u32;
        let lo = nes.read(Address::Ppu(addr)) as usize;
        let hi = nes.read(Address::Ppu(addr + 8)) as usize;
        let result = 0x11111111u32 * (a << 2);
        if (attr & 0x40) != 0 {
            result | EXPAND_R[lo] | EXPAND_R[hi] << 1
        } else {
            result | EXPAND_L[lo] | EXPAND_L[hi] << 1
        }
    }

    fn evaluate_sprites_impl(&mut self, nes: &Nes) {
        let height = if (self.control & CTRL_SPRITESIZE) != 0 {
            16isize
        } else {
            8isize
        };
        let mut count = 0usize;
        for i in 0..64 {
            let y = self.oam[i * 4 + 0] as isize;
            let a = self.oam[i * 4 + 2];
            let x = self.oam[i * 4 + 3] as isize;
            let row = self.scanline - y;

            if !(row >= 0 && row < height) {
                continue;
            }
            if count < self.sprite.len() {
                self.sprite[count].pattern = self.fetch_sprite_pattern_impl(nes, i, row);
                self.sprite[count].position = x;
                self.sprite[count].priority = (a & 0x20) != 0;
                self.sprite[count].index = i as u8;
            }
            count += 1;
        }
        if count > self.sprite.len() {
            count = self.sprite.len();
            self.status |= SPRITE_OVERFLOW;
        }
        self.sprite_count = count;
    }

    fn clock_impl(&mut self, nes: &Nes) -> bool {
        if self.dead > 0 {
            self.dead -= 1;
            return false;
        }

        if self.nmi.delay > 0 {
            self.nmi.delay -= 1;
            if self.nmi.delay == 0 && self.nmi.occurred && self.nmi.output {
                nes.signal_nmi();
            }
        }
        if (self.mask & (MASK_SHOWBG | MASK_SHOWSPRITES)) != 0 {
            if self.f && self.scanline == 261 && self.cycle == 339 {
                self.scanline = 0;
                self.cycle = 0;
                self.f = false;
                self.frame += 1;
                return true;
            }
        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                self.frame += 1;
                self.f = !self.f;
            }
        }
        true
    }

    pub fn rendering_enabled(&self) -> bool {
        self.mask & (MASK_SHOWBG | MASK_SHOWSPRITES) != 0
    }

    pub fn sprite_size(&self) -> bool {
        self.control & CTRL_SPRITESIZE != 0
    }
}

impl Peripheral for Ppu {
    fn write(&mut self, nes: &Nes, address: Address, val: u8) {
        let Address::Cpu(addr) = address else {
            return;
        };
        self.last_regval = val;
        if addr == 0x4014 {
            return self.set_dma_impl(nes, val);
        }
        match addr & 7 {
            0 => self.set_control(val),
            1 => self.mask = val,
            3 => self.oam_addr = val,
            4 => {
                self.oam[self.oam_addr as usize] = val;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            5 => self.set_scroll_impl(val),
            6 => self.set_address_impl(val),
            7 => self.set_data_impl(nes, val),
            _ => {}
        }
    }

    fn read(&mut self, nes: &Nes, address: Address) -> u8 {
        let Address::Cpu(addr) = address else {
            return 0xff;
        };
        match addr & 7 {
            2 => self.read_status_impl(),
            4 => self.oam[self.oam_addr as usize],
            7 => self.read_data_impl(nes),
            _ => 0,
        }
    }

    fn tick(&mut self, nes: &Nes) {
        if self.clock_impl(nes) {
            let pre_line = self.scanline == 261;
            let visible_line = self.scanline < 240;
            let render_line = pre_line || visible_line;
            let prefetch_cycle = self.cycle >= 321 && self.cycle <= 336;
            let visible_cycle = self.cycle > 0 && self.cycle <= 256;
            let fetch_cycle = prefetch_cycle || visible_cycle;
            if visible_line && visible_cycle {
                self.render_pixel_impl(nes);
            }
            if (self.mask & (MASK_SHOWBG | MASK_SHOWSPRITES)) != 0 {
                if render_line && fetch_cycle {
                    self.tiledata <<= 4;
                    match self.cycle % 8 {
                        0 => self.store_tile_data(),
                        1 => self.fetch_nametable_byte_impl(nes),
                        3 => self.fetch_attribute_byte_impl(nes),
                        5 => self.fetch_low_tile_byte_impl(nes),
                        7 => self.fetch_high_tile_byte_impl(nes),
                        _ => {}
                    };
                }

                if pre_line && self.cycle >= 280 && self.cycle <= 304 {
                    self.copy_y();
                }
                if render_line {
                    if fetch_cycle && self.cycle % 8 == 0 {
                        self.increment_x();
                    }
                    if self.cycle == 256 {
                        self.increment_y();
                    }
                    if self.cycle == 257 {
                        self.copy_x();
                    }
                }
                if self.cycle == 257 {
                    if visible_line {
                        self.evaluate_sprites_impl(nes);
                    } else {
                        self.sprite_count = 0;
                    }
                }
            }
            if self.scanline == 241 && self.cycle == 1 {
                self.set_vertical_blank(true);
            }
            if pre_line && self.cycle == 1 {
                self.set_vertical_blank(false);
                self.status = 0;
            }
        }
    }
}
