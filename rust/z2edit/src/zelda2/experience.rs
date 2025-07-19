use anyhow::Result;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use crate::zelda2::text_encoding::Text;
use nes::{Address, NesFile};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceTable {
    pub data: Vec<u16>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub game_text: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceTableGroup {
    pub group: Vec<ExperienceTable>,
}

#[typetag::serde]
impl GameData for ExperienceTableGroup {
    fn name(&self) -> String {
        "ExperienceTableGroup".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::experience::ExperienceTableGroupEditor::new(self, name)
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceValue {
    pub value: u16,
    pub sprites: [u8; 2],
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnemyExperience {
    pub data: Vec<ExperienceValue>,
}

#[typetag::serde]
impl GameData for EnemyExperience {
    fn name(&self) -> String {
        "EnemyExperience".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::experience::EnemyExperienceEditor::new(self, name)
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ExperienceTable {
        pub name: String,
        pub address: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub game_text: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub graphics: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<isize>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub header: Vec<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ExperienceTableGroup {
        pub name: String,
        pub group: Vec<ExperienceTable>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EnemyExperience {
        pub lo: Address,
        pub hi: Address,
        pub graphics: Address,
    }
}

impl config::ExperienceTableGroup {
    // Zelda2 uses 0xF4 as a blank space.
    const CHR_BLANK: u8 = 0xF4;
    // Zelda2 uses 0xD0 - 0xD9 as the digits 0-9.
    const CHR_DIGITS: u8 = 0xD0;

    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("ExperienceTableGroup::unpack {path}");
        let mut eg = ExperienceTableGroup::default();
        let rom = rrom.borrow();
        for et in self.group.iter() {
            let mut table = ExperienceTable::default();
            for i in 0..8 {
                let mut val = rom.read(et.address + i)? as u16;
                if let Some(offset) = et.offset {
                    val |= (rom.read(et.address + offset + i)? as u16) << 8;
                }
                table.data.push(val);
            }
            if let Some(game_text) = et.game_text {
                table.game_text = Text::from_zelda2(rom.read_bytes(game_text, 8)?);
            }
            eg.group.push(table);
        }
        edits.insert(path.into(), Edit::new(eg.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("ExperienceTableGroup::pack {path}");
        if let Some(edit) = edits.get(path) {
            let eg = edit.data_ref::<ExperienceTableGroup>()?;
            let mut rom = rrom.borrow_mut();
            for (et, table) in self.group.iter().zip(eg.group.iter()) {
                for (i, val) in table.data.iter().enumerate() {
                    rom.write(et.address + i, *val as u8)?;
                    if let Some(offset) = et.offset {
                        rom.write(et.address + offset + i, (*val >> 8) as u8)?;
                    }
                    if let Some(graphics) = et.graphics {
                        // This handles writing the level-up points values as
                        // graphics.  The game has in ROM three 24-byte tables
                        // representing the tens, hundreds and thousands places
                        // of life, magic and attack level-up digits:
                        //
                        //   10s: AAAAAAAAMMMMMMMMLLLLLLLL
                        //  100s: AAAAAAAAMMMMMMMMLLLLLLLL
                        // 1000s: AAAAAAAAMMMMMMMMLLLLLLLL
                        let mut factor = 1;
                        for n in 0..3 {
                            factor *= 10;
                            let digit = if *val < factor {
                                Self::CHR_BLANK
                            } else {
                                ((*val / factor) % 10) as u8 + Self::CHR_DIGITS
                            };
                            rom.write(graphics + n * 24 + i, digit)?;
                        }
                    }
                }
                if let Some(game_text) = et.game_text {
                    let mut text = table.game_text.clone();
                    while text.len() < 8 {
                        text.push('.');
                    }
                    rom.write_bytes(game_text, &Text::to_zelda2(&text[0..8]))?;
                }
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("ExperienceTableGroup/{path:?}")).into()),
        }
    }
}

impl config::EnemyExperience {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("EnemyExperience::unpack {path}");
        let mut ee = EnemyExperience::default();
        let rom = rrom.borrow();
        for i in 0..16 {
            ee.data.push(ExperienceValue {
                value: rom.read(self.lo + i)? as u16 | (rom.read(self.hi + i)? as u16) << 8,
                sprites: [
                    rom.read(self.graphics + i)?,
                    rom.read(self.graphics + i + 16)?,
                ],
            });
        }
        edits.insert(path.into(), Edit::new(ee.into()));
        Ok(())
    }
    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("EnemyExperience::pack {path}");
        if let Some(edit) = edits.get(path) {
            let ee = edit.data_ref::<EnemyExperience>()?;
            let mut rom = rrom.borrow_mut();
            for (i, ev) in ee.data.iter().enumerate() {
                rom.write(self.lo + i, ev.value as u8)?;
                rom.write(self.hi + i, (ev.value >> 8) as u8)?;
                rom.write(self.graphics + i, ev.sprites[0])?;
                rom.write(self.graphics + i + 16, ev.sprites[1])?;
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("EnemyExperience/{path:?}")).into()),
        }
    }
}
