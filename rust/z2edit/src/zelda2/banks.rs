use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::nes::NesFile;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::EditList;

pub mod config {
    use super::*;
    use crate::nes::freespace::config::FreeSpace;
    use crate::zelda2::drops::config::DropInfo;
    use crate::zelda2::enemies::config::EnemyGroup;
    use crate::zelda2::experience::config::{EnemyExperience, ExperienceTableGroup};
    use crate::zelda2::items::config::Items;
    use crate::zelda2::misc_hacks::config::Miscellaneous;
    use crate::zelda2::palette::config::PaletteGroup;
    use crate::zelda2::start::config::StartValues;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct GameBank {
        pub drops: Option<DropInfo>,
        pub enemy: IndexMap<String, EnemyGroup>,
        pub palette: IndexMap<String, PaletteGroup>,
        pub freespace: FreeSpace,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct GlobalBank {
        pub item: Items,
        pub drops: DropInfo,
        pub enemy_xp: EnemyExperience,
        pub experience: IndexMap<String, ExperienceTableGroup>,
        pub palette: IndexMap<String, PaletteGroup>,
        pub start: StartValues,
        pub misc: Miscellaneous,
        pub freespace: FreeSpace,
    }
}

impl config::GameBank {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("GameBank::unpack {path}");
        for drop in self.drops.iter() {
            drop.unpack(rrom, &format!("{path}/drops"), edits)?;
        }
        for (k, v) in self.palette.iter() {
            v.unpack(rrom, &format!("{path}/palette/{k}"), edits)?;
        }
        for (k, v) in self.enemy.iter() {
            v.unpack(rrom, &format!("{path}/enemy/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GameBank::pack {path}");
        for drop in self.drops.iter() {
            drop.pack(rrom, &format!("{path}/drops"), edits)?;
        }
        for (k, v) in self.palette.iter() {
            v.pack(rrom, &format!("{path}/palette/{k}"), edits)?;
        }
        for (k, v) in self.enemy.iter() {
            v.pack(rrom, &format!("{path}/enemy/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            ["drops", ..] if self.drops.is_some() => {
                self.drops.as_ref().unwrap().get::<T>(&path[1..])
            }
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
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("GlobalBank::unpack {path}");
        self.misc.unpack(rrom, &format!("{path}/misc"), edits)?;
        self.drops.unpack(rrom, &format!("{path}/drops"), edits)?;
        self.item.unpack(rrom, &format!("{path}/item"), edits)?;
        for (k, v) in self.palette.iter() {
            v.unpack(rrom, &format!("{path}/palette/{k}"), edits)?;
        }
        self.enemy_xp
            .unpack(rrom, &format!("{path}/enemy_xp"), edits)?;
        for (k, v) in self.experience.iter() {
            v.unpack(rrom, &format!("{path}/experience/{k}"), edits)?;
        }
        self.enemy_xp
            .unpack(rrom, &format!("{path}/enemy_xp"), edits)?;
        self.start.unpack(rrom, &format!("{path}/start"), edits)?;
        Ok(())
    }
    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GlobalBank::pack {path}");
        self.misc.pack(rrom, &format!("{path}/misc"), edits)?;
        self.drops.pack(rrom, &format!("{path}/drops"), edits)?;
        self.item.pack(rrom, &format!("{path}/item"), edits)?;
        for (k, v) in self.palette.iter() {
            v.pack(rrom, &format!("{path}/palette/{k}"), edits)?;
        }
        for (k, v) in self.experience.iter() {
            v.pack(rrom, &format!("{path}/experience/{k}"), edits)?;
        }
        self.enemy_xp
            .pack(rrom, &format!("{path}/enemy_xp"), edits)?;
        self.start.pack(rrom, &format!("{path}/start"), edits)?;
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            ["misc", ..] => self.misc.get::<T>(&path[1..]),
            ["drops", ..] => self.drops.get::<T>(&path[1..]),
            ["item", ..] => self.item.get(&path[1..]),
            ["palette", ref n, ..] => {
                let palette = self
                    .palette
                    .get(*n)
                    .ok_or(Error::NotFound(format!("palette/{n}")))?;
                palette.get::<T>(&path[2..])
            }
            ["enemy_xp", ..] => self.enemy_xp.get(&path[1..]),
            ["experience", ref n, ..] => {
                let experience = self
                    .experience
                    .get(*n)
                    .ok_or(Error::NotFound(format!("experience/{n}")))?;
                experience.get::<T>(&path[2..])
            }
            ["start", ..] => self.start.get(&path[1..]),
            _ => Err(Error::NotFound(format!("GlobalBank/{path:?}")).into()),
        }
    }
}
