use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::palette::PaletteGui;
use crate::gui::zelda2::Gui;
use crate::idpath;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Palette {
        pub id: String,
        pub name: String,
        pub address: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub magic_background: Option<Address>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct PaletteGroup {
        pub id: String,
        pub name: String,
        pub palette: Vec<Palette>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config(pub Vec<config::PaletteGroup>);

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::Palette> {
            path.check_len("palette", 2)?;
            for group in self.0.iter() {
                if path.at(0) == group.id {
                    for palette in group.palette.iter() {
                        if path.at(1) == palette.id {
                            return Ok(palette);
                        }
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn find_sprite(&self, path: &IdPath) -> Result<&config::Palette> {
            let path0 = path.at(0);
            let mut area = match path0 {
                "palace_125" | "palace_346" => "palace".to_owned(),
                "death_mountain" => "west_hyrule".to_owned(),
                "maze_island" => "east_hyrule".to_owned(),
                "item" => "west_hyrule".to_owned(),
                _ => path0.to_owned(),
            };
            area.push_str("_sprites");
            self.find(&idpath!(area, 1))
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/palette.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub id: IdPath,
    pub data: Vec<u8>,
}

impl Palette {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if let Some(id) = id {
            Ok(Box::new(Self {
                id: IdPath::from(id),
                ..Default::default()
            }))
        } else {
            Err(ErrorKind::IdPathError("id required".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for Palette {
    fn name(&self) -> String {
        "Palette".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let pcfg = config.palette.find(&self.id)?;
        let length = pcfg.length.unwrap_or(16);
        self.data = edit.rom.borrow().read_bytes(pcfg.address, length)?.to_vec();
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let pcfg = config.palette.find(&self.id)?;
        edit.rom
            .borrow_mut()
            .write_bytes(pcfg.address, &self.data)?;
        if let Some(bg) = pcfg.magic_background {
            edit.rom.borrow_mut().write(bg, self.data[0])?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaletteGroup {
    pub data: Vec<Palette>,
}

impl PaletteGroup {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for PaletteGroup {
    fn name(&self) -> String {
        "PaletteGroup".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        for d in self.data.iter_mut() {
            d.unpack(edit)?;
        }
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        for d in self.data.iter() {
            d.pack(edit)?;
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        PaletteGui::new(project, commit_index)
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
