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
    use crate::zelda2::palette::config::PaletteGroup;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GameBank {
        pub palette: IndexMap<String, PaletteGroup>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GlobalBank {
        pub palette: IndexMap<String, PaletteGroup>,
    }
}

impl config::GameBank {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("GameBank::unpack {path}");
        for (k, v) in self.palette.iter() {
            v.unpack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GameBank::pack {path}");
        for (k, v) in self.palette.iter() {
            v.pack(rom, &format!("{path}/palette/{k}"), edits)?;
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
            _ => Err(Error::NotFound(format!("GameBank/{path:?}")).into()),
        }
    }
}

impl config::GlobalBank {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("GlobalBank::unpack {path}");
        for (k, v) in self.palette.iter() {
            v.unpack(rom, &format!("{path}/palette/{k}"), edits)?;
        }
        Ok(())
    }
    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("GlobalBank::pack {path}");
        for (k, v) in self.palette.iter() {
            v.pack(rom, &format!("{path}/palette/{k}"), edits)?;
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
            _ => Err(Error::NotFound(format!("GlobalBank/{path:?}")).into()),
        }
    }
}
