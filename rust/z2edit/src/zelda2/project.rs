use anyhow::{Context, Result};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::path::Path;

use crate::error::Error;
use crate::nes::NesFile;
use crate::util::time::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::edit::{EditList, EditProxy, GameData};
use crate::zelda2::rom::FileResource;
use crate::AppPreferences;

#[derive(Debug, Serialize)]
#[pyclass(sequence)]
pub struct Project {
    #[pyo3(get, set)]
    pub name: String,
    pub start: FileResource,
    pub configuration: String,
    #[pyo3(get, set)]
    pub fixups: bool,
    #[serde(serialize_with = "serialize_editlist")]
    pub edits: EditList,

    #[serde(skip)]
    #[pyo3(get)]
    pub rom: Py<NesFile>,
    #[serde(skip)]
    pub config: Config,
}

#[derive(Debug, Default, Deserialize)]
struct LoadFile {
    name: String,
    start: FileResource,
    configuration: String,
    fixups: bool,
    edits: EditList,
}

thread_local! {
    static FILTER_EDITLIST: Cell<bool> = Cell::new(true);
}

fn serialize_editlist<S>(edits: &EditList, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(edits.len()))?;
    for (k, v) in edits.iter() {
        if !FILTER_EDITLIST.get() || v.meta.timestamp != 0 {
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
        Python::with_gil(|py| self.config.unpack(self.rom.bind(py), "", &mut edits))?;
        Ok(edits)
    }

    fn setup(mut self) -> Result<Self> {
        let config = Config::named(&self.configuration).ok_or(Error::NotFound(format!(
            "configuration {:?}",
            self.configuration
        )))?;

        self.config = config;
        let mut rom = match self.start {
            FileResource::Vanilla() => NesFile::load(&AppPreferences::get().vanilla_rom)?,
            FileResource::File(ref f) => NesFile::load(f)?,
        };
        rom.register(&self.config.global.freespace)?;
        for bank in self.config.bank.values() {
            rom.register(&bank.freespace)?;
        }
        log::info!("{}", rom.report());
        self.rom = Python::with_gil(|py| Py::new(py, rom))?;
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
        let data = serde_annotate::from_str::<LoadFile>(&data)
            .with_context(|| format!("Could not parse {path:?}"))?;
        let rom = Python::with_gil(|py| Py::new(py, NesFile::default()))?;
        let project = Project {
            name: data.name,
            start: data.start,
            configuration: data.configuration,
            fixups: data.fixups,
            edits: data.edits,
            rom,
            config: Config::default(),
        };
        project.setup()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P, filter: bool) -> Result<()> {
        let path = path.as_ref();
        FILTER_EDITLIST.set(filter);
        let doc = serde_annotate::serialize(self);
        FILTER_EDITLIST.set(true);
        let doc = doc?.to_json5().to_string();
        std::fs::write(path, &doc).with_context(|| format!("Could not write {path:?}"))?;
        Ok(())
    }

    fn pack(&self) -> Result<NesFile> {
        Python::with_gil(|py| {
            let rom = Py::new(py, self.rom.borrow(py).clone())?;
            self.config.pack(rom.bind(py), "", &self.edits)?;
            Ok(rom.extract(py)?)
        })
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
    pub fn new<'p>(
        py: Python<'p>,
        name: &str,
        start: FileResource,
        configuration: &str,
        fixups: bool,
    ) -> Result<Self> {
        let project = Project {
            name: name.into(),
            start,
            configuration: configuration.into(),
            fixups,
            edits: EditList::default(),
            rom: Py::new(py, NesFile::default())?,
            config: Config::default(),
        };
        project.setup()
    }

    #[staticmethod]
    #[pyo3(name = "load")]
    fn _load(path: &str) -> Result<Self> {
        Self::load(path)
    }

    #[pyo3(
        name = "save",
        signature = (path, filter=true)
    )]
    fn _save(&self, path: &str, filter: bool) -> Result<()> {
        self.save(path, filter)
    }

    #[pyo3(name = "export_rom")]
    pub fn _export_rom(&self, path: &str) -> Result<()> {
        self.export_rom(path)
    }

    #[getter]
    fn get_config(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.config)?)
    }

    #[setter]
    fn set_config(&mut self, json: &str) -> Result<()> {
        self.config = serde_annotate::from_str(json)?;
        Ok(())
    }

    #[pyo3(name = "edits")]
    fn _edits(&self) -> Vec<String> {
        self.edits.keys().map(|s| s.clone()).collect()
    }

    fn __getitem__(self_: PyRef<'_, Self>, key: &str) -> Result<EditProxy> {
        if self_.edits.contains_key(key) {
            Ok(EditProxy::new(self_.into(), key.into()))
        } else {
            Err(PyKeyError::new_err(key.to_string()).into())
        }
    }

    fn __delitem_(&mut self, key: &str) -> Result<()> {
        self.edits
            .shift_remove(key)
            .ok_or(PyKeyError::new_err(key.to_string()))?;
        Ok(())
    }
}
