use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::xp_spells::ExperienceTableGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_encoding::Text;

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ExperienceTable {
        pub id: String,
        pub name: String,
        pub address: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub game_name: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub graphics: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<isize>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ExperienceTableGroup {
        pub id: String,
        pub name: String,
        pub table: Vec<ExperienceTable>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub group: Vec<config::ExperienceTableGroup>,
        pub enemy_xp_lo: Address,
        pub enemy_xp_hi: Address,
        pub enemy_xp_gfx: Address,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::ExperienceTable> {
            path.check_len("xp", 2)?;
            for group in self.group.iter() {
                if path.at(0) == group.id {
                    for table in group.table.iter() {
                        if path.at(1) == table.id {
                            return Ok(table);
                        }
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/experience.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExperienceTable {
    pub id: IdPath,
    pub data: Vec<i32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub game_name: String,
}

#[typetag::serde]
impl RomData for ExperienceTable {
    fn name(&self) -> String {
        "ExperienceTable".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let rom = edit.rom.borrow();
        let config = Config::get(&edit.meta.borrow().config)?;
        let tcfg = config.experience.find(&self.id)?;
        let offset = tcfg.offset.unwrap_or(0);
        self.data.clear();
        for i in 0..8 {
            let mut val: i32 = rom.read(tcfg.address + i)? as i32;
            if offset != 0 {
                val |= (rom.read(tcfg.address + offset + i)? as i32) << 8;
            }
            self.data.push(val);
        }
        if let Some(game_name) = tcfg.game_name {
            self.game_name = Text::from_zelda2(&rom.read_bytes(game_name, 8)?.to_vec());
        } else {
            self.game_name.clear();
        }
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let mut rom = edit.rom.borrow_mut();
        let config = Config::get(&edit.meta.borrow().config)?;
        let tcfg = config.experience.find(&self.id)?;
        let offset = tcfg.offset.unwrap_or(0);
        for (i, val) in self.data.iter().enumerate() {
            rom.write(tcfg.address + i, *val as u8)?;
            if offset != 0 {
                rom.write(tcfg.address + i, (*val >> 8) as u8)?;
            }
        }
        if let Some(game_name) = tcfg.game_name {
            // Handle spell names.
            let len = if self.game_name.len() < 8 {
                self.game_name.len()
            } else {
                8
            };
            rom.write_bytes(game_name, &Text::to_zelda2(&self.game_name[0..len]))?;
        }
        if let Some(graphics) = tcfg.graphics {
            // Handle level-up points values graphics.
            // Arranged in the ROM as three 24-byte tables representing the
            // tens, hundreds and thousands places of life, magic and attack
            // level-up digit graphics:
            //
            //   10s: AAAAAAAAMMMMMMMMLLLLLLLL
            //  100s: AAAAAAAAMMMMMMMMLLLLLLLL
            // 1000s: AAAAAAAAMMMMMMMMLLLLLLLL
            let factors = [10, 100, 1000];
            for (i, val) in self.data.iter().enumerate() {
                for n in 0..3 {
                    let factor = factors[n];
                    let digit = if *val < factor {
                        0xf4
                    } else {
                        ((*val / factor) % 10) as u8 + 0xd0
                    };
                    rom.write(graphics + n * 24 + i, digit)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExperienceTableGroup {
    pub data: Vec<ExperienceTable>,
}

#[typetag::serde]
impl RomData for ExperienceTableGroup {
    fn name(&self) -> String {
        "Experience & Spells".to_owned()
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
        ExperienceTableGui::new(project, commit_index)
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
