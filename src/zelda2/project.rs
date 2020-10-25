use std::path::Path;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::fs::File;
use std::rc::Rc;
use std::vec::Vec;
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use whoami;
use ron;


use crate::errors::*;
use crate::util::pyaddress::PyAddress;
use crate::nes::Buffer;
//use crate::nes::MemoryAccess;
use crate::zelda2::import::ImportRom;
use crate::util::UTime;
use crate::gui::zelda2::Gui;

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
        let commit = Edit {
            meta: RefCell::new(meta),
            edit: RefCell::new(ImportRom::from_file(filename)?),
            rom: RefCell::default(),
            action: RefCell::default(),
        };
        let project = Project {
            edits: vec![Rc::new(commit)],
        };
        project.replay(0, -1)?;
        Ok(project)
    }

    pub fn from_reader(r: impl Read) -> Result<Self> {
        let project: Project = ron::de::from_reader(r)?;
        project.replay(0, -1)?;
        Ok(project)
    }

    pub fn from_file(filepath: &Path) -> Result<Self> {
        let file = File::open(filepath)?;
        Project::from_reader(file)
    }

    pub fn to_writer(&self, w: &mut impl Write) -> Result<()> {
        let pretty = ron::ser::PrettyConfig::new();
        ron::ser::to_writer_pretty(w, self, pretty)?;
        Ok(())
    }

    pub fn to_file(&self, filepath: &Path) -> Result<()> {
        let mut file = File::create(filepath)?;
        self.to_writer(&mut file)
    }


    fn normalized_index(&self, index: isize) -> Result<usize> {
        let len = self.edits.len() as isize;
        if index == -1 {
            Ok((len-1) as usize)
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
            if i > 0 {
                let last = &self.edits[i - 1];
                //info!("Project::replay: {}.unpack", commit.edit.borrow().name());
                //commit.edit.borrow_mut().unpack(last)?;
                commit.rom.replace(last.rom.borrow().clone());
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

            let commit = Edit {
                meta: RefCell::new(meta),
                edit: RefCell::new(edit),
                rom: last.rom.clone(),
                action: RefCell::default(),
            };
            commit.edit.borrow().pack(&commit)?;
            self.edits.push(Rc::new(commit));
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
            commit.edit.borrow().pack(&commit)?;
            self.replay(index, -1)?;
            Ok(index)
        } else {
            Err(ErrorKind::CommitIndexError(index).into())
        }
    }
}

#[typetag::serde(tag="type")]
pub trait RomData
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> String;
    fn unpack(&mut self, edit: &Edit) -> Result<()>;
    fn pack(&self, edit: &Edit) -> Result<()>;
    fn gui(&self, _project: &Project, _commit_index: isize) -> Result<Box<dyn Gui>> {
        Err(ErrorKind::NoGuiImplemented(self.name()).into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub action: RefCell<EditAction>,
}

#[pymethods]
impl Project {
    #[new]
    fn new(filename: String) -> PyResult<Self> {
        Project::from_rom(&filename).map_err(|e| e.into())
    }

    fn read_byte(&self, addr: PyAddress) -> Result<u8> {
        Ok(0)
        //self.rom.borrow().read(addr.address)
    }

    fn read_word(&self, addr: PyAddress) -> Result<u16> {
        Ok(0)
        //self.rom.borrow().read_word(addr.address)
    }

    fn read_bytes(&self, addr: PyAddress, len: usize) -> Result<Vec<u8>> {
        //let rom = self.rom.borrow();
        //let bytes = rom.read_bytes(addr.address, len)?.to_vec();
        let bytes = vec![0u8];
        Ok(bytes)
    }

    fn write_byte(&self, addr: PyAddress, val: u8) -> Result<()> {
        Ok(())
        //self.rom.borrow_mut().write(addr.address, val)
    }

    fn write_word(&self, addr: PyAddress, val: u16) -> Result<()> {
        Ok(())
        //self.rom.borrow_mut().write_word(addr.address, val)
    }

    fn write_bytes(&self, addr: PyAddress, val: &[u8]) -> Result<()> {
        Ok(())
        //self.rom.borrow_mut().write_bytes(addr.address, val)
    }
}
