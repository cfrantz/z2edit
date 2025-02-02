use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enemy {
    pub hp: u8,
    pub palette: usize,
    pub steal_xp: bool,
    pub need_fire: bool,
    pub xp: usize,
    pub drop_group: usize,
    pub no_beam: bool,
    pub no_spell: bool,
    pub damage: usize,
    pub no_thunder: bool,
    pub regenerate: bool,
    pub unknown2: bool,
    pub no_sword: bool,
    pub unknown3: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnemyGroup {
    pub group: IndexMap<String, Enemy>,
}

#[typetag::serde]
impl GameData for EnemyGroup {
    fn name(&self) -> String {
        "EnemyGroup".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::enemies::EnemyGroupEditor::new(self, name)
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
    use crate::zelda2::items::config::Sprite;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EnemyGroup {
        pub name: String,
        pub address: Address,
        pub length: usize,
        pub hp: Address,
        pub xp: Address,
        pub table_len: usize,
        pub group: IndexMap<String, Sprite>,
    }
}

impl config::EnemyGroup {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("EnemyGroup::unpack {path}");
        let mut eg = EnemyGroup::default();
        for index in self.group.keys() {
            let i = index.parse::<usize>()?;
            let hp = rom.read(self.hp + i)?;
            let xp0 = rom.read(self.xp + i)?;
            let xp1 = rom.read(self.xp + i + self.table_len * 1)?;
            let xp3 = rom.read(self.xp + i + self.table_len * 3)?;
            eg.group.insert(
                index.into(),
                Enemy {
                    hp,
                    palette: (xp0 >> 6) as usize,
                    need_fire: (xp0 & 0x20) != 0,
                    steal_xp: (xp0 & 0x10) != 0,
                    xp: (xp0 & 0x0F) as usize,
                    drop_group: (xp1 >> 6) as usize,
                    no_beam: (xp1 & 0x20) != 0,
                    no_spell: (xp1 & 0x10) != 0,
                    damage: (xp1 & 0x0F) as usize,
                    no_thunder: (xp3 & 0x80) != 0,
                    regenerate: (xp3 & 0x40) != 0,
                    no_sword: (xp3 & 0x20) != 0,
                    unknown2: (xp3 & 0x10) != 0,
                    unknown3: (xp3 & 0x0F) as usize,
                },
            );
        }
        edits.insert(path.into(), Edit::new(eg.into()));
        Ok(())
    }

    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("EnemyGroup::pack {path}");
        if let Some(edit) = edits.get(path) {
            let eg = edit.data_ref::<EnemyGroup>()?;
            for index in self.group.keys() {
                if let Some(enemy) = eg.group.get(index) {
                    let i = index.parse::<usize>()?;

                    let xp0 = (enemy.palette << 6) as u8
                        | if enemy.need_fire { 0x20 } else { 0x00 }
                        | if enemy.steal_xp { 0x10 } else { 0x00 }
                        | enemy.xp as u8;

                    let xp1 = (enemy.drop_group << 6) as u8
                        | if enemy.no_beam { 0x20 } else { 0x00 }
                        | if enemy.no_spell { 0x10 } else { 0x00 }
                        | enemy.damage as u8;

                    let xp3 = 0 as u8
                        | if enemy.no_thunder { 0x80 } else { 0x00 }
                        | if enemy.regenerate { 0x40 } else { 0x00 }
                        | if enemy.no_sword { 0x20 } else { 0x00 }
                        | if enemy.unknown2 { 0x10 } else { 0x00 }
                        | enemy.unknown3 as u8;

                    rom.write(self.hp + i, enemy.hp)?;
                    rom.write(self.xp + i + self.table_len * 0, xp0)?;
                    rom.write(self.xp + i + self.table_len * 1, xp1)?;
                    rom.write(self.xp + i + self.table_len * 3, xp3)?;
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
            _ => Err(Error::NotFound(format!("EnemyGroup/{path:?}")).into()),
        }
    }
}
