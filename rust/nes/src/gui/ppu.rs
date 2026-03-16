use python_gui::{Color, Image};

use crate::hwpalette;
use crate::peripheral::Peripheral;
use crate::ppu;
use crate::system::Nes;
use crate::Address;

const GREY: [f32; 4] = [0.5, 0.5, 0.5, 0.20];

const HEXDIGIT: [u8; 16] = [
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
];

const PALETTE_NAMES: [&str; 8] = [
    "Background 0",
    "Background 1",
    "Background 2",
    "Background 3",
    "    Sprite 0",
    "    Sprite 1",
    "    Sprite 2",
    "    Sprite 3",
];

pub struct PpuDebug {
    pub chr_visible: bool,
    pub vram_visible: bool,
    chr_palette: [Option<u8>; 2],
    chr_image: [Image; 2],
    best_color: [Vec<u8>; 2],
    scr_pos: Vec<Vec<[f32; 2]>>,
    scr_overlay: [f32; 4],
    spr_alpha: f32,
}

impl PpuDebug {
    pub fn new() -> Self {
        PpuDebug {
            chr_visible: false,
            vram_visible: false,
            chr_palette: [None; 2],
            chr_image: [Image::new(128, 128), Image::new(128, 128)],
            best_color: [vec![0; 256], vec![0; 256]],
            scr_pos: vec![vec![[0.0; 2]; 64]; 60],
            scr_overlay: GREY,
            spr_alpha: 1.0,
        }
    }

    fn most_common(counts: &[u8; 4]) -> u8 {
        let mut max = 0;
        let mut n = 0;
        for (i, &count) in counts[1..].iter().enumerate() {
            if count > max {
                max = count;
                n = (i + 1) as u8;
            }
        }
        n
    }
    fn update_chr_image(&mut self, nes: &Nes, bank: usize, palette: Option<u8>) {
        let pal = if let Some(p) = palette {
            let mut pram = nes.pram.lock().expect("debug pram");
            [
                hwpalette::get(pram.read(nes, Address::Ppu(p as u16 * 4 + 0)) as usize),
                hwpalette::get(pram.read(nes, Address::Ppu(p as u16 * 4 + 1)) as usize),
                hwpalette::get(pram.read(nes, Address::Ppu(p as u16 * 4 + 2)) as usize),
                hwpalette::get(pram.read(nes, Address::Ppu(p as u16 * 4 + 3)) as usize),
            ]
        } else {
            [0xFF000000u32, 0xFF666666u32, 0xFFAAAAAAu32, 0xFFFFFFFFu32]
        };
        let offset = bank * 0x1000;
        for y in 0..16 {
            for x in 0..16 {
                let tile = y * 16 + x;
                let mut counts = [0u8; 4];
                for row in 0..8 {
                    let mut a = nes.read(Address::Ppu((offset + 16 * tile + row) as u16));
                    let mut b = nes.read(Address::Ppu((offset + 16 * tile + row + 8) as u16));
                    for col in 0..8 {
                        let color = ((a & 0x80) >> 7) | ((b & 0x80) >> 6);
                        self.chr_image[bank].set_pixel(
                            (8 * x + col) as u32,
                            (8 * y + row) as u32,
                            Color::new(pal[color as usize]),
                        );
                        counts[color as usize] += 1;
                        a <<= 1;
                        b <<= 1;
                    }
                }
                self.best_color[bank][tile] = PpuDebug::most_common(&counts);
            }
        }
    }

    fn palette_buttons(palette: u8, nes: &Nes, ui: &imgui::Ui) {
        let mut pram = nes.pram.lock().expect("debug pram");
        for i in 0..4 {
            let pcolor = pram.read(nes, Address::Ppu(palette as u16 * 4 + i));
            ui.same_line();
            ui.color_button(
                format!("{:02x}##{i}", pcolor),
                hwpalette::fget(pcolor as usize),
            );
        }
    }

