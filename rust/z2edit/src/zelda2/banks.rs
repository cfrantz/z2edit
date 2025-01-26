use std::any::Any;

use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::nes::NesFile;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::EditList;

pub mod config {
    use super::*;
    use crate::zelda2::enemies::config::EnemyGroup;
    use crate::zelda2::items::config::Items;
    use crate::zelda2::palette::config::PaletteGroup;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct GameBank {
        pub palette: IndexMap<String, PaletteGroup>,
        pub enemy: IndexMap<String, EnemyGroup>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct GlobalBank {
        pub item: Items,
        pub palette: IndexMap<String, PaletteGroup>,
    }
}

impl config::GameBank {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("GameBank::unpack {path}");
        for (k, v) in self.palette.iter() {
            v.unpack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        for (k, v) in self.enemy.iter() {
            v.unpack(rom, &format!("{path}/enemy/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GameBank::pack {path}");
        for (k, v) in self.palette.iter() {
            v.pack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        for (k, v) in self.enemy.iter() {
            v.pack(rom, &format!("{path}/enemy/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            ["palette", ref n, ..] => {
                let palette = self
                    .palette
                    .get(*n)
                    .ok_or(Error::NotFound(format!("palette/{n}")))?;
                palette.get::<T>(&path[2..])
            }
            ["enemy", ref n, ..] => {
                let enemy = self
                    .enemy
                    .get(*n)
                    .ok_or(Error::NotFound(format!("enemy/{n}")))?;
                enemy.get::<T>(&path[2..])
            }

            _ => Err(Error::NotFound(format!("GameBank/{path:?}")).into()),
        }
    }
}

impl config::GlobalBank {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("GlobalBank::unpack {path}");
        self.item.unpack(rom, &format!("{path}/item"), edits)?;
        for (k, v) in self.palette.iter() {
            v.unpack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GlobalBank::pack {path}");
        self.item.pack(rom, &format!("{path}/item"), edits)?;
        for (k, v) in self.palette.iter() {
            v.pack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            ["item", ..] => self.item.get(&path[1..]),
            ["palette", ref n, ..] => {
                let palette = self
                    .palette
                    .get(*n)
                    .ok_or(Error::NotFound(format!("palette/{n}")))?;
                palette.get::<T>(&path[2..])
            }
            _ => Err(Error::NotFound(format!("GlobalBank/{path:?}")).into()),
        }
    }
}
