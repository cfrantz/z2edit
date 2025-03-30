use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use python_gui::{Color, Image};
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::error::Error;
use crate::nes::{hwpalette, Address};
use crate::zelda2::chr::ChrMemory;
use crate::zelda2::enemies::config::EnemyGroup;
use crate::zelda2::items::{Items, Sprite};
use crate::zelda2::metatile::MetatileGroup;
use crate::zelda2::palette::PaletteGroup;
use crate::zelda2::project::Project;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GfxKind {
    RawTile(Address, [u8; 4], u8),
    RawSprite(Address, u8, u8),
    Metatile(Address, String, u8),
    Enemy(String, u16),
    Item(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GfxKey {
    chrbank: u16,
    palette: [u8; 4],
    kind: GfxKind,
}

static mut CACHE: OnceLock<HashMap<String, GfxCache>> = OnceLock::new();

#[derive(Default)]
pub struct GfxCache {
    cache: HashMap<GfxKey, Image>,
}

impl GfxCache {
    fn project(project: &Project) -> &'static mut GfxCache {
        let cache = unsafe {
            let _ = CACHE.get_or_init(Default::default);
            CACHE.get_mut().unwrap()
        };
        if !cache.contains_key(&project.name) {
            cache.insert(project.name.clone(), GfxCache::default());
        }
        cache.get_mut(&project.name).unwrap()
    }

    pub fn clear(project: &Project) {
        let cache = Self::project(project);
        cache.cache.clear();
    }

    fn _render_tile(
        image: &mut Image,
        chrdata: &[u8],
        chrbank: u16,
        palette: &[u8],
        xofs: u32,
        yofs: u32,
        tile: i32,
    ) {
        let base = chrbank as usize * 4096 + (tile & 0xFF) as usize * 16;
        let ty = ((tile >> 8) & 0xFF) as u32;
        let tx = ((tile >> 16) & 0xFF) as u32;
        let mirror = tile & 0x1000000 != 0;
        for y in 0..8 {
            let mut lo = chrdata[base + y];
            let mut hi = chrdata[base + y + 8];
            if !mirror {
                lo = lo.reverse_bits();
                hi = hi.reverse_bits();
            }
            for x in 0..8 {
                let color = (lo & 1) + ((hi & 1) << 1);
                let color = palette[color as usize];
                let color = Color::new(hwpalette::get(color as usize));
                image.set_pixel(xofs + tx + (x as u32), yofs + ty + (y as u32), color);
                lo >>= 1;
                hi >>= 1;
            }
        }
    }

    fn _render_metatile(chrdata: &[u8], chrbank: u16, palette: &[u8], tile: &[u8]) -> Image {
        let mut image = Image::new(16, 16);
        Self::_render_tile(&mut image, chrdata, chrbank, palette, 0, 0, tile[0] as i32);
        Self::_render_tile(&mut image, chrdata, chrbank, palette, 0, 8, tile[1] as i32);
        Self::_render_tile(&mut image, chrdata, chrbank, palette, 8, 0, tile[2] as i32);
        Self::_render_tile(&mut image, chrdata, chrbank, palette, 8, 8, tile[3] as i32);
        image.update();
        image
    }

    fn _render_one_sprite(
        image: &mut Image,
        chrdata: &[u8],
        chrbank: u16,
        palette: &[u8],
        xofs: u32,
        yofs: u32,
        sprite: i32,
    ) {
        let bank_delta = (sprite & 1) as u16;
        let sprite = sprite & !1;
        Self::_render_tile(
            image,
            chrdata,
            chrbank + bank_delta,
            palette,
            xofs,
            yofs,
            sprite,
        );
        Self::_render_tile(
            image,
            chrdata,
            chrbank + bank_delta,
            palette,
            xofs,
            yofs + 8,
            sprite + 1,
        );
    }

    fn _render_sprite(chrdata: &[u8], chrbank: u16, palette: &[u8], sprite: &Sprite) -> Image {
        let mut image = Image::new(sprite.size[0], sprite.size[1]);
        let mut y = 0;
        let mut i = 0;
        while y < sprite.size[1] {
            let mut x = 0;
            let mut last = -1;
            while x < sprite.size[0] {
                let mut id = sprite.sprites.get(i).copied().unwrap_or(-1);
                if id != -1 {
                    if id == last {
                        // If its the same as the last sprite, mirror it.
                        id |= 0x0100_0000;
                    }
                    Self::_render_one_sprite(&mut image, chrdata, chrbank, palette, x, y, id);
                }
                i += 1;
                x += 8;
                last = id;
            }
            y += 16;
        }
        image.update();
        image
    }

    pub fn get<'a>(
        project: &'a Project,
        palette_group: &str,
        group: &str,
        kind: GfxKind,
    ) -> Result<&'a Image> {
        let pgroup = project.data_ref::<PaletteGroup>(palette_group)?;
        let paldata = pgroup
            .group
            .get(group)
            .ok_or_else(|| Error::NotFound(format!("{palette_group}/{group}")))?;
        let key = match kind {
            GfxKind::RawTile(ref address, ref _data, ref palette) => {
                if !address.is_chr() {
                    return Err(anyhow!("TileCache::get {address:?} is not a CHR address"));
                }
                let palette = *palette as usize * 4;
                GfxKey {
                    chrbank: address.bank().unwrap() as u16,
                    palette: paldata[palette..palette + 4].try_into()?,
                    kind,
                }
            }
            GfxKind::RawSprite(ref address, ref _data, ref palette) => {
                if !address.is_chr() {
                    return Err(anyhow!("TileCache::get {address:?} is not a CHR address"));
                }
                let palette = *palette as usize * 4;
                GfxKey {
                    chrbank: address.bank().unwrap() as u16,
                    palette: paldata[palette..palette + 4].try_into()?,
                    kind,
                }
            }
            GfxKind::Metatile(ref address, ref meta_group, ref tile) => {
                if !address.is_chr() {
                    return Err(anyhow!("TileCache::get {address:?} is not a CHR address"));
                }
                let meta_group = project.data_ref::<MetatileGroup>(meta_group)?;
                let gindex = *tile as usize >> 6;
                let group = meta_group
                    .group
                    .get(&gindex)
                    .ok_or_else(|| anyhow!("No metatile group for tile {tile:02x}"))?;
                let palette = group
                    .palette
                    .get((*tile & 0x3f) as usize)
                    .map(|x| *x as usize)
                    .unwrap_or(gindex)
                    * 4;
                GfxKey {
                    chrbank: address.bank().unwrap() as u16,
                    palette: paldata[palette..palette + 4].try_into()?,
                    kind,
                }
            }
            GfxKind::Enemy(ref enemy_group, ref enemy) => {
                let group = project.config.get::<EnemyGroup>(enemy_group)?;
                let enemy = *enemy as u8;
                let sprite = group
                    .group
                    .get(&enemy)
                    .ok_or_else(|| anyhow!("No sprite for enemy {enemy}"))?;
                let palette = if let Some(town_table) = &group.town_table {
                    // FIXME: This reads from the ROM rather than an abstract data structure in the
                    // project.
                    Python::with_gil(|py| project.rom.borrow(py).read(town_table.palette + enemy))?
                        & 0x03
                } else {
                    sprite.palette
                };
                let palette = palette as usize * 4;

                GfxKey {
                    chrbank: sprite.chr.bank().unwrap() as u16,
                    palette: paldata[palette..palette + 4].try_into()?,
                    kind,
                }
            }
            GfxKind::Item(ref item) => {
                let path = if *item < 128 {
                    format!("/global/item/{item}")
                } else {
                    format!("/global/item/fake/{item}")
                };
                let sprite = project.config.get::<Sprite>(&path)?;
                let palette = sprite.palette as usize * 4;
                GfxKey {
                    chrbank: sprite.chr.bank().unwrap() as u16,
                    palette: paldata[palette..palette + 4].try_into()?,
                    kind,
                }
            }
        };

        let my = Self::project(project);
        let chr_memory = project.data_ref::<ChrMemory>("/chr")?;
        let chrdata = chr_memory.data.lock().unwrap();
        if !my.cache.contains_key(&key) {
            let image = match key.kind {
                GfxKind::RawTile(ref _address, ref data, ref _palette) => {
                    Self::_render_metatile(&*chrdata, key.chrbank, &key.palette, data)
                }
                GfxKind::RawSprite(ref _address, ref data, ref _palette) => {
                    let mut image = Image::new(8, 16);
                    Self::_render_one_sprite(
                        &mut image,
                        &*chrdata,
                        key.chrbank,
                        &key.palette,
                        0,
                        0,
                        *data as i32,
                    );
                    image.update();
                    image
                }
                GfxKind::Metatile(ref _address, ref meta_group, ref tile) => {
                    let mgroup = project.data_ref::<MetatileGroup>(meta_group)?;
                    let gindex = *tile as usize >> 6;
                    let group = mgroup
                        .group
                        .get(&gindex)
                        .ok_or_else(|| anyhow!("No metatile group for tile {tile:02x}"))?;
                    let data = group
                        .tile
                        .get((*tile & 0x3f) as usize)
                        .copied()
                        .ok_or_else(|| Error::NotFound(format!("Metatile {meta_group}/{tile}")))?;
                    Self::_render_metatile(
                        &*chrdata,
                        key.chrbank,
                        &key.palette,
                        &data.to_be_bytes(),
                    )
                }
                GfxKind::Enemy(ref enemy_group, ref enemy) => {
                    let group = project.config.get::<EnemyGroup>(enemy_group)?;
                    let town_code = (*enemy >> 8) as usize;
                    let enemy = *enemy as u8;
                    let mut sprite = group
                        .group
                        .get(&enemy)
                        .cloned()
                        .ok_or_else(|| anyhow!("No sprite for enemy {enemy}"))?;
                    if let Some(town_table) = &group.town_table {
                        // FIXME: This reads from the ROM rather than an abstract data structure
                        // in the project.
                        Python::with_gil(|py| -> Result<()> {
                            let rom = project.rom.borrow(py);
                            let index = match enemy {
                                13..27 => rom.read(town_table.mapping2[town_code] + enemy - 13)?,
                                _ => rom.read(town_table.mapping + enemy)?,
                            };
                            for s in rom.read_bytes(town_table.table + (index & 0x7f), 4)? {
                                sprite.sprites.push(*s as i32);
                            }
                            Ok(())
                        })?;
                    }
                    Self::_render_sprite(&*chrdata, key.chrbank, &key.palette, &sprite)
                }
                GfxKind::Item(ref item) => {
                    let sprite = if *item < 128 {
                        let items = project.data_ref::<Items>("/global/item")?;
                        items
                            .item
                            .get(item)
                            .ok_or_else(|| Error::NotFound(format!("Item /global/items/{item}")))?
                    } else {
                        project
                            .config
                            .get::<Sprite>(&format!("/global/item/fake/{item}"))?
                    };
                    Self::_render_sprite(&*chrdata, key.chrbank, &key.palette, sprite)
                }
            };
            my.cache.insert(key.clone(), image);
        }
        Ok(my.cache.get(&key).unwrap())
    }
}