    fn draw_chr(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if !self.chr_visible {
            return;
        }
        let mut visible = self.chr_visible;
        ui.window("CHR View").opened(&mut visible).build(|| {
            for i in 0..2 {
                let id = ui.push_id_usize(i);
                ui.text(format!("Pattern Table {}", i));
                self.chr_image[i].draw(4.0, ui);
                ui.same_line();
                ui.group(|| {
                    ui.radio_button("None", &mut self.chr_palette[i], None);
                    for j in 0..8u8 {
                        let id = ui.push_id_usize(j as usize);
                        ui.radio_button(
                            PALETTE_NAMES[j as usize],
                            &mut self.chr_palette[i],
                            Some(j),
                        );
                        PpuDebug::palette_buttons(j, nes, ui);
                        id.pop();
                    }
                });
                id.pop();
            }
        });
        self.chr_visible = visible;
    }

    fn print_address(addr: u16, ui: &imgui::Ui) {
        let addr = addr as usize;
        let text = [
            ' ' as u8,
            HEXDIGIT[(addr >> 12) & 0xF],
            HEXDIGIT[(addr >> 8) & 0xF],
            HEXDIGIT[(addr >> 4) & 0xF],
            HEXDIGIT[(addr >> 0) & 0xF],
            ':' as u8,
        ];

        unsafe {
            ui.text(std::str::from_utf8_unchecked(&text));
        }
    }

    fn print_colored_hex(&self, nes: &Nes, table: usize, val: u8, palette: u8, ui: &imgui::Ui) {
        let val = val as usize;
        let text = [HEXDIGIT[(val >> 4) & 0xF], HEXDIGIT[(val >> 0) & 0xF]];

        let mut pram = nes.pram.lock().expect("debug pram");
        let color = pram.read(
            nes,
            Address::Ppu((palette * 4 + self.best_color[table][val]) as u16),
        );
        unsafe {
            ui.text_colored(
                hwpalette::fget(color as usize),
                std::str::from_utf8_unchecked(&text),
            );
        }
    }

