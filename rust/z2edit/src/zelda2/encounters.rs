use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use nes::{Address, NesFile};

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Encounter {
    pub area: u8,
    pub screen: u8,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Encounters {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub north: IndexMap<String, Encounter>,
    pub south: IndexMap<String, Encounter>,
}

#[typetag::serde]
impl GameData for Encounters {
    fn name(&self) -> String {
        "Encounters".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::encounters::EncountersEditor::new(self, name)
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
    pub struct Encounters {
        pub address: Address,
        pub terrain: Vec<String>,
    }
}

impl From<u8> for Encounter {
    fn from(val: u8) -> Self {
        Encounter {
            area: val & 0x3f,
            screen: val >> 6,
        }
    }
}

impl From<Encounter> for u8 {
    fn from(val: Encounter) -> Self {
        (val.area & 0x3f) | (val.screen << 6)
    }
}

impl Encounters {
    pub fn is_encounter(&self, area: u8) -> bool {
        for enc in self.north.values().chain(self.south.values()) {
            if area == enc.area {
                return true;
            }
        }
        return false;
    }
}

impl config::Encounters {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Encounters::unpack {path}");
        let mut enc = Encounters::default();
        let rom = rrom.borrow();
        for (i, terrain) in self.terrain.iter().enumerate() {
            enc.north.insert(
                terrain.clone(),
                Encounter::from(rom.read(self.address + i * 2 + 0)?),
            );
            enc.south.insert(
                terrain.clone(),
                Encounter::from(rom.read(self.address + i * 2 + 1)?),
            );
        }
        edits.insert(path.into(), Edit::new(enc.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Encounters::pack {path}");
        if let Some(edit) = edits.get(path) {
            let enc = edit.data_ref::<Encounters>()?;
            let mut rom = rrom.borrow_mut();
            for (i, terrain) in self.terrain.iter().enumerate() {
                if let Some(&e) = enc.north.get(terrain) {
                    rom.write(self.address + i * 2 + 0, u8::from(e))?;
                }
                if let Some(&e) = enc.south.get(terrain) {
                    rom.write(self.address + i * 2 + 1, u8::from(e))?;
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
            _ => Err(Error::NotFound(format!("Encounters/{path:?}")).into()),
        }
    }
}
