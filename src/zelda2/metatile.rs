use std::any::Any;
use std::collections::HashSet;
use std::convert::TryInto;
use std::rc::Rc;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::zelda2::metatile::MetatileGroupGui;
use crate::gui::zelda2::Gui;
use crate::nes::{IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metatile {
    pub id: IdPath,
    pub tile: IndexMap<usize, [u8; 4]>,
    pub palette: IndexMap<usize, i32>,
}

impl Metatile {
    pub fn new(id: IdPath) -> Self {
        Self {
            id: id,
            ..Default::default()
        }
    }
    pub fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut ret = Self::new(id);
        ret.unpack(edit)?;
        Ok(ret)
    }

    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if let Some(id) = id {
            Ok(Box::new(Self::new(IdPath::from(id))))
        } else {
            Err(ErrorKind::IdPathError("id required".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for Metatile {
    fn name(&self) -> String {
        "MetatileGroup".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let rom = edit.rom.borrow();
        let kind = self.id.last();
        self.tile.clear();
        match kind {
            "overworldtile" => {
                let ocfg = config.overworld.find(&self.id)?;
                for i in 0..ocfg.objtable_len {
                    self.tile.insert(
                        i,
                        rom.read_bytes(ocfg.objtable + i * 4, 4)?
                            .try_into()
                            .expect("Metatile::overworldtile"),
                    );
                    self.palette
                        .insert(i, rom.read(ocfg.tile_palette + i)? as i32);
                }
            }
            "metatile" => {
                let scfg = config.sideview.find(&self.id)?;
                for (t, &len) in scfg.metatile_lengths.iter().enumerate() {
                    let base = rom.read_pointer(scfg.metatile_table + t * 2)?;
                    for i in 0..len {
                        self.tile.insert(
                            t * 64 + i,
                            rom.read_bytes(base + i * 4, 4)?
                                .try_into()
                                .expect("Metatile::metatile"),
                        );
                    }
                }
            }
            _ => {
                return Err(
                    ErrorKind::IdPathError(format!("Unknown Metatile type {}", kind)).into(),
                );
            }
        }
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let mut rom = edit.rom.borrow_mut();
        let kind = self.id.last();
        match kind {
            "overworldtile" => {
                let ocfg = config.overworld.find(&self.id)?;
                for (&k, v) in self.tile.iter() {
                    rom.write_bytes(ocfg.objtable + k * 4, v)?;
                    if let Some(&p) = self.palette.get(&k) {
                        rom.write(ocfg.tile_palette + k, p as u8)?;
                    }
                }
            }
            "metatile" => {
                let scfg = config.sideview.find(&self.id)?;
                for (k, v) in self.tile.iter() {
                    let t = k / 64;
                    let base = rom.read_pointer(scfg.metatile_table + t * 2)?;
                    rom.write_bytes(base + (k % 64) * 4, v)?;
                }
            }
            _ => {
                return Err(
                    ErrorKind::IdPathError(format!("Unknown Metatile type {}", kind)).into(),
                );
            }
        }
        Ok(())
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MetatileGroup {
    pub data: Vec<Metatile>,
}

impl MetatileGroup {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }

    pub fn from_rom(edit: &Rc<Edit>) -> Result<Self> {
        let mut ret = Self::default();
        ret.unpack(edit)?;
        Ok(ret)
    }

    pub fn merge(&mut self, other: &MetatileGroup) {
        for o in other.data.iter() {
            for d in self.data.iter_mut() {
                if d.id == o.id {
                    *d = o.clone();
                }
            }
        }
    }
}

#[typetag::serde]
impl RomData for MetatileGroup {
    fn name(&self) -> String {
        "MetatileGroup".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn gui(&self, project: &Project, edit: &Rc<Edit>) -> Result<Box<dyn Gui>> {
        MetatileGroupGui::new(project, Some(Rc::clone(edit)))
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let mut seen = HashSet::new();
        self.data.clear();
        for map in config.overworld.map.iter() {
            if !seen.contains(&map.objtable) {
                seen.insert(map.objtable);
                self.data
                    .push(Metatile::from_rom(edit, map.id.extend("overworldtile"))?);
            }
        }
        for sv in config.sideview.group.iter() {
            if sv.background_layer {
                continue;
            }
            if !seen.contains(&sv.metatile_table) {
                seen.insert(sv.metatile_table);
                self.data
                    .push(Metatile::from_rom(edit, sv.id.extend("metatile"))?);
            }
        }
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        for metatile in self.data.iter() {
            metatile.pack(edit)?;
        }
        Ok(())
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
