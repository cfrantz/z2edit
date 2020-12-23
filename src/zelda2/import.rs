use std::any::Any;
use std::convert::From;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::nes::Buffer;
use crate::nes::IdPath;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, RomData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileResource {
    Unknown,
    Vanilla,
    Name(String),
}

impl Default for FileResource {
    fn default() -> Self {
        FileResource::Unknown
    }
}

impl FileResource {
    pub fn to_pathbuf(&self) -> Result<PathBuf> {
        let pref = AppContext::pref();
        match self {
            FileResource::Unknown => Err(ErrorKind::NotImplemented(
                "No path for FileResource::Unknown".to_string(),
            )
            .into()),
            FileResource::Vanilla => {
                if pref.vanilla_rom.is_empty() {
                    Err(ErrorKind::ConfigError(
                        "Set the location of the vanilla ROM in Prefences".to_string(),
                    )
                    .into())
                } else {
                    Ok(PathBuf::from(&pref.vanilla_rom))
                }
            }
            FileResource::Name(name) => Ok(PathBuf::from(name)),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImportRom {
    pub id: IdPath,
    pub file: FileResource,
}

impl ImportRom {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
    pub fn from_file(filename: &str) -> Result<Box<ImportRom>> {
        ImportRom::new(FileResource::Name(filename.to_owned()))
    }

    pub fn new(file: FileResource) -> Result<Box<ImportRom>> {
        match &file {
            FileResource::Name(filename) => {
                if !Path::new(&filename).is_file() {
                    return Err(ErrorKind::NotFound(filename.clone()).into());
                }
            }
            FileResource::Unknown => {
                return Err(ErrorKind::NotImplemented(format!("Cannot load {:?}", file)).into());
            }
            FileResource::Vanilla => {}
        };

        Ok(Box::new(ImportRom {
            id: IdPath::default(),
            file: file,
        }))
    }
}

#[typetag::serde]
impl RomData for ImportRom {
    fn name(&self) -> String {
        "ImportRom".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, _edit: &Rc<Edit>) -> Result<()> {
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let path = self.file.to_pathbuf()?;
        info!("ImportRom: loading file {:?}", path);
        let rom = Buffer::from_file(&path, Some(config.layout.clone()))?;
        edit.rom.replace(rom);
        Ok(())
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
