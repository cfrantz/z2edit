use anyhow::{Context, Result};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::Error;
use crate::nes::Address;
use crate::nes::NesFile;
use crate::util::time::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::connectivity::Connectivity;
use crate::zelda2::edit::{Edit, EditList, EditProxy, GameData};
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
    #[serde(skip)]
    pub connectivity: Connectivity,
    #[serde(skip)]
    #[pyo3(get, set)]
    pub project_path: PathBuf,
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
    static PROJECT_PATH: RefCell<PathBuf> = RefCell::default();
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
        self.connectivity.scan(&self)?;
        self.connectivity.report();
        Ok(self)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let project_path =
            path.canonicalize()?
                .parent()
                .map(|p| p.to_owned())
                .ok_or(Error::NotFound(format!(
                    "Cannot find parent path of {path:?}"
                )))?;
        log::info!("Project path is {project_path:?}");

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
            connectivity: Connectivity::default(),
            project_path,
        };
        project.setup()
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P, filter: bool) -> Result<()> {
        let path = path.as_ref();
        let project_path =
            path.canonicalize()?
                .parent()
                .map(|p| p.to_owned())
                .ok_or(Error::NotFound(format!(
                    "Cannot find parent path of {path:?}"
                )))?;
        log::info!("Project path is {project_path:?}");
        for edit in self.edits.values_mut() {
            edit.fixup_paths(&project_path)?;
        }
        self.project_path = project_path;

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
            PROJECT_PATH.replace(self.project_path.clone());
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

    pub fn data_ref<T: GameData>(&self, path: &str) -> Result<&T> {
        let edit = self
            .edits
            .get(path)
            .ok_or_else(|| Error::NotFound(path.into()))?;
        edit.data_ref::<T>()
    }

    pub fn data_mut<T: GameData>(&mut self, path: &str) -> Result<&mut T> {
        let edit = self
            .edits
            .get_mut(path)
            .ok_or_else(|| Error::NotFound(path.into()))?;
        edit.data_mut::<T>()
    }

    pub fn update_timestamp(&mut self, path: &str) -> Result<()> {
        let edit = self
            .edits
            .get_mut(path)
            .ok_or_else(|| Error::NotFound(path.into()))?;
        edit.meta.timestamp = UTime::now();
        Ok(())
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

    pub fn insert(&mut self, path: String, data: Box<dyn GameData>) {
        let mut edit = Edit::from_data(data);
        edit.meta.timestamp = UTime::now();
        edit.meta.user = whoami::username();
        self.edits.insert(path, edit);
    }

    fn emulator_prepare(&self, sideview_path: &str, rom: &mut NesFile) -> Result<()> {
        use crate::zelda2::overworld::{config, Overworld};
        use crate::zelda2::sideview::config::SideviewAreas;
        if let Some(ovid) = self.connectivity.get(sideview_path) {
            // The overworld path stored in the connectivity data is of the
            // form: /bank/<num>/overworld/<index>/<connector>
            let (ovpath, connector) = ovid.rsplit_once('/').expect("overworld connector");
            let ovcfg = self
                .config
                .get::<config::Overworld>(ovpath)
                .context(format!("Find overworld config for {ovid}"))?;
            let overworld = self
                .data_ref::<Overworld>(ovpath)
                .context(format!("Find overworld for {ovid}"))?;
            let mut connector = connector.parse::<u8>()?;

            let svcfg = self
                .config
                .get::<SideviewAreas>(sideview_path)
                .context(format!("Find sideview config for {sideview_path}"))?;
            // The sideview path will be of the form:
            // /bank/<num>/sideview/<index>/<area>/<screen>
            let path = sideview_path
                .trim_start_matches('/')
                .split('/')
                .collect::<Vec<_>>();
            let bank = path[1].parse::<u8>()?;
            let area = path[4].parse::<u8>()?;
            // If the sideview path includes a screen number, use it.
            let screen = if path.len() == 6 {
                path[5].parse::<u8>()?
            } else {
                overworld.connection[connector as usize].screen
            };
            let mut world = svcfg.world;
            if world == 1 && ovcfg.overworld == 2 {
                // Overworld 2 towns are world 2, but the editor models all towns as world 1.
                world += 1;
            }
            let town_code = ovcfg.town_code(connector).map(|code| code as u8);
            if town_code.is_some() {
                // Always use the even connector for towns.
                connector &= 0xFE;
            }
            let at = EmulateAt {
                bank,
                overworld: if ovcfg.subworld != 0 {
                    ovcfg.subworld
                } else {
                    ovcfg.overworld
                },
                world,
                town_code: town_code.unwrap_or(0),
                palace_code: ovcfg
                    .palace_code(connector)
                    .map(|code| code as u8)
                    .unwrap_or(0),
                connector,
                area,
                screen,
                prev_overworld: ovcfg.overworld,
            };
            at.patch(rom)
        } else {
            Err(Error::NotFound(format!("No overworld connector for for {sideview_path}")).into())
        }
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
            connectivity: Connectivity::default(),
            project_path: PathBuf::default(),
        };
        project.setup()
    }

    #[staticmethod]
    #[pyo3(name = "load")]
    fn _load(path: &str) -> Result<Self> {
        Self::load(path)
    }

    #[staticmethod]
    pub fn path() -> PathBuf {
        PROJECT_PATH.with_borrow(|path| path.clone())
    }

    #[pyo3(
        name = "save",
        signature = (path, filter=true)
    )]
    fn _save(&mut self, path: &str, filter: bool) -> Result<()> {
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

    #[pyo3(signature = (sideview_path=None))]
    pub fn emulate(&self, sideview_path: Option<&str>) -> Result<()> {
        let mut tmp = std::env::temp_dir();
        tmp.push("z2edit");
        std::fs::create_dir_all(&tmp).with_context(|| format!("Creating {:?}", tmp))?;
        tmp.push(format!("{}.nes", self.name));
        log::info!("Emulate filename: {tmp:?}");

        let mut rom = self
            .pack()
            .with_context(|| format!("Packing {}", self.name))?;
        log::info!("ROM packed");
        if let Some(path) = sideview_path {
            self.emulator_prepare(path, &mut rom)?;
            log::info!("ROM patched");
        }
        rom.save(&tmp).with_context(|| format!("Saving {tmp:?}"))?;
        log::info!("ROM saved");

        let mut emulator = shellwords::split(&AppPreferences::get().emulator)?;
        emulator.push(tmp.to_str().unwrap().into());
        log::info!("Spawn: {emulator:?}");
        Command::new(&emulator[0]).args(&emulator[1..]).spawn()?;
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

#[derive(Debug, Default, Clone)]
#[pyclass]
#[pyo3(get_all, set_all)]
pub struct EmulateAt {
    pub bank: u8,
    pub overworld: u8,
    pub world: u8,
    pub town_code: u8,
    pub palace_code: u8,
    pub connector: u8,
    pub area: u8,
    pub screen: u8,
    pub prev_overworld: u8,
}

impl EmulateAt {
    fn patch(&self, rom: &mut NesFile) -> Result<()> {
        let facing = if self.screen < 3 { 0 } else { 1 };
        #[rustfmt::skip]
        let code = [
            0xa9, self.bank,        // LDA #bank
            0x8d, 0x69, 0x07,       // STA $0769
            0xa9, self.overworld,   // LDA #overworld
            0x8d, 0x06, 0x07,       // STA $0706
            0xa9, self.world,       // LDA #world
            0x8d, 0x07, 0x07,       // STA $0707
            0xa9, self.town_code,   // LDA #town_code
            0x8d, 0x6b, 0x05,       // STA $056b
            0xa9, self.palace_code, // LDA #palace_code
            0x8d, 0x6c, 0x05,       // STA $056c
            0xa9, self.connector,   // LDA #connector
            0x8d, 0x48, 0x07,       // STA $0748
            0xa9, self.area,        // LDA #area
            0x8d, 0x61, 0x05,       // STA $0561
            0xa9, self.screen,      // LDA #screen
            0x8d, 0x5c, 0x07,       // STA $075c
            0xa9, facing,           // LDA #facing
            0x8d, 0x01, 0x07,       // STA $0701
            0xa9, self.prev_overworld, // LDA #prev_overworld
            0x8d, 0x0a, 0x07,       // STA $070a
            0x60,                   // RTS
        ];
        let addr = Address::Prg(0, 0xAA3F);
        rom.write_bytes(addr, &code)?;
        Ok(())
    }
}
