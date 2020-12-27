use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::idpath;
use crate::nes::hwpalette;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::Edit;

const ERROR_BITMAP: [u32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Debug)]
pub enum Schema {
    Overworld(String, IdPath),
    MetaTile(String, IdPath, i32),
    Enemy(String, IdPath, i32),
    Item(String),
}

#[derive(Debug)]
pub struct TileCache {
    schema: Schema,
    edit: Rc<Edit>,
    cache: RefCell<HashMap<u8, Image>>,
    error: RefCell<Image>,
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
            error: RefCell::new(error_image),
            chr_override: None,
            pal_override: None,
        }
    }

    pub fn reset(&mut self, schema: Schema) {
        self.schema = schema;
        self.cache.borrow_mut().clear();
        self.chr_override = None;
        self.pal_override = None;
    }

    pub fn set_chr_override(&mut self, addr: Option<Address>) {
        self.cache.borrow_mut().clear();
        self.chr_override = addr;
    }

    pub fn set_pal_override(&mut self, addr: Option<Address>) {
        self.cache.borrow_mut().clear();
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
        for y in 0..8 {
            let a = rom.read(tileaddr + y)?;
            let b = rom.read(tileaddr + y + 8)?;
            for x in 0..8 {
                let pix = if mirror {
                    (((a >> x) & 0x01) << 0) | (((b >> x) & 0x01) << 1)
                } else {
                    (((a << x) & 0x80) >> 7) | (((b << x) & 0x80) >> 6)
                };
                let idx = rom.read(paladdr + pix)?;
                let color = if idx < 64 {
                    if idx == 0 && sprite {
                        0
                    } else {
                        hwpalette::get(idx as usize)
                    }
                } else {
                    0
                };
                let offset = (y0 + y) * image.width + (x0 + x);
                image.pixels[offset as usize] = color;
            }
        }
        Ok(())
    }

    fn get_overworld_tile(&self, tile: u8, config: &str, id: &IdPath) -> Result<Image> {
        let config = Config::get(config)?;
        let ov = config.overworld.find(id)?;

        let table = ov.objtable + tile * 4;
        let rom = self.edit.rom.borrow();
        let table = rom.read_bytes(table, 4)?;
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

    fn get_meta_tile(&self, tile: u8, config: &str, id: &IdPath, palidx: i32) -> Result<Image> {
        let config = Config::get(config)?;
        let sv = config.sideview.find(id)?;
        let group = tile >> 6;
        let tile = tile & 0x3f;

        let rom = self.edit.rom.borrow();
        let table = rom.read_pointer(sv.metatile_table + group * 2)?;
        let tiles = rom.read_bytes(table + tile * 4, 4)?;
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

    fn get_sprite(&self, tile: u8, config: &str, id: &IdPath, palidx: i32) -> Result<Image> {
        let config = Config::get(config)?;
        let rom = self.edit.rom.borrow();
        let palette = config.palette.find_sprite(id, palidx as usize)?;
        let (romaddr, sprite_info) = if id.at(0) == "item" {
            (Some(config.items.sprite_table), config.items.find(tile)?)
        } else {
            let (_, sprite) = config.enemy.find_by_index(id, tile)?;
            (None, sprite)
        };

        let mut rom_sprites = [0i32, 0i32];
        let sprites = if sprite_info.sprites.is_empty() {
            if let Some(addr) = romaddr {
                let table = rom.read_bytes(addr + tile * 2, 2)?;
                rom_sprites[0] = table[0] as i32;
                rom_sprites[1] = table[1] as i32;
                &rom_sprites
            } else {
                return Err(ErrorKind::NotFound(format!("sprite {}", id)).into());
            }
        } else {
            &sprite_info.sprites[..]
        };

        let (width, height) = sprite_info.size;
        let mut image = Image::new(width, height);
        let width = width / 8;
        let height = height / 16;
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let sprite_id = sprites[(y * width + x) as usize];
                if sprite_id == -1 {
                    x += 1;
                    continue;
                }
                let mirror = sprite_id & 0x0100_0000 != 0
                    || (x & 1 == 1 && sprite_id == sprites[(y * width + x - 1) as usize]);
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

    fn _get(&self, tile: u8) -> Result<Ref<'_, Image>> {
        {
            let mut cache = self.cache.borrow_mut();
            if !cache.contains_key(&tile) {
                let image = match &self.schema {
                    Schema::Overworld(config, id) => self.get_overworld_tile(tile, config, id)?,
                    Schema::MetaTile(config, id, palidx) => {
                        self.get_meta_tile(tile, config, id, *palidx)?
                    }
                    Schema::Enemy(config, id, palidx) => {
                        self.get_sprite(tile, config, id, *palidx)?
                    }
                    Schema::Item(config) => self.get_sprite(tile, config, &idpath!("item"), 0)?,
                };
                cache.insert(tile, image);
            }
        }
        let cache = self.cache.borrow();
        Ok(Ref::map(cache, |c| c.get(&tile).unwrap()))
    }

    pub fn get(&self, tile: u8) -> Ref<'_, Image> {
        match self._get(tile) {
            Ok(v) => v,
            Err(e) => {
                error!("TileCache: could not look up 0x{:02x}: {:?}", tile, e);
                self.error.borrow()
            }
        }
    }
}
