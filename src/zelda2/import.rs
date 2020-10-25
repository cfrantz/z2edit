use std::path::Path;
use std::io;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::nes::IdPath;
use crate::nes::Buffer;
use crate::zelda2::project::{Edit, RomData};
use crate::zelda2::config::Config;


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
}
