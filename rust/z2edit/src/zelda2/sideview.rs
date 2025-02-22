use anyhow::{anyhow, ensure, Result};
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, AddressRange, Alloc, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use crate::zelda2::encounters::Encounters;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapCommand {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
    pub param: u8,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dialog: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyList {
    pub data: Vec<Enemy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secondary: Vec<Enemy>,
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

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SideviewAreas {
        pub name: String,
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
        pub max_door_index: usize,
        pub metatile: String,
        pub palette: String,
        pub background: Option<String>,
        pub encounters: Option<String>,
        pub enemy_group: Option<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SideviewGroup {
        #[serde(flatten)]
        pub group: IndexMap<String, SideviewAreas>,
        pub enemy_ram_offset: usize,
        pub enemy_rom_offset: usize,
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
            let is_encounter = self
                .encounters
                .as_ref()
                .map(|v| edits.get(v))
                .flatten()
                .map(|edit| edit.data_ref::<Encounters>())
                .transpose()?
                .map(|e| e.is_encounter(index as u8))
                .unwrap_or(false);
            let sv = match Sideview::from_rom(&*rom, group, self, index, is_encounter) {
                Ok(sv) => sv,
                Err(e) => {
                    log::error!("Error reading {path}/{index} from ROM: {e}");
                    Sideview::default()
                }
            };
            edits.insert(format!("{path}/{index}"), Edit::new(sv.into()));
        }
        Ok(())
    }
    pub fn pack(
        &self,
        _group: &config::SideviewGroup,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &EditList,
    ) -> Result<()> {
        log::debug!("SideviewAreas::pack {path}");
        if let Some(edit) = edits.get(path) {
            let mut _rom = rrom.borrow_mut();
            let _sv = edit.data_ref::<Sideview>()?;
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
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
        let Some((p1, _)) = path.rsplit_once('/') else {
            return Err(anyhow!("SidviewGroup::pack: bad path {path:?}"));
        };
        let Some((_, group)) = p1.rsplit_once('/') else {
            return Err(anyhow!("SidviewGroup::pack: bad path {path:?}"));
        };
        let Some(cfg) = self.group.get(group) else {
            return Err(anyhow!("SidviewGroup::pack: no group for {group:?}"));
        };
        cfg.pack(self, rrom, path, edits)?;
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
            width: ((data[1] >> 5) & 3) + 1,
            grass: data[1] & 8 == 8,
            bushes: data[1] & 4 == 4,
            ceiling: data[2] & 0x80 == 0,
            floor: data[2] & 0x0f,
            tileset: (data[2] >> 4) & 7,
            sprite_palette: (data[3] >> 6) & 3,
            background_palette: (data[3] >> 3) & 3,
            background_map: data[3] & 7,
            cursor_moves_left,
            data: commands,
        }
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

impl EnemyList {
    fn from_rom(
        rom: &NesFile,
        group: &config::SideviewGroup,
        cfg: &config::SideviewAreas,
        index: usize,
        is_encounter: bool,
    ) -> Result<Self> {
        let delta = group.enemy_rom_offset - group.enemy_ram_offset;
        let ram_addr = rom.read_pointer(cfg.enemylist + index * 2)?;
        let addr = ram_addr + delta;
        let mut list = Self::default();
        let mut total = 0;

        if ram_addr.offset() >= group.enemy_ram_offset
            && ram_addr.offset() < group.enemy_ram_offset + 0x400
        {
            let len = rom.read(addr)? as usize;
            total += len;
            list.data = Self::list_from_bytes(rom.read_bytes(addr, len)?);
            if is_encounter {
                let addr = addr + len;
                let len = rom.read(addr)? as usize;
                total += len;
                list.secondary = Self::list_from_bytes(rom.read_bytes(addr, len)?);
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
            let table = rom.read_bytes(cfg.connections + index * 4, 4)?;
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
}
