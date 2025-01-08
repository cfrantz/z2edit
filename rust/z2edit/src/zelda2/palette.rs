use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaletteGroup {
    pub group: IndexMap<String, Vec<u8>>,
}

#[typetag::serde]
impl GameData for PaletteGroup {
    fn name(&self) -> String {
        "Palette".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::palette::PaletteGroupEditor::new(self, name)
    }
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Palette {
        pub name: String,
        pub address: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub magic_background: Option<Address>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct PaletteGroup {
        pub name: String,
        #[serde(flatten)]
        pub group: IndexMap<String, Palette>,
    }
}

/***
impl config::Palette {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        let length = self.length.unwrap_or(16);
        let data = rom.read_bytes(self.address, length)?.to_vec();
        edits.insert(path.into(), Edit::new(Palette { data }.into()));
        Ok(())
    }

    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        if let Some(edit) = edits.get(path) {
            let palette = edit.data_ref::<Palette>()?;
            rom.write_bytes(self.address, &palette.data)?;
            if let Some(bg) = self.magic_background {
                rom.write(bg, palette.data[0])?;
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
}
***/

impl config::PaletteGroup {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        let mut pg = PaletteGroup::default();
        for (name, palette) in self.group.iter() {
            let length = palette.length.unwrap_or(16);
            let data = rom.read_bytes(palette.address, length)?.to_vec();
            pg.group.insert(name.into(), data);
        }
        edits.insert(path.into(), Edit::new(pg.into()));
        Ok(())
    }

    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        if let Some(edit) = edits.get(path) {
            let pg = edit.data_ref::<PaletteGroup>()?;
            for (name, palette) in self.group.iter() {
                if let Some(data) = pg.group.get(name) {
                    rom.write_bytes(palette.address, &data)?;
                    if let Some(bgaddr) = palette.magic_background {
                        rom.write(bgaddr, data[0])?;
                    }
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
            [n] => {
                let palette = self
                    .group
                    .get(*n)
                    .ok_or(Error::NotFound(format!("PaletteGroup/{n}")))?;
                get_config::<T>(palette)
            }
            _ => Err(Error::NotFound(format!("PaletteGroup/{path:?}")).into()),
        }
    }
}
