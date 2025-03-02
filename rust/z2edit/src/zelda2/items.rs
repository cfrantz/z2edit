use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    #[serde(default)]
    pub offset: u8,
    pub chr: Address,
    pub palette: u8,
    pub size: [u32; 2],
    #[serde(default)]
    pub sprites: Vec<i32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Items {
    pub item: IndexMap<u8, Sprite>,
}

#[typetag::serde]
impl GameData for Items {
    fn name(&self) -> String {
        "Items".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        Err(Error::NotImplemented(format!("editor for {name}")).into())
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
    pub struct Items {
        pub sprite_table: Address,
        pub item: IndexMap<String, Sprite>,
        pub fake: IndexMap<String, Sprite>,
    }
}

impl config::Items {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Items::unpack {path}");
        let rom = rrom.borrow();
        let mut items = Items::default();
        for it in self.item.values() {
            let mut it = it.clone();
            let a = rom.read(self.sprite_table + it.offset * 2)? as i32;
            let mut b = rom.read(self.sprite_table + it.offset * 2 + 1)? as i32;
            if a == b {
                b |= 0x0100_0000;
            }
            it.sprites.push(a);
            it.sprites.push(b);
            items.item.insert(it.offset, it);
        }
        edits.insert(path.into(), Edit::new(items.into()));
        Ok(())
    }

    pub fn pack(&self, _rrom: &Bound<'_, NesFile>, path: &str, _edits: &EditList) -> Result<()> {
        log::debug!("Items::pack {path}");
        Ok(())
    }

    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            ["fake", n] => {
                let sprite = self
                    .fake
                    .get(*n)
                    .ok_or(Error::NotFound(format!("Items/fake/{n}")))?;
                get_config::<T>(sprite)
            }
            [n] => {
                let sprite = self
                    .item
                    .get(*n)
                    .ok_or(Error::NotFound(format!("Items/{n}")))?;
                get_config::<T>(sprite)
            }
            _ => Err(Error::NotFound(format!("Items/{path:?}")).into()),
        }
    }
}
