use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::path::Path;
use std::sync::OnceLock;

use crate::error::Error;
use crate::zelda2::banks::config::{GameBank, GlobalBank};
use crate::zelda2::chr::config::ChrMemory;
use crate::zelda2::edit::EditList;
use crate::zelda2::object::RenderInfo;
use crate::zelda2::vchr::config::VirtualChr;
use nes::NesFile;

static mut CONFIGS: OnceLock<IndexMap<String, Config>> = OnceLock::new();

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
#[pyclass]
pub struct Config {
    #[pyo3(get)]
    pub name: String,
    pub include: Vec<String>,
    pub include_render_info: Vec<String>,
    pub chr: ChrMemory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vchr: Option<VirtualChr>,
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
        for i in game.include_render_info.iter() {
            path.set_file_name(i);
            let data = std::fs::read_to_string(&path)?;
            let data = serde_annotate::from_str::<IndexMap<String, RenderInfo>>(&data)?;
            game.global.render.extend(data);
        }

        Ok(game)
    }

    fn _get() -> &'static IndexMap<String, Config> {
        unsafe { CONFIGS.get_or_init(IndexMap::default) }
    }

    fn _get_mut() -> &'static mut IndexMap<String, Config> {
        let _ = Self::_get();
        unsafe { CONFIGS.get_mut().unwrap() }
    }
}

#[pymethods]
impl Config {
    #[staticmethod]
    #[pyo3(name = "load")]
    fn _load(path: &str) -> Result<()> {
        let cfg = Self::load(path)?;
        Self::_get_mut().insert(cfg.name.clone(), cfg);
        Ok(())
    }

    #[staticmethod]
    pub fn keys() -> Vec<String> {
        Self::_get().keys().map(|s| s.clone()).collect()
    }

    #[staticmethod]
    pub fn named(name: &str) -> Option<Config> {
        Self::_get().get(name).cloned()
    }

    #[staticmethod]
    pub fn insert(json: &str) -> Result<()> {
        let cfg = serde_annotate::from_str::<Config>(json)?;
        Self::_get_mut().insert(cfg.name.clone(), cfg);
        Ok(())
    }

    #[getter]
    pub fn get_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
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
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Config unpack {path}");
        self.chr.unpack(rrom, &format!("{path}/chr"), edits)?;
        for vchr in self.vchr.iter() {
            vchr.unpack(rrom, &format!("{path}/vchr"), edits)?;
        }
        for (k, v) in self.bank.iter() {
            v.unpack(rrom, &format!("{path}/bank/{k}"), edits)?;
        }
        self.global.unpack(rrom, &format!("{path}/global"), edits)?;
        Ok(())
    }
    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Config pack {path}");
        self.chr.pack(rrom, &format!("{path}/chr"), edits)?;
        for vchr in self.vchr.iter() {
            vchr.pack(rrom, &format!("{path}/vchr"), edits)?;
        }
        for (k, v) in self.bank.iter() {
            v.pack(rrom, &format!("{path}/bank/{k}"), edits)?;
        }
        self.global.pack(rrom, &format!("{path}/global"), edits)?;
        Ok(())
    }

    pub fn post_unpack_fixup(&self, path: &str, edits: &mut EditList) -> Result<()> {
        self.chr.post_unpack_fixup(&format!("{path}/chr"), edits)?;
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
            ["vchr", ..] => {
                let vchr = self.vchr.as_ref().ok_or(Error::NotFound(format!("vchr")))?;
                vchr.get::<T>(&path[1..])
            }
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
