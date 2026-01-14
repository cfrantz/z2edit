use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use python_gui::{Color, Image};
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::error::Error;
use crate::zelda2::chr::{ChrMemory, ChrSchema};
use crate::zelda2::enemies::config::EnemyGroup;
use crate::zelda2::items::{Items, Sprite};
use crate::zelda2::metatile::MetatileGroup;
use crate::zelda2::palette::PaletteGroup;
use crate::zelda2::project::Project;
use crate::zelda2::vchr::{self, VirtualChr};
use nes::{hwpalette, Address};

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

    fn maybe_remap(project: &Project, chrbank: u16, tile: i32, background: bool) -> Result<usize> {
        if let Ok(config) = project.config.get::<vchr::config::VirtualChr>("/vchr") {
            match config.schema {
                ChrSchema::Mmc5_1k => {
                    // When using MMC5 vbanks, we maintain a table of 1K banks
                    // mapped into the normal low/high PPU bank space.  Furthermore,
                    // if the tile is a background tile, MMC5 has a magical
                    // background-only bank.
                    //
                    // We look up the vchr bank for the classic chr bank number
                    // and then calculate the true 1K bank based on the tile ID.
                    // Finally, we calculate the actual memory offset based on
                    // the low bits of the tile ID.
                    let chrbank = chrbank as usize;
                    let vbanks =
                        project.data_ref::<VirtualChr>(&format!("/vchr/{}", chrbank / 2))?;
                    let t = tile as usize & 0xFF;
                    let subbank = if background {
                        t / 0x40 + 8
                    } else {
                        t / 0x40 + 4 * (chrbank & 1)
                    };
                    log::debug!(
                        "remapped chr{chrbank} to offset {subbank} -> {:02x}",
                        vbanks.data[subbank]
                    );
                    Ok(vbanks.data[subbank] as usize * 1024 + (t % 0x40) * 16)
                }
                _ => Err(Error::Configuration(format!(
                    "VBanks not implemented for schema {:?}",
                    config.schema
                ))
                .into()),
            }
        } else {
            // When there are no vbanks, the chrbank/tile calculation just calculates
            // the tile offset in a given bank.
            Ok(chrbank as usize * 4096 + (tile & 0xFF) as usize * 16)
        }
    }

    fn _render_tile(
        image: &mut Image,
        chrdata: &[u8],
        base: usize,
        palette: &[u8],
        xofs: u32,
        yofs: u32,
        tile: i32,
    ) {
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

    fn _render_metatile(
        project: &Project,
        chrdata: &[u8],
        chrbank: u16,
        palette: &[u8],
        tile: &[u8],
    ) -> Result<Image> {
        let mut image = Image::new(16, 16);
        const COORD: [(u32, u32); 4] = [(0, 0), (0, 8), (8, 0), (8, 8)];

        for i in 0..4 {
            let base = Self::maybe_remap(project, chrbank, tile[i] as i32, true)?;
            Self::_render_tile(
                &mut image,
                chrdata,
                base,
                palette,
                COORD[i].0,
                COORD[i].1,
                tile[i] as i32,
            );
        }
        image.update();
        Ok(image)
    }

    fn _render_one_sprite(
        project: &Project,
        image: &mut Image,
        chrdata: &[u8],
        chrbank: u16,
        palette: &[u8],
        xofs: u32,
        yofs: u32,
        sprite: i32,
    ) -> Result<()> {
        let bank_delta = (sprite & 1) as u16;
        let sprite = sprite & !1;
        let base = Self::maybe_remap(project, chrbank + bank_delta, sprite, false)?;
        Self::_render_tile(image, chrdata, base, palette, xofs, yofs, sprite);

        let base = Self::maybe_remap(project, chrbank + bank_delta, sprite + 1, false)?;
        Self::_render_tile(image, chrdata, base, palette, xofs, yofs + 8, sprite + 1);
        Ok(())
    }

    fn _render_sprite(
        project: &Project,
        chrdata: &[u8],
        chrbank: u16,
        palette: &[u8],
        sprite: &Sprite,
    ) -> Result<Image> {
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
                    Self::_render_one_sprite(
                        project, &mut image, chrdata, chrbank, palette, x, y, id,
                    )?;
                }
                i += 1;
                x += 8;
                last = id;
            }
            y += 16;
        }
        image.update();
        Ok(image)
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
                    Python::attach(|py| project.rom.borrow(py).read(town_table.palette + enemy))?
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
                    Self::_render_metatile(project, &*chrdata, key.chrbank, &key.palette, data)?
                }
                GfxKind::RawSprite(ref _address, ref data, ref _palette) => {
                    let mut image = Image::new(8, 16);
                    Self::_render_one_sprite(
                        project,
                        &mut image,
                        &*chrdata,
                        key.chrbank,
                        &key.palette,
                        0,
                        0,
                        *data as i32,
                    )?;
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
                        project,
                        &*chrdata,
                        key.chrbank,
                        &key.palette,
                        &data.to_be_bytes(),
                    )?
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
                        Python::attach(|py| -> Result<()> {
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
                    Self::_render_sprite(project, &*chrdata, key.chrbank, &key.palette, &sprite)?
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
                    Self::_render_sprite(project, &*chrdata, key.chrbank, &key.palette, sprite)?
                }
            };
            my.cache.insert(key.clone(), image);
        }
        Ok(my.cache.get(&key).unwrap())
    }
}
