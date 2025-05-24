use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DropTable {
    pub counter: u8,
    pub small: Vec<u8>,
    pub large: Vec<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dropper {
    pub enemy: Option<u8>,
    pub hp: Option<u8>,
    pub item: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DropInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<DropTable>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub dropper: IndexMap<String, Dropper>,
}

#[typetag::serde]
impl GameData for DropInfo {
    fn name(&self) -> String {
        "DropInfo".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::drops::DropInfoEditor::new(self, name)
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
    pub struct DropTable {
        pub counter: Address,
        pub small: Address,
        pub large: Address,
        pub length: usize,
    }

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Dropper {
        pub name: String,
        pub enemy_group: String,
        pub enemy: Option<Address>,
        pub hp: Option<Address>,
        pub item: Option<Address>,
    }

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct DropInfo {
        pub table: Option<DropTable>,
        pub dropper: IndexMap<String, Dropper>,
    }
}

impl config::DropInfo {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("DropInfo::unpack {path}");
        let mut di = DropInfo::default();
        let rom = rrom.borrow();
        if let Some(table) = &self.table {
            di.table = Some(DropTable {
                counter: rom.read(table.counter)?,
                small: rom.read_bytes(table.small, table.length)?.to_vec(),
                large: rom.read_bytes(table.large, table.length)?.to_vec(),
            });
        }
        for (name, drop) in self.dropper.iter() {
            di.dropper.insert(
                name.into(),
                Dropper {
                    enemy: drop.enemy.map(|addr| rom.read(addr)).transpose()?,
                    hp: drop.hp.map(|addr| rom.read(addr)).transpose()?,
                    item: drop.item.map(|addr| rom.read(addr)).transpose()?,
                },
            );
        }
        edits.insert(path.into(), Edit::new(di.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("DropInfo::pack {path}");
        if let Some(edit) = edits.get(path) {
            let di = edit.data_ref::<DropInfo>()?;
            let mut rom = rrom.borrow_mut();
            if let Some(table) = &self.table {
                let values = di
                    .table
                    .as_ref()
                    .ok_or_else(|| anyhow!("Missing drop table at {path:?}"))?;
                rom.write(table.counter, values.counter)?;
                rom.write_bytes(table.small, &values.small)?;
                rom.write_bytes(table.large, &values.large)?;
            }
            for (name, drop) in self.dropper.iter() {
                if let Some(value) = di.dropper.get(name) {
                    drop.enemy
                        .map(|addr| -> Result<()> {
                            rom.write(
                                addr,
                                value.enemy.ok_or_else(|| {
                                    anyhow!("Missing dropper.enemy value for {path}:{name}")
                                })?,
                            )
                        })
                        .transpose()?;
                    drop.hp
                        .map(|addr| -> Result<()> {
                            rom.write(
                                addr,
                                value.hp.ok_or_else(|| {
                                    anyhow!("Missing dropper.hp value for {path}:{name}")
                                })?,
                            )
                        })
                        .transpose()?;
                    drop.item
                        .map(|addr| -> Result<()> {
                            rom.write(
                                addr,
                                value.item.ok_or_else(|| {
                                    anyhow!("Missing dropper.item value for {path}:{name}")
                                })?,
                            )
                        })
                        .transpose()?;
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
            _ => Err(Error::NotFound(format!("DropInfo/{path:?}")).into()),
        }
    }
}
