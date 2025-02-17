use anyhow::{bail, ensure, Result};
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, AddressRange, Alloc, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PalaceDetail {
    pub index: usize,
    pub chr_bank: u8,
    pub palette: u8,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub x: u8,
    pub y: u8,
    pub area: u8,
    pub screen: u8,
    pub entry_right: bool,
    pub dest_world: u8,
    pub dest_overworld: u8,
    pub external: bool,
    pub second: bool,
    pub exit_2_lower: bool,
    pub passthru: bool,
    pub fall: bool,
    pub hidden: Option<bool>,
    pub palace: Option<PalaceDetail>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Overworld {
    pub compress_boulder: Option<bool>,
    pub compress_spider: Option<bool>,
    pub map: Map,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub connection: IndexMap<u8, Connector>,
}

#[typetag::serde]
impl GameData for Overworld {
    fn name(&self) -> String {
        "Overworld".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::overworld::OverworldEditor::new(self, name)
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Overworld {
        pub name: String,
        pub overworld: u8,
        pub subworld: u8,
        pub pointer: Address,
        pub connector: Address,
        pub metatile: String,
        pub palette: String,
        pub config: String,
        pub chr: Address,
        pub width: usize,
        pub height: usize,
        pub raft_connector: usize,
        pub raft_table: Address,
        pub palace: PalaceDetail,
        pub hidden: Vec<HiddenSpot>,
        pub consts: OverworldConsts,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct PalaceDetail {
        pub stone_table: Address,
        pub chr_table: Address,
        pub palette_table: Address,
        pub length: usize,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct HiddenSpot {
        pub kind: HiddenKind,
        pub connector: Address,
        pub overworld: Address,
        pub x: Address,
        pub y: Address,
        pub return_y: Address,
        pub ppu_macro: Address,
        pub discriminator: Address,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct OverworldConsts {
        pub y_offset: u8,
        pub palace_connectors: Vec<u8>,
        pub town_connectors: Vec<u8>,
        pub ram: AddressRange,
    }

    impl Default for OverworldConsts {
        fn default() -> Self {
            OverworldConsts {
                y_offset: 30,
                palace_connectors: vec![52, 53, 54, 55],
                town_connectors: vec![44, 45, 46, 47, 48, 49, 50, 51],
                ram: AddressRange {
                    address: Address::Cpu(0x7c00),
                    length: 896,
                },
            }
        }
    }
}

impl config::Overworld {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Overworld::unpack {path}");
        let rom = rrom.borrow();
        let mut ov = Overworld::new(self);
        ov.decompress(self, &*rom)?;
        for index in 0..63 {
            ov.connection
                .insert(index, self.get_connector(index, &*rom)?);
        }
        edits.insert(path.into(), Edit::new(ov.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Overworld::pack {path}");
        if let Some(edit) = edits.get(path) {
            let mut rom = rrom.borrow_mut();
            let ov = edit.data_ref::<Overworld>()?;
            let map = ov.compress(self)?;
            if map.data.len() > self.consts.ram.length as usize {
                return Err(Error::Length(format!(
                    "overworld too big: compressed size of {} bytes is larger than {} bytes",
                    map.data.len(),
                    self.consts.ram.length,
                ))
                .into());
            }
            let length = {
                let mut orig = Overworld::new(self);
                orig.decompress(self, &*rom)?
            };
            let addr = rom.read_pointer(self.pointer)?;
            rom.free(addr, length as u16)?;
            let addr = rom.alloc(addr, map.data.len() as u16, Alloc::Best)?;
            rom.write_bytes(addr, &map.data)?;
            rom.write_pointer(self.pointer, addr)?;

            for (i, &offset) in map.palace_offset.iter().enumerate() {
                if offset != 0 {
                    rom.write_word(
                        self.palace.stone_table + i * 2,
                        self.consts.ram.address.offset() as u16 + offset,
                    )?;
                } else {
                    // TODO: what to do when its zero?
                }
            }

            for (&i, conn) in ov.connection.iter() {
                self.put_connector(conn, i, &mut *rom)?;
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("Overworld:{path:?}")).into()),
        }
    }

    fn get_connector(&self, index: u8, rom: &NesFile) -> Result<Connector> {
        let mut y = rom.read(self.connector + index + 0x00)?;
        let x = rom.read(self.connector + index + 0x3f)?;
        let z = rom.read(self.connector + index + 0x7e)?;
        let w = rom.read(self.connector + index + 0xbd)?;

        let second = (x & 0x40) != 0;
        let exit_2_lower = (x & 0x80) != 0;
        let x = x & 0x3f;

        let area = z & 0x3f;
        let screen = z >> 6;

        let dest_overworld = w & 0x03;
        let dest_world = (w & 0x1c) >> 2;
        let entry_right = (w & 0x20) != 0;
        let passthru = (w & 0x40) != 0;
        let fall = (w & 0x80) != 0;

        let palace = if let Some(p) = self.palace_code(index) {
            if p < self.palace.length {
                Some(PalaceDetail {
                    index: p,
                    chr_bank: rom.read(self.palace.chr_table + p)? * 2,
                    palette: rom.read(self.palace.palette_table + p)? / 16,
                })
            } else {
                None
            }
        } else {
            None
        };
        let hidden = if let Some(spot) = self.hidden_spot(rom, index)? {
            let is_hidden = y == 0;
            y = rom.read(spot.connector + 2)?;
            Some(is_hidden)
        } else {
            None
        };

        Ok(Connector {
            x,
            y: (y & 0x7f).wrapping_sub(self.consts.y_offset),
            area,
            screen,
            entry_right,

            dest_world,
            dest_overworld,
            external: (y & 0x80) != 0,
            second,
            exit_2_lower,
            passthru,
            fall,
            hidden,
            palace,
        })
    }

    fn put_connector(&self, conn: &Connector, index: u8, rom: &mut NesFile) -> Result<()> {
        let y = if conn.external { 0x80 } else { 0x00 } | conn.y.wrapping_add(self.consts.y_offset);
        let x = if conn.second { 0x40 } else { 0x00 }
            | if conn.exit_2_lower { 0x80 } else { 0x00 }
            | conn.x as u8;
        let z = (conn.screen << 6) as u8 | conn.area as u8;

        let w = if conn.entry_right { 0x20 } else { 0x00 }
            | if conn.passthru { 0x40 } else { 0x00 }
            | if conn.fall { 0x80 } else { 0x00 }
            | conn.dest_overworld as u8
            | (conn.dest_world << 2) as u8;

        rom.write(self.connector + index + 0x00, y)?;
        rom.write(self.connector + index + 0x3f, x)?;
        rom.write(self.connector + index + 0x7e, z)?;
        rom.write(self.connector + index + 0xbd, w)?;

        if let Some(palace) = &conn.palace {
            rom.write(self.palace.chr_table + palace.index, palace.chr_bank / 2)?;
            rom.write(
                self.palace.palette_table + palace.index,
                palace.palette * 16,
            )?;
        }
        if let Some(hidden) = conn.hidden {
            let spot = self.hidden_spot(rom, index)?.ok_or_else(|| {
                Error::Configuration(format!(
                    "Connector {index} is marked hidden, but no HiddenSpot configuration"
                ))
            })?;
            match spot.kind {
                HiddenKind::Palace => {
                    // Hidden palace Y coordinate includes the y_offset.
                    // Furthermore, the "call" spot is 2 tiles north of the target.
                    ensure!(
                        spot.ppu_macro.offset() != 0,
                        Error::Configuration(format!(
                            "No ppu_macro defined for hidden {:?} in overworld {}",
                            spot.kind, self.overworld
                        ))
                    );

                    rom.write(spot.connector + 2, y)?;
                    rom.write(spot.y, (y & 0x7f) - 2)?;
                    rom.write(spot.x, conn.x)?;
                    rom.write(spot.return_y, y & 0x7f)?;

                    // Compute the destination address in VRAM.
                    let xx = conn.x as u16;
                    let yy = conn.y as u16;
                    let ppu_addr =
                        0x2000 + 2 * (32 * (yy % 15) + (xx % 16)) + 0x800 * (yy % 30 / 15);

                    rom.write(spot.ppu_macro + 0, (ppu_addr >> 8) as u8)?;
                    rom.write(spot.ppu_macro + 1, ppu_addr as u8)?;
                    rom.write(spot.ppu_macro + 5, ((ppu_addr + 32) >> 8) as u8)?;
                    rom.write(spot.ppu_macro + 6, (ppu_addr + 32) as u8)?;
                    // Being lazy and not dealing with the color bits.
                    rom.write(spot.ppu_macro + 10, 0xff)?;
                    if hidden {
                        rom.write(self.connector + index, 0)?;
                    }
                }
                HiddenKind::Town => {
                    // Hidden Town Y coordinate does not include the overworld_y_offset.
                    // Furthermore, the x location seems to be 1 more than the actual
                    // coordinate.
                    ensure!(
                        spot.discriminator.offset() != 0,
                        Error::Configuration(format!(
                            "No discriminator defined for hidden {:?} in overworld {}",
                            spot.kind, self.overworld
                        ))
                    );
                    rom.write(spot.connector + 2, y)?;
                    rom.write(spot.y, conn.y)?;
                    rom.write(spot.x, conn.x + 1)?;
                    rom.write(spot.return_y, y & 0x7f)?;
                    rom.write(spot.discriminator, conn.x)?;
                    if hidden {
                        rom.write(self.connector + index, 0)?;
                    }
                }
                _ => {
                    bail!(Error::Configuration(format!(
                        "Don't know how to pack hidden {:?} in overworld {}",
                        spot.kind, self.overworld
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn palace_code(&self, connector: u8) -> Option<usize> {
        self.consts
            .palace_connectors
            .iter()
            .position(|&v| v == connector)
    }

    pub fn town_code(&self, connector: u8) -> Option<usize> {
        self.consts
            .town_connectors
            .iter()
            .position(|&v| v == connector)
            .map(|i| i / 2)
    }

    fn hidden_spot(&self, rom: &NesFile, connector: u8) -> Result<Option<&config::HiddenSpot>> {
        for spot in self.hidden.iter() {
            if rom.read(spot.connector)? == connector
                && rom.read(spot.overworld)? == self.overworld
                && self.subworld == 0
            {
                // For now, we assume a hidden spot cannot be on DM or MZ.
                return Ok(Some(spot));
            }
        }
        Ok(None)
    }
}

#[derive(Eq, PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub enum HiddenKind {
    #[default]
    Unknown,
    Town,
    Palace,
}

#[derive(Default)]
struct CompressedMap {
    data: Vec<u8>,
    palace_offset: [u16; 4],
    hidden_palace: Option<u8>,
    hidden_town: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(try_from = "JsonMap")]
#[serde(into = "JsonMap")]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<u8>>,
}

impl Overworld {
    pub fn new(cfg: &config::Overworld) -> Self {
        Overworld {
            map: Map {
                width: cfg.width,
                height: cfg.height,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn calculate_size(&self, cfg: &config::Overworld) -> Result<usize> {
        self.compress(cfg).map(|m| m.data.len())
    }

    pub fn connector_at(&self, x: usize, y: usize) -> Option<(u8, &Connector)> {
        for (&key, conn) in self.connection.iter() {
            if conn.x == x as u8 && conn.y == y as u8 {
                return Some((key, conn));
            }
        }
        None
    }

    fn decompress(&mut self, cfg: &config::Overworld, rom: &NesFile) -> Result<usize> {
        self.decompress_standard(cfg, rom)
    }

    fn compress(&self, cfg: &config::Overworld) -> Result<CompressedMap> {
        self.compress_standard(cfg)
    }

    fn decompress_standard(&mut self, cfg: &config::Overworld, rom: &NesFile) -> Result<usize> {
        self.map.data = vec![vec![0xf as u8; self.map.width]; self.map.height];
        let addr = rom.read_pointer(cfg.pointer)?;
        let mut y = 0;
        let mut index = 0;
        while y < self.map.height {
            let mut x = 0;
            while x < self.map.width {
                let val = rom.read(addr + index)?;
                let tile = val & 0x0F;
                let count = (val >> 4) as usize + 1;
                for _ in 0..count {
                    if x < self.map.width {
                        self.map.data[y][x] = tile;
                    }
                    x += 1;
                }
                index += 1;
            }
            y += 1;
        }
        Ok(index)
    }

    fn compress_standard(&self, cfg: &config::Overworld) -> Result<CompressedMap> {
        let mut map = CompressedMap::default();
        for (y, row) in self.map.data.iter().enumerate() {
            let mut x = 0;
            while x < self.map.width {
                let tile = row[x];
                let mut want_compress = if tile == 0x0E {
                    self.compress_boulder.unwrap_or(true)
                } else if tile == 0x0F {
                    self.compress_spider.unwrap_or(true)
                } else {
                    true
                };
                if let Some((index, conn)) = self.connector_at(x, y) {
                    if let Some(palace) = cfg.palace_code(index) {
                        want_compress = false;
                        map.palace_offset[palace] = map.data.len() as u16;
                        if let Some(true) = &conn.hidden {
                            map.hidden_palace = Some(tile);
                        }
                    } else {
                        if let Some(true) = &conn.hidden {
                            want_compress = false;
                            map.hidden_town = Some(tile);
                        }
                    }
                }
                let mut count = 0u8;
                while want_compress && count < 15 && x + 1 < self.map.width && tile == row[x + 1]
                // TODO: && !overworld.skip_compress(x + 1, y)
                {
                    x += 1;
                    count += 1;
                }
                map.data.push(tile | count << 4);
                x += 1;
            }
        }
        Ok(map)
    }
}

const JSON_TRANSFORM: [u8; 64] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<String>,
}

impl From<Map> for JsonMap {
    fn from(a: Map) -> Self {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = String::new();
            for &col in row.iter() {
                let col = (col as usize).min(JSON_TRANSFORM.len() - 1);
                s.push(JSON_TRANSFORM[col] as char);
            }
            map.push(s);
        }
        JsonMap {
            width: a.width,
            height: a.height,
            data: map,
        }
    }
}

impl TryFrom<JsonMap> for Map {
    type Error = Error;
    fn try_from(a: JsonMap) -> Result<Self, Self::Error> {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = Vec::new();
            for col in row.chars() {
                let v = JSON_TRANSFORM
                    .iter()
                    .position(|&x| col as u8 == x)
                    .ok_or_else(|| {
                        Error::Map(format!("Map transform could not convert '{col:?}'",))
                    })?;
                s.push(v as u8);
            }
            map.push(s);
        }
        Ok(Map {
            width: a.width,
            height: a.height,
            data: map,
            ..Default::default()
        })
    }
}
