use anyhow::{ensure, Result};
use indexmap::map::Entry;
use indexmap::{IndexMap, IndexSet};
use itertools::join;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, Alloc, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use crate::zelda2::encounters::Encounters;
use crate::zelda2::object::{BackgroundTiles, Object, RenderInfo, Renderer};
use crate::zelda2::project::Project;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize, Hash)]
pub struct MapCommand {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
    pub param: u8,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize, Hash)]
pub struct Map {
    pub objset: u8,
    pub width: u8,
    pub grass: bool,
    pub bushes: bool,
    pub ceiling: bool,
    pub floor: u8,
    pub tileset: u8,
    pub sprite_palette: u8,
    pub background_palette: u8,
    pub background_map: u8,
    pub cursor_moves_left: bool,
    pub data: Vec<MapCommand>,
}
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialog: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialog2: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyList {
    pub data: Vec<Vec<Enemy>>,
    #[serde(skip)]
    pub ram_address: Address,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub area: u8,
    pub screen: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_target_back: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sideview {
    pub map: Map,
    pub enemy: EnemyList,
    pub connection: Vec<Connection>,
    pub availability: Vec<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub door: Vec<Connection>,
}

#[typetag::serde]
impl GameData for Sideview {
    fn name(&self) -> String {
        "Sideview".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::sideview::SideviewEditor::new(self, name)
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum AreaKind {
    #[default]
    Overworld,
    Town,
    Palace,
    GreatPalace,
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SideviewAreas {
        pub name: String,
        pub area_kind: AreaKind,
        pub world: u8,
        pub overworld: u8,
        pub subworld: u8,
        pub chr: Address,
        pub length: usize,
        pub address: Address,
        pub availability: Address,
        pub enemylist: Address,
        pub connections: Address,
        pub max_connectable_index: usize,
        pub doors: Address,
        pub door_objects: IndexSet<u8>,
        pub max_door_index: usize,
        pub metatile: String,
        pub palette: String,
        pub render_info: String,
        pub is_background_layer: bool,
        pub background: Option<String>,
        pub encounters: Option<String>,
        pub enemy_group: Option<String>,
        pub text_table: Option<String>,
        pub area_names: IndexMap<u8, String>,
        // Area indices that we should intentionally memory-leak.
        // We don't track duplicates between categories (e.g. "background" and "0").
        // In the vanilla game, background[2] is a duplicate of all town wizard basements,
        // so to prevent a double-free, we just leak background[2] so alloc/free works properly
        // when we get to the wizard basements.
        pub leak: IndexSet<usize>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SideviewGroup {
        pub group: IndexMap<String, SideviewAreas>,
        pub enemy_ram_offset: usize,
        pub enemy_rom_offset: Address,
        pub enemy_length: usize,
    }
}

impl config::SideviewAreas {
    pub fn unpack(
        &self,
        group: &config::SideviewGroup,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("SideviewAreas::unpack {path}");
        let rom = rrom.borrow();
        for index in 0..self.length {
            let enc = self.is_encounter(index as u8, edits)?;
            let sv = match Sideview::from_rom(&*rom, group, self, index, enc) {
                Ok(sv) => sv,
                Err(e) => {
                    log::warn!("Reading {path}/{index} from ROM: {e}");
                    Sideview::default()
                }
            };
            let empty_enemies = sv.enemy.is_empty();
            let mut edit = Edit::new(sv.into());

            // check for aliases
            let addr = rom.read_pointer(self.address + index * 2)?;
            edit.meta
                .extra
                .insert("original-address".into(), format!("{addr:x?}"));
            let mut alias = Vec::new();
            for j in 0..self.length {
                if j != index {
                    let a = rom.read_pointer(self.address + j * 2)?;
                    if a == addr {
                        alias.push(j);
                    }
                }
            }
            if !alias.is_empty() {
                edit.meta.extra.insert("alias".into(), join(&alias, ","));
            }

            // check for enemy list aliases
            if !empty_enemies {
                let addr = rom.read_pointer(self.enemylist + index * 2)?;
                let mut alias = Vec::new();
                for j in 0..self.length {
                    if j != index {
                        let a = rom.read_pointer(self.enemylist + j * 2)?;
                        if a == addr {
                            alias.push(j);
                        }
                    }
                }
                if !alias.is_empty() {
                    edit.meta
                        .extra
                        .insert("enemy-alias".into(), join(&alias, ","));
                }
            }

            if self.leak.contains(&index) {
                edit.meta.extra.insert("leak".into(), "true".into());
            }

            let i = if self.is_background_layer {
                index + 1
            } else {
                index
            };
            edits.insert(format!("{path}/{i}"), edit);
        }
        Ok(())
    }

    pub fn pack(
        &self,
        group: &config::SideviewGroup,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &EditList,
        packed_enemies: &mut PackedEnemies,
        free: &mut IndexSet<Address>,
        done: &mut IndexMap<Map, Address>,
    ) -> Result<()> {
        log::debug!("SideviewAreas::pack {path}");
        for index in 0..self.length {
            let i = if self.is_background_layer {
                index + 1
            } else {
                index
            };
            let mut rom = rrom.borrow_mut();
            let path = format!("{path}/{i}");
            log::debug!("SideviewAreas::pack {path}");

            if let Some(edit) = edits.get(&path) {
                // Are we supposed to leak this memory block?
                let leak = edit
                    .meta
                    .extra
                    .get("leak")
                    .map(|v| v == "true")
                    .unwrap_or(false);

                let addr = rom.read_pointer(self.address + index * 2)?;
                if !free.contains(&addr) && !leak {
                    // Free memory if not already freed.
                    if addr.is_valid() {
                        let len = rom.read(addr)?;
                        log::debug!("Freeing {len} bytes at {addr:x?} for index {index}");
                        rom.free(addr, len as u16)?;
                        free.insert(addr);
                    } else {
                        log::debug!("Skipping invalid address {addr:x?} for index {index}");
                    }
                }

                let enemy_alias = edit
                    .meta
                    .extra
                    .get("enemy-alias")
                    .map(|a| {
                        a.split(',')
                            .map(|a| a.parse::<usize>())
                            .collect::<Result<Vec<_>, std::num::ParseIntError>>()
                    })
                    .unwrap_or_else(|| Ok(Vec::new()))?;
                packed_enemies.set_duplicates(index, &enemy_alias);

                let sv = edit.data_ref::<Sideview>()?;
                sv.to_rom(&mut rom, group, self, index, packed_enemies, done)?;
            }
        }
        Ok(())
    }

    pub fn is_encounter(&self, area: u8, edits: &EditList) -> Result<bool> {
        Ok(self
            .encounters
            .as_ref()
            .map(|path| edits.get(path))
            .flatten()
            .map(|edit| edit.data_ref::<Encounters>())
            .transpose()?
            .map(|enc| enc.is_encounter(area))
            .unwrap_or(false))
    }
}

impl config::SideviewGroup {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("SideviewGroup::unpack {path}");
        for (k, cfg) in self.group.iter() {
            cfg.unpack(self, rrom, &format!("{path}/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("SideviewGroup::pack {path}");
        let mut free = IndexSet::new();
        let mut done = IndexMap::new();
        // The enemy list is per-bank, even when there are two sideview groups in a bank.
        // We have to track duplicates because the area of the ROM holding the enemy list
        // is often sized exactly to what the bank needs.
        //
        // We reset duplicate tracking between the per-bank sideview groups.
        let mut enemylist = PackedEnemies::new(self.group[0].area_kind);
        for (k, cfg) in self.group.iter() {
            enemylist.reset_duplicates();
            cfg.pack(
                self,
                rrom,
                &format!("{path}/{k}"),
                edits,
                &mut enemylist,
                &mut free,
                &mut done,
            )?;
        }
        // Once the entire enemylist has been packed, we can write the whole thing to ROM.
        if enemylist.data.len() <= self.enemy_length {
            let mut rom = rrom.borrow_mut();
            enemylist.data.resize(self.enemy_length, 0xff);
            rom.write_bytes(self.enemy_rom_offset, &enemylist.data)?;
        } else {
            return Err(Error::Length(format!(
                "Enemy list too big for {path}.  {} > {}",
                enemylist.data.len(),
                self.enemy_length
            ))
            .into());
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            [ref n, ..] => {
                let areas = self
                    .group
                    .get(*n)
                    .ok_or(Error::NotFound(format!("SideviewGroup/{n}")))?;
                get_config::<T>(areas)
            }
        }
    }
}

// FIXME: Temporary until packing sideview areas is ready.
#[allow(dead_code)]
impl Map {
    fn from_bytes(data: &[u8]) -> Self {
        let len = data[0] as usize;
        let mut commands = Vec::new();
        let mut i = 4;
        let mut xpos = 0;
        let mut cursor_moves_left = false;
        while i < len {
            let xy = data[i + 0];
            let kp = data[i + 1];

            // While reading in the map, we convert from relative-X to absolute-X cordinates.
            let x = xy & 0xF;
            let y = xy >> 4;
            if y == 14 {
                // Y-position 14 is really the X-Skip command.
                if x * 16 < xpos {
                    cursor_moves_left = true;
                }
                xpos = x * 16;
            } else {
                xpos += x;
            }
            let (kind, param) = if y == 13 || y == 14 {
                // Y-position 13 means new floor height.
                // Y-position 14 means X-Skip.
                (0, kp)
            } else if kp < 0x0f {
                // Small object has a kind only.
                (kp, 0)
            } else if kp == 0x0f {
                // Extra object has the param in the next byte.
                i += 1;
                (kp, data[i + 1])
            } else {
                // Larg object has the kind in the high nybble, param in the low nybble.
                (kp & 0xF0, kp & 0x0F)
            };
            commands.push(MapCommand {
                x: xpos,
                y,
                kind,
                param,
            });
            i += 2;
        }
        Map {
            objset: if data[1] & 0x80 != 0 { 1 } else { 0 },
            width: ((data[1] >> 5) & 3),
            grass: data[1] & 8 == 8,
            bushes: data[1] & 4 == 4,
            ceiling: data[2] & 0x80 == 0,
            floor: data[2] & 0x0f,
            tileset: (data[2] >> 4) & 7,
            sprite_palette: (data[3] >> 6) & 3,
            background_palette: (data[3] >> 3) & 7,
            background_map: data[3] & 7,
            cursor_moves_left,
            data: commands,
        }
    }

    pub fn elevator(&self) -> Option<u8> {
        for item in self.data.iter() {
            // An "extra" object of kind 0x50 is an elevator.
            if item.y == 15 && item.kind == 0x50 {
                return Some(item.x);
            }
        }
        None
    }

    pub fn doors(&self, config: &config::SideviewAreas) -> Vec<Option<u8>> {
        let mut result = vec![None; 4];
        for item in self.data.iter() {
            if config.door_objects.contains(&item.kind) {
                result[(item.x / 16) as usize] = Some(item.x);
            }
        }
        result
    }

    fn sort_data(data: &mut Vec<MapCommand>) {
        data.sort_by(|a, b| {
            if a.x == b.x {
                // We wrap the y coordinate around so that meta-ops will
                // sort before regular ops (new floor, extra items)
                let ya = (a.y + 3) % 16;
                let yb = (b.y + 3) % 16;
                ya.cmp(&yb)
            } else {
                a.x.cmp(&b.x)
            }
        });
    }

    pub fn sort(&mut self) {
        Map::sort_data(&mut self.data);
    }

    fn optimize(&self) -> Vec<MapCommand> {
        let mut data = self.data.clone();
        if !self.cursor_moves_left {
            Map::sort_data(&mut data);
        }
        // Strip out all "skip to screen" commands, as we'll
        // recreate them exactly where they're needed.
        data.retain(|elem| elem.y != 14);

        let mut x = 0i8;
        let mut i = 0;
        while i < data.len() {
            let delta = data[i].x as i8 - x;
            if delta < 0 || delta > 15 {
                // If we need to move too far, insert a "skip to screen" command.
                let newx = data[i].x & !15;
                data.insert(
                    i,
                    MapCommand {
                        y: 14,
                        x: newx / 16,
                        ..Default::default()
                    },
                );
                x = newx as i8;
                i += 1;
            } else {
                // Otherwise, just remember the last x coordinate.
                x = data[i].x as i8;
            }
            i += 1;
        }
        data
    }

    fn to_relative(&self) -> Vec<MapCommand> {
        let mut result = Vec::new();
        let mut x = 0;
        for item in self.optimize().iter() {
            let mut command = item.clone();
            if item.y != 14 {
                // Not an X-skip; compute distance relative to prior command.
                command.x -= x;
                x = item.x;
            } else {
                // Is an X-skip, set new last position.
                x = item.x * 16;
            }
            result.push(command);
        }
        result
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(0); // length (filled in later).
        result.push(
            // flags
            ((self.objset as u8) << 7)
                | (self.width & 3) << 5
                | if self.grass { 8 } else { 0 }
                | if self.bushes { 4 } else { 0 },
        );
        result.push(
            // floor & tileset.
            if self.ceiling { 0x00 } else { 0x80 }
                | self.floor as u8 & 0x0F
                | (self.tileset as u8 & 7) << 4,
        );
        result.push(
            // palettes.
            ((self.sprite_palette as u8 & 3) << 6)
                | ((self.background_palette as u8 & 7) << 3)
                | (self.background_map as u8 & 7),
        );

        let commands = self.to_relative();
        for c in commands.iter() {
            let xy = (c.x as u8 & 0xF) | (c.y as u8 & 0xF) << 4;
            let kind = match c.y {
                13 => c.param as u8,
                14 => 0,
                _ => {
                    if c.kind < 0x10 {
                        c.kind as u8
                    } else {
                        c.kind as u8 | c.param as u8 & 0x0F
                    }
                }
            };
            result.push(xy);
            result.push(kind);
            if c.kind == 0x0F {
                // Collectables are 3 bytes.
                result.push(c.param as u8);
            }
        }
        result[0] = result.len() as u8;
        result
    }
}

impl From<u8> for Connection {
    fn from(val: u8) -> Self {
        Connection {
            area: val >> 2,
            screen: val & 3,
            ..Default::default()
        }
    }
}

impl From<&Connection> for u8 {
    fn from(val: &Connection) -> Self {
        (val.screen & 3) | (val.area << 2)
    }
}

impl Connection {
    pub fn outside() -> Self {
        Connection {
            area: 63,
            ..Default::default()
        }
    }
}

impl EnemyList {
    fn from_rom(
        rom: &NesFile,
        group: &config::SideviewGroup,
        cfg: &config::SideviewAreas,
        index: usize,
        is_encounter: bool,
    ) -> Result<Self> {
        let delta = group.enemy_rom_offset.offset() - group.enemy_ram_offset;
        let ram_addr = rom.read_pointer(cfg.enemylist + index * 2)?;
        let addr = ram_addr + delta;
        let mut list = Self::default();
        let mut total = 0;

        if ram_addr.offset() >= group.enemy_ram_offset
            && ram_addr.offset() < group.enemy_ram_offset + 0x400
        {
            let len = rom.read(addr)? as usize;
            total += len;
            list.data
                .push(Self::list_from_bytes(rom.read_bytes(addr, len)?));
            if is_encounter {
                let addr = addr + len;
                let len = rom.read(addr)? as usize;
                total += len;
                list.data
                    .push(Self::list_from_bytes(rom.read_bytes(addr, len)?));
            }
        }
        log::debug!("EnemyList: index {index} read from {addr:x?} ({total} bytes)");
        Ok(list)
    }

    fn list_from_bytes(data: &[u8]) -> Vec<Enemy> {
        let length = data[0] as usize;
        let length = if length % 2 == 0 { length - 1 } else { length };
        let mut i = 1;
        let mut list = Vec::new();
        while i < length {
            let xy = data[i + 0];
            let y = (xy) >> 4;
            let y = if y == 0 { y + 1 } else { y + 2 };
            let kind = data[i + 1];
            list.push(Enemy {
                x: (xy & 0x0F) | (kind >> 2) & 0x30,
                y,
                kind: kind & 0x3f,
                ..Default::default()
            });
            i += 2;
        }
        list
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for list in self.data.iter() {
            result.push(list.len() as u8 * 2 + 1);
            for enemy in list.iter() {
                let y = if enemy.y <= 1 { 0 } else { enemy.y - 2 };
                let xy = (y << 4) | (enemy.x & 0x0F);
                let kind = (enemy.x & 0x30) << 2 | (enemy.kind & 0x3f);
                result.push(xy);
                result.push(kind);
            }
        }
        result
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty() || self.data.iter().all(|x| x.is_empty())
    }
}

impl Sideview {
    fn from_rom(
        rom: &NesFile,
        group: &config::SideviewGroup,
        cfg: &config::SideviewAreas,
        index: usize,
        is_encounter: bool,
    ) -> Result<Self> {
        let addr = rom.read_pointer(cfg.address + index * 2)?;
        ensure!(
            addr.is_valid(),
            Error::NotFound(format!(
                "Sideview::from_rom: index {index} map pointer is invalid ({addr:x?})"
            ))
        );
        let length = rom.read(addr)? as usize;
        log::debug!(
            "Sideview::from_rom reading index {index} from ROM @ {addr:x?} for {length} bytes"
        );
        let length = length.max(4);
        let map = Map::from_bytes(rom.read_bytes(addr, length)?);

        let availability = if cfg.availability.is_valid() {
            let a = rom.read(cfg.availability + index / 2)?;
            let a = if index & 1 == 0 { a >> 4 } else { a & 0x0F };
            vec![a & 8 != 0, a & 4 != 0, a & 2 != 0, a & 1 != 0]
        } else {
            Vec::new()
        };

        let enemy = if cfg.enemylist.is_valid() {
            EnemyList::from_rom(rom, group, cfg, index, is_encounter)?
        } else {
            EnemyList::default()
        };
        let connection = if cfg.connections.is_valid() && index <= cfg.max_connectable_index {
            let table = rom.read_bytes(cfg.connections + index * 4, 4)?;
            table
                .iter()
                .map(|&v| Connection::from(v))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let door = if cfg.doors.is_valid() && index <= cfg.max_door_index {
            let table = rom.read_bytes(cfg.doors + index * 4, 4)?;
            table
                .iter()
                .map(|&v| Connection::from(v))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        Ok(Sideview {
            map,
            enemy,
            connection,
            availability,
            door,
        })
    }

    fn to_rom(
        &self,
        rom: &mut NesFile,
        group: &config::SideviewGroup,
        cfg: &config::SideviewAreas,
        index: usize,
        packed_enemies: &mut PackedEnemies,
        done: &mut IndexMap<Map, Address>,
    ) -> Result<()> {
        let addr = match done.entry(self.map.clone()) {
            Entry::Occupied(o) => {
                let addr = *o.get();
                log::debug!("Sideview::to_rom index {index} is identical to map at {addr:x?}");
                addr
            }
            Entry::Vacant(v) => {
                let data = v.key().to_bytes();
                let addr = rom.alloc(cfg.address, data.len() as u16, Alloc::Best)?;
                log::debug!(
                    "Sideview::to_rom allocated {} bytes at {addr:x?}",
                    data.len()
                );
                rom.write_bytes(addr, &data)?;
                *v.insert(addr)
            }
        };
        rom.write_pointer(cfg.address + index * 2, addr)?;

        if cfg.enemylist.is_valid() {
            let offset = packed_enemies.add(index, &self.enemy) + (group.enemy_ram_offset as u16);
            rom.write_word(cfg.enemylist + index * 2, offset)?;
        }

        if cfg.availability.is_valid() {
            let avail = if self.availability.get(0).copied().unwrap_or(false) {
                8
            } else {
                0
            } | if self.availability.get(1).copied().unwrap_or(false) {
                4
            } else {
                0
            } | if self.availability.get(2).copied().unwrap_or(false) {
                2
            } else {
                0
            } | if self.availability.get(3).copied().unwrap_or(false) {
                1
            } else {
                0
            };
            let a = rom.read(cfg.availability + index / 2)?;
            let a = if index & 1 == 0 {
                (a & 0x0F) | (avail << 4)
            } else {
                (a & 0xF0) | (avail << 0)
            };
            rom.write(cfg.availability + index / 2, a)?;
        }

        if cfg.connections.is_valid() && index <= cfg.max_connectable_index {
            for (i, c) in self.connection.iter().enumerate() {
                rom.write(cfg.connections + index * 4 + i, u8::from(c))?;
            }
        }

        if cfg.doors.is_valid() && index <= cfg.max_door_index {
            for (i, c) in self.door.iter().enumerate() {
                rom.write(cfg.doors + index * 4 + i, u8::from(c))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Decompressor {
    pub layers: [[[u8; 64]; 13]; 3],
    pub data: [[u8; 64]; 13],
    pub item: [[u8; 64]; 13],
    pub bgtile: u8,
    pub layer: usize,
}

impl Decompressor {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 13;

    pub fn new() -> Self {
        Decompressor {
            layers: [[[0; 64]; 13]; 3],
            data: [[0; 64]; 13],
            item: [[0xFF; 64]; 13],
            bgtile: 0,
            layer: 0,
        }
    }

    pub fn decompress(&mut self, path: &str, sideview: &Sideview, project: &Project) -> Result<()> {
        let map = &sideview.map;
        let mut xcursor = 0;
        let mut floor = map.floor as usize;
        let mut ceiling = map.ceiling;
        let cfg = project.config.get::<config::SideviewAreas>(path)?;
        let render = project.config.get::<RenderInfo>(&cfg.render_info)?;
        let (_, background) = &render
            .background
            .get_index(map.tileset as usize)
            .ok_or_else(|| {
                Error::NotFound(format!("Background info for tileset {}", map.tileset))
            })?;
        let width = Decompressor::WIDTH as u8;

        log::debug!("Render {}:", path);
        log::debug!(
            "ObjSet={} Width={} Grass={} Bushes={}",
            map.objset,
            map.width,
            map.grass,
            map.bushes
        );
        log::debug!(
            "Ceiling={} Floor={} Tileset={}",
            map.ceiling,
            map.floor,
            map.tileset
        );
        log::debug!(
            "SprPal={} BgPal={} BgMap={}",
            map.sprite_palette,
            map.background_palette,
            map.background_map
        );

        self.bgtile = background.background;
        self.clear();

        if map.background_map != 0 {
            let background = cfg.background.as_ref().ok_or_else(|| {
                Error::Configuration("No background path specified in config".into())
            })?;
            let bgpath = format!("{background}/{}", map.background_map);
            if let Some(edit) = project.edits.get(&bgpath) {
                let bgmap = edit.data_ref::<Sideview>()?;
                self.decompress(&bgpath, bgmap, project)?;
                self.layer += 1;
            } else {
                log::error!(
                    "Map {} has background map, but no background map found.",
                    path
                );
            }
        }

        let mut data = map.data.clone();
        if !sideview.map.cursor_moves_left {
            Map::sort_data(&mut data);
        }
        for command in data.iter() {
            let mut extra = false;
            if command.y == 13 {
                while xcursor < command.x && xcursor < width {
                    self.draw_floor(xcursor, floor, ceiling, map, background);
                    xcursor += 1;
                }
                floor = command.param as usize & 0x0F;
                ceiling = command.param & 0x80 == 0;
                log::debug!(
                    "Render NewFloor @ y={:02} x={:02}: floor={} ceiling={}",
                    command.y,
                    command.x,
                    floor,
                    ceiling
                );
                continue;
            } else if command.y == 14 {
                // Y Position 14 means "X-Skip".  The Map reader will already
                // have adjusted the X coordinate from screen number to X position.
                while xcursor < command.x && xcursor < width {
                    self.draw_floor(xcursor, floor, ceiling, map, background);
                    xcursor += 1;
                }
                log::debug!(
                    "Render X-Skip   @ y={:02} x={:02}: to_screen={}",
                    command.y,
                    command.x,
                    command.x / 16
                );
                continue;
            } else if command.y == 15 {
                extra = true;
            }

            while xcursor < command.x && xcursor < width {
                self.draw_floor(xcursor, floor, ceiling, map, background);
                xcursor += 1;
            }

            // Cursor might move left.
            xcursor = command.x;
            self.draw_floor(xcursor, floor, ceiling, map, background);

            let (kind, object) = if extra {
                ("extra", &render.extra)
            } else {
                if command.kind < 0x10 {
                    ("small", &render.small)
                } else if map.objset == 0 {
                    ("objset0", &render.objset0)
                } else {
                    ("objset1", &render.objset1)
                }
            };

            log::debug!(
                "Render Object   @ y={:02} x={:02}: {}/{:02x}",
                command.y,
                command.x,
                kind,
                command.kind
            );

            if let Some(obj) = object.get(&command.kind) {
                match &obj.render {
                    Renderer::Grid => self.draw_grid(xcursor, command.y, command.param, obj),
                    Renderer::Horizontal => {
                        self.draw_horizontal(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Vertical => {
                        self.draw_vertical(xcursor, command.y, command.param, obj)
                    }
                    Renderer::TopUnique => {
                        self.draw_top_unique(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Building => {
                        self.draw_building(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Window => self.draw_window(xcursor, command.y, command.param, obj),
                    Renderer::Item => self.draw_item(xcursor, command.y, command.param, obj),
                }
            } else {
                log::error!("Cannot render {:?}: {}/{:02x}", path, kind, command.kind);
            }
        }
        // Finish rendering to end of room.
        while xcursor < width {
            self.draw_floor(xcursor, floor, ceiling, map, background);
            xcursor += 1;
        }
        self.collapse_layers();
        Ok(())
    }

    fn clear(&mut self) {
        for z in 0..3 {
            for y in 0..Decompressor::HEIGHT {
                for x in 0..Decompressor::WIDTH {
                    self.layers[z][y][x] = if z == 0 { self.bgtile } else { 0 };
                    self.item[y][x] = 0xFF;
                }
            }
        }
        // Layer 0 is the background tile.
        // Layers 1 & 2 are map data.
        self.layer = 1;
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        if x < Decompressor::WIDTH && y < Decompressor::HEIGHT {
            self.layers[self.layer][y][x] = val;
        }
    }

    fn set_if_bg(&mut self, x: usize, y: usize, val: u8) {
        if x < Decompressor::WIDTH && y < Decompressor::HEIGHT {
            if self.layers[self.layer][y][x] == 0 {
                self.layers[self.layer][y][x] = val;
            }
        }
    }

    fn collapse_layers(&mut self) {
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                for z in 0..3 {
                    if self.layers[z][y][x] != 0 {
                        self.data[y][x] = self.layers[z][y][x];
                    }
                }
            }
        }
    }

    fn draw_floor(
        &mut self,
        x: u8,
        floor: usize,
        ceiling: bool,
        map: &Map,
        background: &BackgroundTiles,
    ) {
        let x = x as usize;
        if map.grass {
            self.set_if_bg(x, 10, background.alternate);
        }
        if map.bushes {
            self.set_if_bg(x, 9, background.alternate);
        }

        if floor < 8 {
            let fy = Decompressor::HEIGHT - floor - 2;
            self.set_if_bg(x, fy, background.floor[0]);
            for y in (fy + 1)..Decompressor::HEIGHT {
                self.set_if_bg(x, y, background.floor[1]);
            }
            if ceiling {
                self.set_if_bg(x, 0, background.ceiling[1]);
            }
        } else if floor < 15 {
            if ceiling {
                // Ceiling height is "floor - 6".  Minus 1 more for inclusive range.
                let cy = (floor - 6) - 1;
                for y in 0..cy {
                    self.set_if_bg(x, y, background.ceiling[0]);
                }
                self.set_if_bg(x, cy, background.ceiling[1]);
            }
            self.set_if_bg(x, 11, background.floor[0]);
            self.set_if_bg(x, 12, background.floor[1]);
        } else {
            for y in 0..Decompressor::HEIGHT {
                self.set_if_bg(x, y, background.floor[1]);
            }
        }
    }

    fn calculate_y(y0: u8, param: u8, object: &Object) -> usize {
        if let Some(top) = object.fixed_y_minus_param {
            top - (param as usize + 1)
        } else {
            object.fixed_y.unwrap_or(y0 as usize)
        }
    }

    fn draw_grid(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        for y in 0..object.height {
            for x in 0..object.width {
                let metatile = object.metatile[y * object.width + x];
                if metatile != 0 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    fn draw_horizontal(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..object.height {
            for x in 0..(object.width * param) {
                self.set(
                    x0 + x,
                    y0 + y,
                    object.metatile[y * object.width + (x % object.width)],
                );
            }
        }
    }

    fn draw_vertical(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..(object.height * param) {
            for x in 0..object.width {
                self.set(
                    x0 + x,
                    y0 + y,
                    object.metatile[(y % object.height) * object.width + x],
                );
            }
        }
    }

    fn draw_top_unique(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let height = param as usize + 1;
        for y in 0..height {
            for x in 0..object.width {
                if y == 0 {
                    self.set(x0 + x, y0 + y, object.metatile[0 * object.width + x]);
                } else {
                    self.set(x0 + x, y0 + y, object.metatile[1 * object.width + x]);
                }
            }
        }
    }

    fn draw_item(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        if x0 < Decompressor::WIDTH && y0 < Decompressor::HEIGHT {
            self.item[y0][x0] = if object.name == "Collectable" {
                // ID 15 is "collectable object"
                param as u8
            } else {
                object.metatile[0]
            };
        }
    }

    fn draw_building(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let width = param as usize + 1;
        for y in 0..Decompressor::HEIGHT {
            for x in 0..width {
                let metatile = if x < width - 1 {
                    object.metatile[0]
                } else {
                    object.metatile[1]
                };
                if metatile != 0 && y0 + y < 11 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    fn draw_window(&mut self, x0: u8, y0: u8, param: u8, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..(object.height * param) {
            for x in 0..object.width {
                // Windows stop rendering at tile y-coordinate 10.
                let metatile = object.metatile[(y % object.height) * object.width + x];
                if metatile != 0 && y0 + y < 10 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for y in 0..Decompressor::HEIGHT {
            let mut row = String::new();
            for x in 0..Decompressor::WIDTH {
                let mut ch = self.data[y][x];
                if ch == self.bgtile {
                    ch = 0x20;
                } else {
                    ch &= 0x7F;
                    if ch < 0x20 {
                        ch |= 0x20;
                    }
                }
                row.push(ch as char);
            }
            result.push(row);
        }
        result
    }
}

#[derive(Debug)]
pub struct PackedEnemies {
    data: Vec<u8>,
    offsets: Vec<u16>,
    duplicates: IndexMap<usize, usize>,
}

impl PackedEnemies {
    pub fn new(area_kind: AreaKind) -> Self {
        let data = if area_kind == AreaKind::Overworld {
            vec![1, 1]
        } else {
            vec![1]
        };
        Self {
            data,
            offsets: Vec::new(),
            duplicates: IndexMap::new(),
        }
    }

    pub fn reset_duplicates(&mut self) {
        self.duplicates.clear();
    }

    pub fn set_duplicates(&mut self, index: usize, duplicates: &[usize]) {
        for d in duplicates {
            self.duplicates.insert(*d, index);
        }
    }

    pub fn add(&mut self, index: usize, enemies: &EnemyList) -> u16 {
        let bank = self.offsets.len() / 63;
        let area = self.offsets.len() % 63;
        let mut position = self.data.len() as u16;
        let enemies = enemies.to_bytes();
        if enemies.len() > 2 {
            // An enemy list must be at least 3 bytes long.
            if let Some(dup) = self.duplicates.get(&index).copied() {
                position = self.offsets[dup + bank * 63];
            } else {
                self.data.extend(&enemies);
            }
        } else {
            // An empty enemy list will be 1 byte long; a pair of empty lists (for an encounter
            // area) will be 2 bytes long.  Re-use the fixed empty-pair at the beginning of
            // the list.
            position = 0;
        }
        self.offsets.push(position);
        log::info!(
            "Packing enemylist {bank}:{area} at offset {position}; len={}",
            self.data.len()
        );
        position
    }
}
