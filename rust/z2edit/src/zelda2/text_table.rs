use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::gui::Gui;
use crate::nes::{Address, AddressRange, Alloc, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use crate::zelda2::text_encoding::Text;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextTable {
    pub data: IndexMap<u8, String>,
    pub index: Vec<Vec<u8>>,
    pub conditions: Vec<u8>,
}

impl TextTable {
    pub fn get_text_ids(&self, enemy: u8, town_code: u8) -> (Option<u8>, Option<u8>, Option<u8>) {
        if enemy < 10 {
            return (None, None, None);
        }
        let town_code = town_code as usize;
        let person = (enemy - 10) as usize;
        let dialog = self.index[0].get(person * 4 + town_code).copied();
        let dialog2 = self.index[1].get(person * 4 + town_code).copied();
        let condition = if person >= 9 && person < 13 {
            self.conditions.get((person - 9) * 4 + town_code).copied()
        } else {
            None
        };
        (dialog, dialog2, condition)
    }
}

#[typetag::serde]
impl GameData for TextTable {
    fn name(&self) -> String {
        "TextTable".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::text_table::TextTableEditor::new(self, name)
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

    #[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub struct TextTable {
        pub name: String,
        pub pointer: Address,
        pub offset: u8,
        pub length: u8,
        pub conditions: AddressRange,
        pub index: Vec<AddressRange>,
    }
}

impl config::TextTable {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("TextTable::unpack {path}");
        let mut tt = TextTable::default();
        let rom = rrom.borrow();
        let table = rom.read_pointer(self.pointer)?;
        for i in 0..self.length {
            let text = rom.read_pointer(table + i * 2)?;
            tt.data
                .insert(i, Text::from_zelda2(rom.read_terminated(text, 0xff)?));
        }
        for person in 0..4 {
            for town in 0..4 {
                tt.conditions
                    .push(rom.read(self.conditions.address + person * 8 + self.offset * 4 + town)?);
            }
        }
        for index in self.index.iter() {
            tt.index
                .push(rom.read_bytes(index.address, index.length as usize)?.into());
        }

        edits.insert(path.into(), Edit::new(tt.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("TextTable::pack {path}");
        if let Some(edit) = edits.get(path) {
            let tt = edit.data_ref::<TextTable>()?;
            let mut rom = rrom.borrow_mut();

            let table = rom.read_pointer(self.pointer)?;
            // Free the entire block of text
            for i in 0..self.length {
                let tptr = rom.read_pointer(table + i * 2)?;
                let text_len = rom.read_terminated(tptr, 0xff)?.len() as u16;
                rom.free(tptr, text_len + 1)?;
            }

            // Re-allocate and write the strings into the ROM.
            for (i, text) in tt.data.iter() {
                let text = Text::to_zelda2(text.as_str());
                let tptr = rom.read_pointer(table + i * 2)?;
                let tptr = rom.alloc(tptr, text.len() as u16 + 1, Alloc::Near)?;
                rom.write_pointer(table + i * 2, tptr)?;
                rom.write_terminated(tptr, &text, 0xff)?;
            }
            for (i, &byte) in tt.conditions.iter().enumerate() {
                let person = i / 4;
                let town = i % 4;
                rom.write(
                    self.conditions.address + person * 8 + self.offset * 4 + town,
                    byte,
                )?;
            }
            for (index, data) in self.index.iter().zip(tt.index.iter()) {
                rom.write_bytes(index.address, data)?;
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [..] => get_config::<T>(self),
        }
    }
}
