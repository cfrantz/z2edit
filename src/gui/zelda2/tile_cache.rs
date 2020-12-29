use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::nes::hwpalette;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::items::config::Sprite;
use crate::zelda2::palette::config::Palette;
use crate::zelda2::project::Edit;

const ERROR_BITMAP: [u32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Debug)]
pub enum Schema {
    None,
    Overworld(String, IdPath),
    MetaTile(String, IdPath, i32),
    Enemy(String, IdPath, i32),
    Item(String, IdPath),
    RawSprite(String, Address, IdPath, u8),
    RawTile(Address, [u32; 4]),
}

#[derive(Debug)]
pub struct TileCache {
    schema: Schema,
    edit: Rc<Edit>,
    cache: RefCell<HashMap<u8, Image>>,
    error: Image,
    error_ref: RefCell<Image>,
    chr_override: Option<Address>,
    pal_override: Option<Address>,
}

impl TileCache {
    pub fn new(edit: &Rc<Edit>, schema: Schema) -> Self {
        let mut error_image = Image::new(8, 8);
        for (i, val) in ERROR_BITMAP.iter().enumerate() {
            error_image.pixels[i] = if *val == 1 { 0xFF0000FF } else { 0xFF000000 };
        }
        error_image.update();

        TileCache {
            schema: schema,
            edit: Rc::clone(edit),
            cache: RefCell::new(HashMap::<u8, Image>::new()),
            error: error_image.clone(),
            error_ref: RefCell::new(error_image),
            chr_override: None,
            pal_override: None,
        }
    }

    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }

    pub fn reset(&mut self, schema: Schema) {
        self.schema = schema;
        self.clear();
        self.chr_override = None;
        self.pal_override = None;
    }

    pub fn set_chr_override(&mut self, addr: Option<Address>) {
        self.clear();
        self.chr_override = addr;
    }

    pub fn set_pal_override(&mut self, addr: Option<Address>) {
        self.clear();
        self.pal_override = addr;
    }

    fn blit(
        &self,
        image: &mut Image,
        tileaddr: Address,
        paladdr: Address,
        x0: u32,
        y0: u32,
        mirror: bool,
        sprite: bool,
    ) -> Result<()> {
        let rom = self.edit.rom.borrow();
        let mut palette = [0u32; 4];
        for (c, &i) in palette.iter_mut().zip(rom.read_bytes(paladdr, 4)?.iter()) {
            *c = if i < 64 {
                hwpalette::get(i as usize)
            } else {
                0
            };
        }
        self.blit_raw(image, tileaddr, &palette, x0, y0, mirror, sprite)
    }

    fn blit_raw(
        &self,
        image: &mut Image,
        tileaddr: Address,
        palette: &[u32],
        x0: u32,
        y0: u32,
        mirror: bool,
        sprite: bool,
    ) -> Result<()> {
        let rom = self.edit.rom.borrow();
        for y in 0..8 {
            let a = rom.read(tileaddr + y)?;
            let b = rom.read(tileaddr + y + 8)?;
            for x in 0..8 {
                let pix = if mirror {
                    (((a >> x) & 0x01) << 0) | (((b >> x) & 0x01) << 1)
                } else {
                    (((a << x) & 0x80) >> 7) | (((b << x) & 0x80) >> 6)
                };
                let color = if sprite && pix == 0 {
                    0
                } else {
                    palette[pix as usize]
                };
                let offset = (y0 + y) * image.width + (x0 + x);
                image.pixels[offset as usize] = color;
            }
        }
        Ok(())
    }

    fn unblit(
        &self,
        image: &Image,
        tileaddr: Address,
        palette: &[u32],
        x0: u32,
        y0: u32,
    ) -> Result<()> {
        let mut rom = self.edit.rom.borrow_mut();
        for y in 0..8 {
            let mut a = 0u8;
            let mut b = 0u8;
            for x in 0..8 {
                let offset = (y0 + y) * image.width + (x0 + x);
                let color = image.pixels[offset as usize];
                let mut index = 0;
                for (i, c) in palette.iter().enumerate() {
                    if color == *c {
                        index = i;
                    };
                }

                a |= if index & 1 == 0 { 0 } else { 1 << (7 - x) };
                b |= if index & 2 == 0 { 0 } else { 1 << (7 - x) };
            }
            rom.write(tileaddr + y, a)?;
            rom.write(tileaddr + y + 8, b)?;
        }
        Ok(())
    }

    fn get_raw_tile(&self, tile: u8, chr: Address, palette: &[u32]) -> Result<Image> {
        let chr = self.chr_override.unwrap_or(chr);
        let mut image = Image::new(8, 8);
        self.blit_raw(
            &mut image,
            chr + tile as usize * 16,
            palette,
            0,
            0,
            false,
            false,
        )?;
        image.update();
        Ok(image)
    }

    fn get_overworld_tile(
        &self,
        tile: u8,
        config: &str,
        id: &IdPath,
        table: Option<&[u8]>,
    ) -> Result<Image> {
        let config = Config::get(config)?;
        let ov = config.overworld.find(id)?;

        let rom = self.edit.rom.borrow();
        let table = table.unwrap_or(rom.read_bytes(ov.objtable + tile as usize * 4, 4)?);
        let palidx = rom.read(ov.tile_palette + tile)?;

        let chr = self.chr_override.unwrap_or(ov.chr);
        let palette = self.pal_override.unwrap_or(ov.palette);
        let mut image = Image::new(16, 16);
        self.blit(
            &mut image,
            chr.set_val(table[0] as usize * 16),
            palette + palidx * 4,
            0,
            0,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr.set_val(table[1] as usize * 16),
            palette + palidx * 4,
            0,
            8,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr.set_val(table[2] as usize * 16),
            palette + palidx * 4,
            8,
            0,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr.set_val(table[3] as usize * 16),
            palette + palidx * 4,
            8,
            8,
            false,
            false,
        )?;

        // If the tile is walkable water, darken it so it's visible as a
        // distinct entity on the map.
        if tile == 13 {
            for p in image.pixels.iter_mut() {
                *p = (*p >> 1) & 0x7f7f7f7f | 0xFF000000;
            }
        }
        image.update();
        Ok(image)
    }

    fn get_meta_tile(
        &self,
        tile: u8,
        config: &str,
        id: &IdPath,
        palidx: i32,
        table: Option<&[u8]>,
    ) -> Result<Image> {
        let config = Config::get(config)?;
        let sv = config.sideview.find(id)?;
        let group = tile >> 6;
        let tile = tile & 0x3f;

        let rom = self.edit.rom.borrow();
        let ptr = rom.read_pointer(sv.metatile_table + group * 2)?;
        let tiles = table.unwrap_or(rom.read_bytes(ptr + tile * 4, 4)?);
        let palidx = palidx as usize * 16;

        let chr = self.chr_override.unwrap_or(sv.chr);
        let palette = self.pal_override.unwrap_or(sv.palette + palidx);
        let mut image = Image::new(16, 16);
        self.blit(
            &mut image,
            chr + tiles[0] as usize * 16,
            palette + group * 4,
            0,
            0,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr + tiles[1] as usize * 16,
            palette + group * 4,
            0,
            8,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr + tiles[2] as usize * 16,
            palette + group * 4,
            8,
            0,
            false,
            false,
        )?;
        self.blit(
            &mut image,
            chr + tiles[3] as usize * 16,
            palette + group * 4,
            8,
            8,
            false,
            false,
        )?;

        image.update();
        Ok(image)
    }

    fn get_raw_sprite(
        &self,
        tile: u8,
        config: &str,
        chr: Address,
        id: &IdPath,
        palidx: u8,
    ) -> Result<Image> {
        let config = Config::get(config)?;
        let palette = config.palette.find(id)?;
        let sprite_info = Sprite {
            chr: chr,
            palette: palidx,
            size: (8, 16),
            sprites: vec![tile as i32],
            ..Default::default()
        };
        self.get_sprite_helper(&sprite_info, palette)
    }

    fn get_enemy_sprite(&self, tile: u8, config: &str, id: &IdPath, palidx: i32) -> Result<Image> {
        let config = Config::get(config)?;
        let palette = config.palette.find_sprite(id, palidx as usize)?;
        let (_, sprite_info) = config.enemy.find_by_index(id, tile)?;
        self.get_sprite_helper(sprite_info, palette)
    }

    fn get_item_sprite(&self, tile: u8, config: &str, id: &IdPath, palidx: i32) -> Result<Image> {
        let config = Config::get(config)?;
        let palette = config.palette.find_sprite(id, palidx as usize)?;
        let mut sprite_info = config.items.find(tile)?.clone();

        if sprite_info.sprites.is_empty() {
            let addr = config.items.sprite_table + tile * 2;
            let rom = self.edit.rom.borrow();
            let table = rom.read_bytes(addr, 2)?;
            sprite_info.sprites.push(table[0] as i32);
            sprite_info.sprites.push(table[1] as i32);
        }
        self.get_sprite_helper(&sprite_info, palette)
    }

    fn get_sprite_helper(&self, sprite_info: &Sprite, palette: &Palette) -> Result<Image> {
        let (width, height) = sprite_info.size;
        let mut image = Image::new(width, height);
        let width = width / 8;
        let height = height / 16;
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let sprite_id = sprite_info.sprites[(y * width + x) as usize];
                if sprite_id == -1 {
                    x += 1;
                    continue;
                }
                let mirror = sprite_id & 0x0100_0000 != 0
                    || (x & 1 == 1
                        && sprite_id == sprite_info.sprites[(y * width + x - 1) as usize]);
                let xdelta = (sprite_id as u32 >> 16) & 0xFF;
                let ydelta = (sprite_id as u32 >> 8) & 0xFF;
                let tile = sprite_id & 0xFE;
                let chr = self.chr_override.unwrap_or(sprite_info.chr);
                let chr = chr.add_bank(sprite_id as isize & 1);
                self.blit(
                    &mut image,
                    chr + tile * 16,
                    palette.address + sprite_info.palette * 4,
                    x * 8 + xdelta,
                    y * 16 + ydelta,
                    mirror,
                    true,
                )?;
                self.blit(
                    &mut image,
                    chr + (tile + 1) * 16,
                    palette.address + sprite_info.palette * 4,
                    x * 8 + xdelta,
                    y * 16 + ydelta + 8,
                    mirror,
                    true,
                )?;
                x += 1;
            }
            y += 1;
        }
        image.update();
        Ok(image)
    }

    fn _get_raw(&self, tile: u8, table: Option<&[u8]>) -> Result<Image> {
        let image = match &self.schema {
            Schema::None => {
                return Err(ErrorKind::NotFound("Schema::None".to_string()).into());
            }
            Schema::Overworld(config, id) => self.get_overworld_tile(tile, config, id, table)?,
            Schema::MetaTile(config, id, palidx) => {
                self.get_meta_tile(tile, config, id, *palidx, table)?
            }
            Schema::Enemy(config, id, palidx) => {
                self.get_enemy_sprite(tile, config, id, *palidx)?
            }
            Schema::Item(config, area_id) => self.get_item_sprite(tile, config, area_id, 0)?,
            Schema::RawSprite(config, chr, pal_id, palette) => {
                self.get_raw_sprite(tile, config, *chr, pal_id, *palette)?
            }
            Schema::RawTile(chr, palette) => self.get_raw_tile(tile, *chr, palette)?,
        };
        Ok(image)
    }

    fn _get_cached(&self, tile: u8, table: Option<&[u8]>) -> Result<Ref<'_, Image>> {
        {
            let mut cache = self.cache.borrow_mut();
            if !cache.contains_key(&tile) {
                let image = self._get_raw(tile, table)?;
                cache.insert(tile, image);
            }
        }
        let cache = self.cache.borrow();
        Ok(Ref::map(cache, |c| c.get(&tile).unwrap()))
    }

    pub fn get(&self, tile: u8) -> Ref<'_, Image> {
        match self._get_cached(tile, None) {
            Ok(v) => v,
            Err(e) => {
                error!("TileCache: could not look up 0x{:02x}: {:?}", tile, e);
                self.error_ref.borrow()
            }
        }
    }

    pub fn get_alternate(&self, tile: u8, table: &[u8]) -> Ref<'_, Image> {
        match self._get_cached(tile, Some(table)) {
            Ok(v) => v,
            Err(e) => {
                error!("TileCache: could not look up 0x{:02x}: {:?}", tile, e);
                self.error_ref.borrow()
            }
        }
    }

    pub fn get_alternate_uncached(&self, tile: u8, table: &[u8]) -> Cow<'_, Image> {
        match self._get_raw(tile, Some(table)) {
            Ok(i) => Cow::Owned(i),
            Err(e) => {
                error!("TileCache: could not look up 0x{:02x}: {:?}", tile, e);
                Cow::Borrowed(&self.error)
            }
        }
    }

    pub fn get_bank(
        &self,
        chr: Address,
        palette: &[u32],
        sprite_layout: bool,
        border: u32,
    ) -> Result<Image> {
        let width = border + 16 * (8 + border);
        let height = if sprite_layout {
            border + 8 * (16 + border)
        } else {
            border + 16 * (8 + border)
        };
        let mut image = Image::new_with_color(width, height, 0xFFFF00FF);
        self.bank_worker(sprite_layout, border, &mut |tile, x, y| {
            self.blit_raw(&mut image, chr + tile * 16, palette, x, y, false, false)
        })?;
        image.update();
        Ok(image)
    }

    pub fn put_bank(
        &self,
        image: &Image,
        chr: Address,
        palette: &[u32],
        sprite_layout: bool,
        border: u32,
    ) -> Result<()> {
        let width = border + 16 * (8 + border);
        let height = if sprite_layout {
            border + 8 * (16 + border)
        } else {
            border + 16 * (8 + border)
        };
        if image.width != width || image.height != height {
            return Err(ErrorKind::TransformationError(format!(
                "Image size ({}, {}) does not match expected size of ({}, {})",
                image.width, image.height, width, height
            ))
            .into());
        }
        self.bank_worker(sprite_layout, border, &mut |tile, x, y| {
            self.unblit(&image, chr + tile * 16, palette, x, y)
        })
    }

    fn bank_worker<F>(&self, sprite_layout: bool, border: u32, work: &mut F) -> Result<()>
    where
        F: FnMut(u32, u32, u32) -> Result<()>,
    {
        if sprite_layout {
            for y in 0..8 {
                for x in 0..16 {
                    let tile = y * 32 + x * 2;
                    work(tile, border + x * (8 + border), border + y * (16 + border))?;
                    work(
                        tile + 1,
                        border + x * (8 + border),
                        border + y * (16 + border) + 8,
                    )?;
                }
            }
        } else {
            for y in 0..16 {
                for x in 0..16 {
                    let tile = y * 16 + x;
                    work(tile, border + x * (8 + border), border + y * (8 + border))?;
                }
            }
        }
        Ok(())
    }
}
