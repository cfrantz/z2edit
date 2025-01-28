use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Levels {
    pub attack: u8,
    pub magic: u8,
    pub life: u8,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Spells {
    pub shield: bool,
    pub jump: bool,
    pub life: bool,
    pub fairy: bool,
    pub fire: bool,
    pub reflect: bool,
    pub spell: bool,
    pub thunder: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub heart: u8,
    pub magic: u8,
    pub crystals: u8,
    pub lives: u8,
    pub candle: bool,
    pub glove: bool,
    pub raft: bool,
    pub boots: bool,
    pub flute: bool,
    pub cross: bool,
    pub hammer: bool,
    pub magic_key: bool,
    pub downstab: bool,
    pub upstab: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct StartValues {
    pub level: Levels,
    pub spell: Spells,
    pub inventory: Inventory,
}

#[typetag::serde]
impl GameData for StartValues {
    fn name(&self) -> String {
        "StartValues".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::start::StartValuesEditor::new(self, name)
    }
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct StartValues {
        pub values: Address,
        pub lives: Address,
    }
}

impl config::StartValues {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("StartValues::unpack {path}");
        let tech = rom.read(self.values + 31)?;
        let sv = StartValues {
            level: Levels {
                attack: rom.read(self.values + 0)?,
                magic: rom.read(self.values + 1)?,
                life: rom.read(self.values + 2)?,
            },
            spell: Spells {
                shield: rom.read(self.values + 4)? != 0,
                jump: rom.read(self.values + 5)? != 0,
                life: rom.read(self.values + 6)? != 0,
                fairy: rom.read(self.values + 7)? != 0,
                fire: rom.read(self.values + 8)? != 0,
                reflect: rom.read(self.values + 9)? != 0,
                spell: rom.read(self.values + 10)? != 0,
                thunder: rom.read(self.values + 11)? != 0,
            },
            inventory: Inventory {
                magic: rom.read(self.values + 12)?,
                heart: rom.read(self.values + 13)?,
                candle: rom.read(self.values + 14)? != 0,
                glove: rom.read(self.values + 15)? != 0,
                raft: rom.read(self.values + 16)? != 0,
                boots: rom.read(self.values + 17)? != 0,
                flute: rom.read(self.values + 18)? != 0,
                cross: rom.read(self.values + 19)? != 0,
                hammer: rom.read(self.values + 20)? != 0,
                magic_key: rom.read(self.values + 21)? != 0,
                crystals: rom.read(self.values + 29)?,
                lives: rom.read(self.lives)?,
                downstab: (tech & 0x10) != 0,
                upstab: (tech & 0x04) != 0,
            },
        };
        edits.insert(path.into(), Edit::new(sv.into()));
        Ok(())
    }

    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("StartValues::pack {path}");
        if let Some(edit) = edits.get(path) {
            let sv = edit.data_ref::<StartValues>()?;
            rom.write(self.values + 0, sv.level.attack)?;
            rom.write(self.values + 1, sv.level.magic)?;
            rom.write(self.values + 2, sv.level.life)?;

            rom.write(self.values + 4, sv.spell.shield as u8)?;
            rom.write(self.values + 5, sv.spell.jump as u8)?;
            rom.write(self.values + 6, sv.spell.life as u8)?;
            rom.write(self.values + 7, sv.spell.fairy as u8)?;
            rom.write(self.values + 8, sv.spell.fire as u8)?;
            rom.write(self.values + 9, sv.spell.reflect as u8)?;
            rom.write(self.values + 10, sv.spell.spell as u8)?;
            rom.write(self.values + 11, sv.spell.thunder as u8)?;

            rom.write(self.values + 12, sv.inventory.magic)?;
            rom.write(self.values + 13, sv.inventory.heart)?;
            rom.write(self.values + 14, sv.inventory.candle as u8)?;
            rom.write(self.values + 15, sv.inventory.glove as u8)?;
            rom.write(self.values + 16, sv.inventory.raft as u8)?;
            rom.write(self.values + 17, sv.inventory.boots as u8)?;
            rom.write(self.values + 18, sv.inventory.flute as u8)?;
            rom.write(self.values + 19, sv.inventory.cross as u8)?;
            rom.write(self.values + 20, sv.inventory.hammer as u8)?;
            rom.write(self.values + 21, sv.inventory.magic_key as u8)?;
            rom.write(self.values + 29, sv.inventory.crystals)?;
            rom.write(
                self.values + 31,
                if sv.inventory.downstab { 0x10 } else { 0 }
                    | if sv.inventory.upstab { 0x04 } else { 0 },
            )?;
            rom.write(self.lives, sv.inventory.lives)?;
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("PaletteGroup/{path:?}")).into()),
        }
    }
}
