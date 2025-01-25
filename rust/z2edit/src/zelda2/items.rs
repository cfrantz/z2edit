use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::EditList;

// TODO: decide if we should unpack the items from the items table.

pub mod config {
    use super::*;

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
        pub sprite_table: Address,
        pub item: IndexMap<String, Sprite>,
        pub fake: IndexMap<String, Sprite>,
    }
}

impl config::Items {
    pub fn unpack(&self, _rom: &NesFile, path: &str, _edits: &mut EditList) -> Result<()> {
        log::debug!("Items::unpack {path}");
        Ok(())
    }

    pub fn pack(&self, _rom: &mut NesFile, path: &str, _edits: &EditList) -> Result<()> {
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
