use anyhow::{Context, Result};
use pyo3::prelude::*;
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::Error;
use crate::nes::NesFile;
use crate::util::time::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::edit::{EditList, GameData};
use crate::zelda2::rom::FileResource;
use crate::AppPreferences;

#[pyclass]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub start: FileResource,
    pub configuration: String,
    pub fixups: bool,
    #[serde(serialize_with = "serialize_editlist")]
    pub edits: EditList,
    #[serde(skip)]
    pub rom: NesFile,
    #[serde(skip)]
    pub config: Config,
}

fn serialize_editlist<S>(edits: &EditList, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(edits.len()))?;
    for (k, v) in edits.iter() {
        if v.meta.timestamp != 0 {
            map.serialize_entry(k, v)?;
        }
    }
    map.end()
}

impl Project {
    fn apply_fixes(&mut self) -> Result<()> {
        Ok(())
    }

    fn unpack(&self) -> Result<EditList> {
        let mut edits = EditList::default();
        self.config.unpack(&self.rom, "", &mut edits)?;
        Ok(edits)
    }

    fn setup(mut self) -> Result<Self> {
        let config = Config::named(&self.configuration).ok_or(Error::NotFound(format!(
            "configuration {:?}",
            self.configuration
        )))?;
        let rom = match self.start {
            FileResource::Vanilla() => NesFile::load(&AppPreferences::get().vanilla_rom)?,
            FileResource::File(ref f) => NesFile::load(f)?,
        };
        self.config = config;
        self.rom = rom;
        self.apply_fixes()?;
        let mut edits = self.unpack()?;
        // Place any edits in the project over the top of what was unpacked
        // from the ROM.
        for (name, edit) in self.edits {
            edits.insert(name, edit);
        }
        self.edits = edits;
        Ok(self)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let data =
            std::fs::read_to_string(path).with_context(|| format!("Could not read {path:?}"))?;
        let project = serde_annotate::from_str::<Self>(&data)
            .with_context(|| format!("Could not parse {path:?}"))?;
        project.setup()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let doc = serde_annotate::serialize(self)?;
        let doc = doc.to_json5().to_string();
        std::fs::write(path, &doc).with_context(|| format!("Could not write {path:?}"))?;
        Ok(())
    }

    fn pack(&self) -> Result<NesFile> {
        let mut rom = self.rom.clone();
        self.config.pack(&mut rom, "", &self.edits)?;
        Ok(rom)
    }

    pub fn export_rom<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let rom = self
            .pack()
            .with_context(|| format!("Packing {}", self.name))?;
        let path = path.as_ref();
        rom.save(path).with_context(|| format!("Saving {path:?}"))
    }

    pub fn commit(&mut self, path: &str, data: Box<dyn GameData>) -> Result<()> {
        let edit = self
            .edits
            .get_mut(path)
            .ok_or_else(|| Error::NotFound(path.into()))?;
        edit.data = data;
        edit.meta.timestamp = UTime::now();
        edit.meta.user = whoami::username();
        Ok(())
    }
}

#[pymethods]
impl Project {
    #[new]
    pub fn new(name: &str, start: FileResource, configuration: &str, fixups: bool) -> Result<Self> {
        let project = Project {
            name: name.into(),
            start,
            configuration: configuration.into(),
            fixups,
            ..Default::default()
        };
        project.setup()
    }

    #[staticmethod]
    #[pyo3(name = "load")]
    fn _load(path: &str) -> Result<Self> {
        Self::load(path)
    }

    #[pyo3(name = "save")]
    fn _save(&self, path: &str) -> Result<()> {
        self.save(path)
    }

    #[pyo3(name = "export_rom")]
    pub fn _export_rom(&self, path: &str) -> Result<()> {
        self.export_rom(path)
    }
}