    fn hexdump_vram(&mut self, nes: &Nes, ui: &imgui::Ui, v: u16) {
        let table = if (nes.ppu.lock().expect("hexdump_vram").control & ppu::CTRL_BGTABLE) != 0 {
            1
        } else {
            0
        };
        let x0 = if (v & 0x400) == 0 { 0 } else { 32 };
        let y0 = if (v & 0x800) == 0 { 0 } else { 30 };
        let mut v = v;
        for y in 0..30 {
            for x in 0..32 {
                let val = nes.read(Address::Ppu(v));
                let a = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 7);
                let shift = ((v >> 4) & 4) | (v & 2);
                let attr = (nes.read(Address::Ppu(a)) >> shift) & 3;
                if x == 0 {
                    PpuDebug::print_address(v, ui);
                }
                ui.same_line();
                self.scr_pos[y0 + y][x0 + x] = ui.cursor_screen_pos();
                self.print_colored_hex(nes, table, val, attr, ui);
                v += 1;
            }
        }
    }

    fn draw_visibile_area(&mut self, nes: &Nes, ui: &imgui::Ui) {
        let draw_list = ui.get_window_draw_list();
        let ppu = nes.ppu.lock().expect("ppu debug");
        let mut pram = nes.pram.lock().expect("debug pram");
        let xmul = (self.scr_pos[0][1][0] - self.scr_pos[0][0][0]) / 8.0;
        let ymul = (self.scr_pos[1][0][1] - self.scr_pos[0][0][1]) / 8.0;
        let sprsz = if ppu.control & ppu::CTRL_SPRITESIZE == 0 {
            8
        } else {
            16
        };
        let sprtbl = if ppu.control & ppu::CTRL_SPRITETABLE == 0 {
            0
        } else {
            1
        };
        let alpha = ((self.spr_alpha * 255.0) as u32) << 24;
        draw_list.channels_split(2, |channels| {
            for line in 1..240 {
                // Based on the scroll positions saved by the PPU, highlight
                // the visible area of VRAM.
                let x0 = ppu.scroll[line].x;
                let y0 = ppu.scroll[line].y;
                let x1 = (ppu.scroll[line].x + 255) % 512;
                let coarse_x0 = (x0 / 8) as usize;
                let coarse_x1 = (x1 / 8) as usize;
                let coarse_y = (y0 / 8) as usize;
                let fine_x0 = x0 % 8;
                let fine_x1 = x1 % 8;
                let fine_y = y0 % 8;

                let mut p1 = self.scr_pos[coarse_y][coarse_x0];
                p1[0] += xmul * fine_x0 as f32;
                p1[1] += ymul * fine_y as f32;
                let mut p2 = self.scr_pos[coarse_y][coarse_x1];
                p2[0] += xmul * fine_x1 as f32;
                p2[1] += ymul * (fine_y + 1) as f32;

                channels.set_current(0);
                if p1[0] < p2[0] {
                    // Single contiguous visible area
                    draw_list
                        .add_rect(p1, p2, self.scr_overlay)
                        .filled(true)
                        .build();
                } else {
                    // Visible area wraps around edge of nametables.
                    let mut lhs = self.scr_pos[coarse_y][0];
                    lhs[1] += ymul * fine_y as f32;
                    let mut rhs = self.scr_pos[coarse_y][63];
                    rhs[0] += xmul * 8.0;
                    rhs[1] += ymul * (fine_y + 1) as f32;
                    draw_list
                        .add_rect(p1, rhs, self.scr_overlay)
                        .filled(true)
                        .build();
                    draw_list
                        .add_rect(lhs, p2, self.scr_overlay)
                        .filled(true)
                        .build();
                }

                // Compute where to draw sprites within the visible area.
                channels.set_current(1);
                for i in 0..64 {
                    let y = ppu.oam[i * 4 + 0] as usize;
                    if y != line - 1 {
                        continue;
                    }
                    let sprid = ppu.oam[i * 4 + 1] as usize;
                    let mut tile = sprid;
                    let attr = ppu.oam[i * 4 + 2];
                    let x = (ppu.oam[i * 4 + 3] as u16 + x0) as usize % 512;
                    let table = if sprsz == 16 {
                        tile &= !1;
                        (tile & 1) as usize
                    } else {
                        sprtbl
                    };
                    let sp0 = [
                        self.scr_pos[coarse_y][x / 8][0] + xmul * (x % 8) as f32,
                        self.scr_pos[coarse_y][x / 8][1] + ymul * fine_y as f32,
                    ];
                    let mut sp1 = sp0;
                    sp1[0] += xmul * 8.0;
                    sp1[1] += ymul * (sprsz as f32);

                    let palette = 0x10 + 4 * (attr & 3);
                    let color = hwpalette::get(pram.read(
                        nes,
                        Address::Ppu((palette + self.best_color[table][tile]) as u16),
                    ) as usize);
                    let color = (color & 0x00FFFFFF) | alpha;
                    draw_list
                        .add_rect(sp0, sp1, hwpalette::color_as_f32(color))
                        .filled(true)
                        .build();

                    let text = [
                        HEXDIGIT[(i >> 4) & 0xF],
                        HEXDIGIT[(i >> 0) & 0xF],
                        '\n' as u8,
                        HEXDIGIT[(sprid >> 4) & 0xF],
                        HEXDIGIT[(sprid >> 0) & 0xF],
                    ];
                    unsafe {
                        draw_list.add_text(
                            sp0,
                            hwpalette::color_as_f32(color ^ 0x00FFFFFF),
                            std::str::from_utf8_unchecked(&text),
                        );
                    }
                }
            }
        });
    }

    fn ppu_ctrl(&mut self, nes: &Nes, ui: &imgui::Ui) {
        let ppu = nes.ppu.lock().expect("ppu_ctrl");
        ui.text(format!(
            "{}{}{}{}{}{}{}  ",
            if ppu.control & 0x80 == 0 { "v" } else { "V" },
            if ppu.control & 0x40 == 0 { "p" } else { "P" },
            if ppu.control & 0x20 == 0 { "h" } else { "H" },
            if ppu.control & 0x10 == 0 { "b" } else { "B" },
            if ppu.control & 0x08 == 0 { "s" } else { "S" },
            if ppu.control & 0x04 == 0 { "i" } else { "I" },
            ppu.control & 3
        ));
    }

    fn ppu_mask(&mut self, nes: &Nes, ui: &imgui::Ui) {
        let mut ppu = nes.ppu.lock().expect("ppu_mask");
        ui.text(format!(
            "{}{}{} {} {} {} {} {}",
            if ppu.mask & 0x80 == 0 { "b" } else { "B" },
            if ppu.mask & 0x40 == 0 { "g" } else { "G" },
            if ppu.mask & 0x20 == 0 { "r" } else { "R" },
            if ppu.mask & 0x10 == 0 { "spr" } else { "SPR" },
            if ppu.mask & 0x08 == 0 { "bg" } else { "BG" },
            if ppu.mask & 0x04 == 0 { "lspr" } else { "LSPR" },
            if ppu.mask & 0x02 == 0 { "lbg" } else { "LBG" },
            if ppu.mask & 0x01 == 0 { "gr" } else { "GR" },
        ));
        ui.same_line();
        ui.checkbox("Sprites", &mut ppu.dbg_render_sprites);
        ui.same_line();
        ui.checkbox("Background", &mut ppu.dbg_render_background);
    }
    pub const _MASK_GRAYSCALE: u8 = 0x01u8;
    pub const MASK_SHOWLEFTBG: u8 = 0x02u8;
    pub const MASK_SHOWLEFTSPRITE: u8 = 0x04u8;
    pub const MASK_SHOWBG: u8 = 0x08u8;
    pub const MASK_SHOWSPRITES: u8 = 0x10u8;
    pub const _MASK_REDTINT: u8 = 0x20u8;
    pub const _MASK_GREENTINT: u8 = 0x40u8;
    pub const _MASK_BLUETINT: u8 = 0x80u8;

    fn draw_vram(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if !self.vram_visible {
            return;
        }
        let mut visible = self.vram_visible;
        ui.window("VRAM View").opened(&mut visible).build(|| {
            self.ppu_ctrl(nes, ui);
            ui.same_line();
            self.ppu_mask(nes, ui);
            ui.same_line();
            ui.color_edit4_config("Overlay", &mut self.scr_overlay)
                .alpha(true)
                .inputs(false)
                .picker(true)
                .build();
            ui.same_line();
            let token = ui.push_item_width(64.0);
            imgui::Drag::new("Sprite Alpha")
                .range(0.0, 1.0)
                .speed(0.01)
                .display_format("%.02f")
                .build(ui, &mut self.spr_alpha);
            token.end();

            let style = ui.push_style_var(imgui::StyleVar::ItemSpacing([1.0, 1.0]));
            ui.group(|| self.hexdump_vram(nes, ui, 0x2000));
            ui.same_line();
            ui.group(|| self.hexdump_vram(nes, ui, 0x2400));
            ui.text("");
            ui.group(|| self.hexdump_vram(nes, ui, 0x2800));
            ui.same_line();
            ui.group(|| self.hexdump_vram(nes, ui, 0x2c00));
            style.pop();
            self.draw_visibile_area(nes, ui);
        });
        self.vram_visible = visible;
    }

    pub fn draw(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if self.chr_visible || self.vram_visible {
            for i in 0..2 {
                let p = self.chr_palette[i];
                self.update_chr_image(nes, i, p);
            }
            self.draw_chr(nes, ui);
            self.draw_vram(nes, ui);
        }
    }
}
