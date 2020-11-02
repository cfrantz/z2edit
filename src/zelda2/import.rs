use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

use crate::errors::*;
use crate::nes::Buffer;
use crate::nes::IdPath;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, RomData};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImportRom {
    pub id: IdPath,
    pub filename: String,
}

impl ImportRom {
    pub fn from_file(filename: &str) -> Result<Box<ImportRom>> {
        if Path::new(filename).is_file() {
            Ok(Box::new(ImportRom {
                id: IdPath::default(),
                filename: filename.to_owned(),
            }))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, filename).into())
        }
    }
}

#[typetag::serde]
impl RomData for ImportRom {
    fn name(&self) -> String {
        "ImportRom".to_owned()
    }

    fn unpack(&mut self, _edit: &Edit) -> Result<()> {
        Ok(())
    }

    fn pack(&self, edit: &Edit) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let path = Path::new(&self.filename);
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
            Ok(v) => { *self = v; Ok(()) },
            Err(e) => Err(e.into()),
        }
    }
}
