use anyhow::Result;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::chr::{ChrSchema, Layout};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use nes::{Address, NesFile};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VirtualChr {
    pub schema: ChrSchema,
    pub data: Vec<u8>,
    pub layout: Layout,
    pub border: i32,
}

#[typetag::serde]
impl GameData for VirtualChr {
    fn name(&self) -> String {
        "VirtualChr".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::vchr::VirtualChrEditor::new(self, name)
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
    pub struct VirtualChr {
        pub schema: ChrSchema,
        pub address: Address,
        pub banks: usize,
    }
}

impl config::VirtualChr {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("ChrMemory::unpack {path}");
        let rom = rrom.borrow();
        match self.schema {
            ChrSchema::Mmc1_4k => {
                return Err(Error::Configuration(format!(
                    "VirtualChr does not support schema {:?}",
                    self.schema
                ))
                .into())
            }
            ChrSchema::Mmc5_1k => {
                for bank in 0..self.banks {
                    let mut data = Vec::new();
                    data.extend(rom.read_bytes(self.address + 4 * (0 * self.banks + bank), 4)?);
                    data.extend(rom.read_bytes(self.address + 4 * (1 * self.banks + bank), 4)?);
                    data.extend(rom.read_bytes(self.address + 4 * (2 * self.banks + bank), 4)?);
                    let vchr = VirtualChr {
                        schema: self.schema,
                        data,
                        ..Default::default()
                    };
                    log::debug!("VirtualChr::unpack {path}/{bank}");
                    edits.insert(format!("{path}/{bank}"), Edit::new(vchr.into()));
                }
            }
        }
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("ChrMemory::pack {path}");
        match self.schema {
            ChrSchema::Mmc1_4k => {
                return Err(Error::Configuration(format!(
                    "VirtualChr does not support schema {:?}",
                    self.schema
                ))
                .into())
            }
            ChrSchema::Mmc5_1k => {
                let mut rom = rrom.borrow_mut();
                for bank in 0..self.banks {
                    if let Some(edit) = edits.get(&format!("{path}/{bank}")) {
                        let vchr = edit.data_ref::<VirtualChr>()?;
                        rom.write_bytes(
                            self.address + 4 * (0 * self.banks + bank),
                            &vchr.data[0..4],
                        )?;
                        rom.write_bytes(
                            self.address + 4 * (1 * self.banks + bank),
                            &vchr.data[4..8],
                        )?;
                        rom.write_bytes(
                            self.address + 4 * (2 * self.banks + bank),
                            &vchr.data[8..12],
                        )?;
                    } else {
                        log::warn!("No data for {path:?}");
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            [n] if n.parse::<usize>()? < self.banks => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("VirtualChr/{path:?}")).into()),
        }
    }
}
