use std::any::Any;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::vec::Vec;
use whoami;

use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::nes::freespace::FreeSpace;
use crate::nes::{Buffer, MemoryAccess};
use crate::util::pyaddress::PyAddress;
use crate::util::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::import::ImportRom;

#[pyclass(unsendable)]
#[derive(Default, Serialize, Deserialize)]
pub struct Project {
    pub edits: Vec<Rc<Edit>>,
}

impl Project {
    pub fn from_rom(filename: &str) -> Result<Self> {
        let meta = Metadata {
            label: "ImportRom".to_owned(),
            user: whoami::username(),
            timestamp: UTime::now(),
            comment: String::default(),
            config: "vanilla".to_owned(),
        };
        let config = Config::get(&meta.config)?;
        let commit = Edit {
            meta: RefCell::new(meta),
            edit: RefCell::new(ImportRom::from_file(filename)?),
            rom: RefCell::default(),
            memory: RefCell::new(FreeSpace::new(&config.misc.freespace)?),
            action: RefCell::default(),
        };
        let project = Project {
            edits: vec![Rc::new(commit)],
        };
        project.replay(0, -1)?;
        Ok(project)
    }

    pub fn from_reader(r: impl Read) -> Result<Self> {
        let project: Project = serde_json::from_reader(r)?;
        project.replay(0, -1)?;
        Ok(project)
    }

    pub fn from_file(filepath: &Path) -> Result<Self> {
        let file = File::open(filepath)?;
        Project::from_reader(file)
    }

    pub fn to_writer(&self, w: &mut impl Write) -> Result<()> {
        serde_json::to_writer_pretty(w, self)?;
        Ok(())
    }

    pub fn to_file(&self, filepath: &Path) -> Result<()> {
        let mut file = File::create(filepath)?;
        self.to_writer(&mut file)
    }

    fn normalized_index(&self, index: isize) -> Result<usize> {
        let len = self.edits.len() as isize;
        if index == -1 {
            Ok((len - 1) as usize)
        } else if index < len {
            Ok(index as usize)
        } else {
            Err(ErrorKind::CommitIndexError(index).into())
        }
    }

    pub fn replay(&self, start: isize, end: isize) -> Result<()> {
        let start = self.normalized_index(start)?;
        let end = self.normalized_index(end)? + 1;
        for i in start..end {
            let commit = &self.edits[i];
            let config = Config::get(&commit.meta.borrow().config)?;
            if i == 0 {
                commit
                    .memory
                    .replace(FreeSpace::new(&config.misc.freespace)?);
            } else {
                let last = &self.edits[i - 1];
                //info!("Project::replay: {}.unpack", commit.edit.borrow().name());
                //commit.edit.borrow_mut().unpack(last)?;
                commit.rom.replace(last.rom.borrow().clone());
                commit.memory.replace(last.memory.borrow().clone());
            }
            info!("Project::replay: {}.pack", commit.edit.borrow().name());
            commit.edit.borrow().pack(&commit)?;
        }
        Ok(())
    }

    pub fn last_commit(&self) -> Rc<Edit> {
        self.get_commit(-1).unwrap()
    }

    pub fn get_commit(&self, index: isize) -> Result<Rc<Edit>> {
        let index = self.normalized_index(index)?;
        Ok(Rc::clone(&self.edits[index]))
    }

