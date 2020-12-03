use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::nes::hwpalette;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::Edit;

#[derive(Debug)]
pub enum Schema {
    Overworld(String, IdPath),
}

#[derive(Debug)]
pub struct TileCache {
    schema: Schema,
    edit: Rc<Edit>,
    cache: RefCell<HashMap<u8, Image>>,
}

impl TileCache {
    pub fn new(edit: &Rc<Edit>, schema: Schema) -> Self {
        TileCache {
            schema: schema,
            edit: Rc::clone(edit),
            cache: RefCell::new(HashMap::<u8, Image>::new()),
        }
    }

    pub fn reset(&mut self, schema: Schema) {
        self.schema = schema;
        self.cache.borrow_mut().clear();
    }

    fn blit(
        &self,
        image: &mut Image,
        tileaddr: Address,
        paladdr: Address,
        x0: u32,
        y0: u32,
    ) -> Result<()> {
        let rom = self.edit.rom.borrow();
        for y in 0..8 {
            let a = rom.read(tileaddr + y)?;
            let b = rom.read(tileaddr + y + 8)?;
            for x in 0..8 {
                let pix = (((a << x) & 0x80) >> 7) | (((b << x) & 0x80) >> 6);
                let idx = rom.read(paladdr + pix)?;
                let color = if idx < 64 {
                    hwpalette::get(idx as usize)
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

        let mut image = Image::new(16, 16);
        self.blit(
            &mut image,
            ov.chr.set_val(table[0] as usize * 16),
            ov.palette + palidx * 4,
            0,
            0,
        )?;
        self.blit(
            &mut image,
            ov.chr.set_val(table[1] as usize * 16),
            ov.palette + palidx * 4,
            0,
            8,
        )?;
        self.blit(
            &mut image,
            ov.chr.set_val(table[2] as usize * 16),
            ov.palette + palidx * 4,
            8,
            0,
        )?;
        self.blit(
            &mut image,
            ov.chr.set_val(table[3] as usize * 16),
            ov.palette + palidx * 4,
            8,
            8,
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

    fn _get(&self, tile: u8) -> Result<Ref<'_, Image>> {
        {
            let mut cache = self.cache.borrow_mut();
            if !cache.contains_key(&tile) {
                let image = match &self.schema {
                    Schema::Overworld(config, id) => self.get_overworld_tile(tile, config, id)?,
                };
                cache.insert(tile, image);
            }
        }
        let cache = self.cache.borrow();
        Ok(Ref::map(cache, |c| c.get(&tile).unwrap()))
    }

    pub fn get(&self, tile: u8) -> Ref<'_, Image> {
        self._get(tile).unwrap()
    }
}
