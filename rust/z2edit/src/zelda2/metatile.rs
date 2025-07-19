use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_annotate::Annotate;
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use nes::{Address, NesFile};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize, Annotate)]
pub struct Metatile {
    #[annotate(format = hex)]
    pub tile: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub palette: Vec<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MetatileGroup {
    pub group: IndexMap<usize, Metatile>,
}

#[typetag::serde]
impl GameData for MetatileGroup {
    fn name(&self) -> String {
        "MetatileGroup".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::metatile::MetatileGroupEditor::new(self, name)
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

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct MetatileGroup {
        pub name: String,
        /// Zero if address points directly, at a metatile group,
        /// else, n if address points at a list of pointers.
        pub pointers: usize,
        pub address: Address,
        /// Lengths of the metatile list.  Must be either 1 or `pointers` long.
        pub length: Vec<usize>,
        /// Only valid for the case where pointers == 0.  Address of a palette index for each
        /// metatile in the group (e.g. overworld tiles).
        pub tile_palette: Option<Address>,
        /// Idpath of palettes used to color these metatiles.
        pub palette: String,
    }
}

impl config::MetatileGroup {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("MetatileGroup::unpack {path}");
        let mut mtg = MetatileGroup::default();
        let rom = rrom.borrow();
        let addrs = if self.pointers == 0 {
            vec![self.address]
        } else {
            (0..self.pointers)
                .map(|i| rom.read_pointer(self.address + i * 2))
                .collect::<Result<Vec<_>>>()?
        };
        for (i, (&addr, &length)) in addrs.iter().zip(self.length.iter()).enumerate() {
            let mut mt = Metatile::default();
            for j in 0..length {
                mt.tile.push(u32::from_be_bytes(
                    rom.read_bytes(addr + j * 4, 4)?.try_into()?,
                ));
                if let Some(tp) = self.tile_palette {
                    mt.palette.push(rom.read(tp + j)?);
                }
            }
            mtg.group.insert(i, mt);
        }
        edits.insert(path.into(), Edit::new(mtg.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("MetatileGroup::pack {path}");
        if let Some(edit) = edits.get(path) {
            let mtg = edit.data_ref::<MetatileGroup>()?;
            let mut rom = rrom.borrow_mut();
            let addrs = if self.pointers == 0 {
                vec![self.address]
            } else {
                (0..self.pointers)
                    .map(|i| rom.read_pointer(self.address + i * 2))
                    .collect::<Result<Vec<_>>>()?
            };
            log::error!("MetatileGroup::pack {addrs:x?}");
            for (i, &addr) in addrs.iter().enumerate() {
                let group = mtg.group.get(&i).ok_or(Error::NotFound(format!(
                    "group {i} in MetatileGroup:{path}"
                )))?;
                for (j, tile) in group.tile.iter().enumerate() {
                    rom.write_bytes(addr + j * 4, &tile.to_be_bytes())?;
                    if let Some(tp) = self.tile_palette {
                        rom.write(tp + j, group.palette[j])?;
                    }
                }
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("MetatileGroup:{path:?}")).into()),
        }
    }
}
