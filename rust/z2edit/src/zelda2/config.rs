use std::any::Any;
use std::path::Path;

use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::nes::NesFile;
use crate::zelda2::banks::config::{GameBank, GlobalBank};
use crate::zelda2::chr::config::ChrMemory;
use crate::zelda2::edit::EditList;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
#[pyclass]
pub struct Config {
    pub name: String,
    pub include: Vec<String>,
    pub chr: ChrMemory,
    pub bank: IndexMap<String, GameBank>,
    pub global: GlobalBank,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut path = path.as_ref().to_owned();
        let data = std::fs::read_to_string(&path)?;
        let mut game = serde_annotate::from_str::<Config>(&data)?;
        for i in game.include.iter() {
            path.set_file_name(i);
            let data = std::fs::read_to_string(&path)?;
            let data = serde_annotate::from_str::<Config>(&data)?;
            game.bank.extend(data.bank);
        }
        Ok(game)
    }
}

pub(super) fn get_config<T: Any>(item: &dyn Any) -> Result<&T> {
    item.downcast_ref::<T>().ok_or_else(|| {
        Error::Cast(format!(
            "cannot cast config item to {}",
            std::any::type_name::<T>()
        ))
        .into()
    })
}

impl Config {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("Config unpack {path}");
        self.chr.unpack(rom, &format!("{path}/chr"), edits)?;
        for (k, v) in self.bank.iter() {
            v.unpack(rom, &format!("{path}/bank/{k}"), edits)?;
        }
        self.global.unpack(rom, &format!("{path}/global"), edits)?;
        Ok(())
    }
    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Config pack {path}");
        self.chr.pack(rom, &format!("{path}/chr"), edits)?;
        for (k, v) in self.bank.iter() {
            v.pack(rom, &format!("{path}/bank/{k}"), edits)?;
        }
        self.global.pack(rom, &format!("{path}/global"), edits)?;
        Ok(())
    }

    pub fn get<T: Any>(&self, path: &str) -> Result<&T> {
        let path = path
            .trim_start_matches('/')
            .split('/')
            .collect::<Vec<&str>>();
        match path.as_slice() {
            [] => get_config::<T>(self),
            ["chr", ..] => self.chr.get::<T>(&path[1..]),
            ["bank", ref n, ..] => {
                let bank = self
                    .bank
                    .get(*n)
                    .ok_or(Error::NotFound(format!("bank/{n}")))?;
                bank.get::<T>(&path[2..])
            }

            ["global", ..] => self.global.get::<T>(&path[1..]),
            _ => Err(Error::NotFound(format!("Game/{path:?}")).into()),
        }
    }
}
