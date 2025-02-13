use anyhow::{anyhow, Result};
use python_gui::{Color, Image};
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::error::Error;
use crate::nes::{hwpalette, Address};
use crate::zelda2::chr::ChrMemory;
use crate::zelda2::palette::PaletteGroup;
use crate::zelda2::project::Project;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GfxKind {
    Tile = 1,
    Sprite = 2,
    Metatile = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GfxKey {
    chrbank: u16,
    kind: GfxKind,
    tiles: [u8; 4],
    palette: [u8; 4],
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

    fn _render(chrdata: &[u8], key: &GfxKey) -> Image {
        let (width, height) = match key.kind {
            GfxKind::Tile => (8, 8),
            GfxKind::Sprite => (8, 16),
            GfxKind::Metatile => (16, 16),
        };
        let base = key.chrbank as usize * 4096;
        let mut image = Image::new(width, height);

        for (i, &tile) in key.tiles[0..(key.kind as usize)].iter().enumerate() {
            let xofs = (i as u32 & 2) * 4;
            let yofs = (i as u32 & 1) * 8;
            let base = base + (tile as usize * 16);
            for y in 0..8 {
                let mut lo = chrdata[base + y].reverse_bits() as usize;
                let mut hi = chrdata[base + y + 8].reverse_bits() as usize;
                for x in 0..8 {
                    let color = (lo & 1) + ((hi & 1) << 1);
                    let color = key.palette[color];
                    let color = Color::new(hwpalette::get(color as usize));
                    image.set_pixel(xofs + (x as u32), yofs + (y as u32), color);
                    lo >>= 1;
                    hi >>= 1;
                }
            }
        }
        image.update();
        image
    }

    fn _get(&mut self, chrdata: &[u8], key: GfxKey) -> &Image {
        if !self.cache.contains_key(&key) {
            let image = Self::_render(chrdata, &key);
            self.cache.insert(key.clone(), image);
        }
        self.cache.get(&key).unwrap()
    }

    fn get<'a>(
        project: &'a Project,
        chrbank: u16,
        palette_group: &str,
        group: &str,
        palette: usize,
        kind: GfxKind,
        tiles: [u8; 4],
    ) -> Result<&'a Image> {
        let cache = Self::project(project);
        let pgroup = project.data_ref::<PaletteGroup>(palette_group)?;
        let paldata = pgroup
            .group
            .get(group)
            .ok_or_else(|| Error::NotFound(format!("{palette_group}/{group}")))?;
        let chrdata = project.data_ref::<ChrMemory>("/chr")?;
        let palette = palette * 4;

        let data = chrdata.data.lock().unwrap();
        Ok(cache._get(
            data.as_slice(),
            GfxKey {
                chrbank,
                kind,
                tiles,
                palette: paldata[palette..palette + 4].try_into()?,
            },
        ))
    }

    pub fn raw<'a>(
        project: &'a Project,
        chr: Address,
        palette_group: &str,
        group: &str,
        palette: usize,
        tiles: &[u8],
    ) -> Result<&'a Image> {
        if !chr.is_chr() {
            return Err(anyhow!("TileCache::raw {chr:?} is not a CHR address"));
        }
        let (kind, buf) = match tiles.len() {
            1 => (GfxKind::Tile, [tiles[0], 0, 0, 0]),
            2 => (GfxKind::Sprite, [tiles[0], tiles[1], 0, 0]),
            4 => (GfxKind::Metatile, [tiles[0], tiles[1], tiles[2], tiles[3]]),
            _ => {
                return Err(
                    Error::NotImplemented(format!("GfxCache tile length {}", tiles.len())).into(),
                )
            }
        };
        Self::get(
            project,
            chr.bank().unwrap() as u16,
            palette_group,
            group,
            palette,
            kind,
            buf,
        )
    }

    pub fn sprite<'a>(
        project: &'a Project,
        chr: Address,
        palette_group: &str,
        group: &str,
        palette: usize,
        sprite: u8,
    ) -> Result<&'a Image> {
        if !chr.is_chr() {
            return Err(anyhow!("TileCache::raw {chr:?} is not a CHR address"));
        }
        let bank_delta = (sprite & 1) as u16;
        let sprite = sprite & !1;
        let chrbank = (chr.bank().unwrap() as u16 & !1) + bank_delta;
        Self::get(
            project,
            chrbank,
            palette_group,
            group,
            palette,
            GfxKind::Sprite,
            [sprite, sprite + 1, 0, 0],
        )
    }
}