    pub fn commit(&mut self, index: isize, edit: Box<dyn RomData>) -> Result<isize> {
        let len = self.edits.len() as isize;
        if index == -1 {
            let last = self.get_commit(index)?;
            let meta = Metadata {
                label: edit.name(),
                user: whoami::username(),
                timestamp: UTime::now(),
                comment: String::default(),
                config: last.meta.borrow().config.clone(),
            };

            let commit = Rc::new(Edit {
                meta: RefCell::new(meta),
                edit: RefCell::new(edit),
                rom: last.rom.clone(),
                memory: last.memory.clone(),
                action: RefCell::default(),
            });
            commit.edit.borrow().pack(&commit)?;
            self.edits.push(commit);
            Ok(len)
        } else if index < len {
            let last = self.get_commit(index - 1)?;
            let commit = self.get_commit(index)?;
            {
                let mut meta = commit.meta.borrow_mut();
                meta.user = whoami::username();
                meta.timestamp = UTime::now();
            }
            commit.edit.replace(edit);
            commit.rom.replace(last.rom.borrow().clone());
            self.replay(index, -1)?;
            Ok(index)
        } else {
            Err(ErrorKind::CommitIndexError(index).into())
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait RomData
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> String;
    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()>;
    fn pack(&self, edit: &Rc<Edit>) -> Result<()>;
    fn gui(&self, _project: &Project, _commit_index: isize) -> Result<Box<dyn Gui>> {
        Err(ErrorKind::NotImplemented(self.name()).into())
    }
    fn to_text(&self) -> Result<String> {
        Err(ErrorKind::NotImplemented(format!("{}::to_text", self.name())).into())
    }
    fn from_text(&mut self, _text: &str) -> Result<()> {
        Err(ErrorKind::NotImplemented(format!("{}::from_text", self.name())).into())
    }
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Serialize, Deserialize, FromPyObject, IntoPyObject)]
pub struct Metadata {
    pub label: String,
    pub user: String,
    pub timestamp: u64,
    pub comment: String,
    pub config: String,
}

#[derive(Debug)]
pub enum EditAction {
    None,
    MoveTo(isize),
    Delete,
    Update,
}

impl Default for EditAction {
    fn default() -> Self {
        EditAction::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edit {
    pub meta: RefCell<Metadata>,
    pub edit: RefCell<Box<dyn RomData>>,
    #[serde(skip)]
    pub rom: RefCell<Buffer>,
    #[serde(skip)]
    pub memory: RefCell<FreeSpace>,
    #[serde(skip)]
    pub action: RefCell<EditAction>,
}

impl Edit {
    pub fn export(&self, filename: &str) -> Result<()> {
        let rom = self.rom.borrow();
        rom.save(Path::new(filename))
    }
}

#[pyclass(unsendable)]
pub struct EditProxy {
    edit: Rc<Edit>,
}

impl EditProxy {
    pub fn new(edit: Rc<Edit>) -> EditProxy {
        EditProxy { edit: edit }
    }
}

#[pymethods]
impl EditProxy {
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.edit.edit.borrow().name())
    }

    #[getter]
    fn get_meta(&self) -> PyResult<Metadata> {
        Ok(self.edit.meta.borrow().clone())
    }

    #[setter]
    fn set_meta(&self, meta: Metadata) -> PyResult<()> {
        self.edit.meta.replace(meta);
        Ok(())
    }

    #[getter]
    fn get_text(&self) -> PyResult<String> {
        self.edit.edit.borrow().to_text().map_err(|e| e.into())
    }

    #[setter]
    fn set_text(&self, text: &str) -> PyResult<()> {
        match self.edit.edit.borrow_mut().from_text(text) {
            Ok(()) => {
                self.edit.action.replace(EditAction::Update);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    fn read_byte(&self, addr: PyAddress) -> Result<u8> {
        self.edit.rom.borrow().read(addr.address)
    }

    fn read_word(&self, addr: PyAddress) -> Result<u16> {
        self.edit.rom.borrow().read_word(addr.address)
    }

    fn read_bytes(&self, addr: PyAddress, len: usize) -> Result<Vec<u8>> {
        let rom = self.edit.rom.borrow();
        let bytes = rom.read_bytes(addr.address, len)?.to_vec();
        Ok(bytes)
    }

    fn write_byte(&self, addr: PyAddress, val: u8) -> Result<()> {
        self.edit.rom.borrow_mut().write(addr.address, val)
    }

    fn write_word(&self, addr: PyAddress, val: u16) -> Result<()> {
        self.edit.rom.borrow_mut().write_word(addr.address, val)
    }

    fn write_bytes(&self, addr: PyAddress, val: &[u8]) -> Result<()> {
        self.edit.rom.borrow_mut().write_bytes(addr.address, val)
    }
}

#[pymethods]
impl Project {
    fn save(&self, filename: &str) -> PyResult<()> {
        self.to_file(&Path::new(filename)).map_err(|e| e.into())
    }
}

#[pyproto]
impl PySequenceProtocol for Project {
    fn __len__(&self) -> usize {
        self.edits.len()
    }

    fn __getitem__(&self, index: isize) -> PyResult<EditProxy> {
        match self.get_commit(index) {
            Ok(edit) => Ok(EditProxy { edit: edit }),
            Err(_) => Err(PyIndexError::new_err("list index out of range")),
        }
    }
}
