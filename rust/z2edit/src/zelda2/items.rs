use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use nes::{Address, NesFile};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    #[serde(default)]
    pub offset: u8,
    pub chr: Address,
    pub palette: u8,
    pub size: [u32; 2],
    #[serde(default)]
    pub sprites: Vec<i32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub bit: i8,
    pub slot: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Items {
    pub item: IndexMap<u8, Sprite>,
    pub effect: IndexMap<String, Effect>,
}

#[typetag::serde]
impl GameData for Items {
    fn name(&self) -> String {
        "Items".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::items::ItemsEditor::new(self, name)
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
    pub struct Effect {
        pub load: Address,
        pub save: Address,
        pub bits: Address,
        pub count: Option<Address>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Effects {
        pub town_base: i16,
        pub magic_base: i16,
        pub effect: IndexMap<String, Effect>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Items {
        pub sprite_table: Address,
        pub item: IndexMap<String, Sprite>,
        pub fake: IndexMap<String, Sprite>,
        pub effects: Effects,
    }
}

impl config::Items {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Items::unpack {path}");
        let rom = rrom.borrow();
        let mut items = Items::default();
        for it in self.item.values() {
            let mut it = it.clone();
            let a = rom.read(self.sprite_table + it.offset * 2)? as i32;
            let mut b = rom.read(self.sprite_table + it.offset * 2 + 1)? as i32;
            if a == b {
                b |= 0x0100_0000;
            }
            it.sprites.push(a);
            it.sprites.push(b);
            items.item.insert(it.offset, it);
        }

        let magic_offset = 8 + self.effects.town_base - self.effects.magic_base;
        for (item, effect) in self.effects.effect.iter() {
            let mut slot = rom.read_word(effect.load)? as i16 - self.effects.town_base;
            if slot < 0 {
                slot += magic_offset;
            }
            let slot = slot as i8;
            let bit = rom
                .read(effect.bits)?
                .checked_ilog2()
                .map(|v| v as i8)
                .unwrap_or(-1);
            let count = effect.count.map(|c| rom.read(c)).transpose()?;
            items
                .effect
                .insert(item.clone(), Effect { slot, bit, count });
        }
        edits.insert(path.into(), Edit::new(items.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Items::pack {path}");
        if let Some(edit) = edits.get(path) {
            let items = edit.data_ref::<Items>()?;
            let mut rom = rrom.borrow_mut();
            let magic_offset = 8 + self.effects.town_base - self.effects.magic_base;
            for (item, cfg) in self.effects.effect.iter() {
                if let Some(effect) = items.effect.get(item) {
                    let slot = effect.slot as i16;
                    let addr = if slot < 8 {
                        slot + self.effects.town_base
                    } else {
                        slot + self.effects.town_base - magic_offset
                    };
                    rom.write_word(cfg.load, addr as u16)?;
                    rom.write_word(cfg.save, addr as u16)?;

                    let byte = if effect.bit < 0 {
                        0u8
                    } else {
                        1 << (effect.bit as u8)
                    };
                    rom.write(cfg.bits, byte)?;

                    match (cfg.count, effect.count) {
                        (None, None) => {}
                        (Some(addr), Some(count)) => {
                            rom.write(addr, count)?;
                        }
                        _ => {
                            log::error!(
                                "Mismatched count option: {:?} {:?}",
                                cfg.count,
                                effect.count
                            );
                        }
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
            ["fake", n] => {
                let sprite = self
                    .fake
                    .get(*n)
                    .ok_or(Error::NotFound(format!("Items/fake/{n}")))?;
                get_config::<T>(sprite)
            }
            [n] => {
                let sprite = self
                    .item
                    .get(*n)
                    .ok_or(Error::NotFound(format!("Items/{n}")))?;
                get_config::<T>(sprite)
            }
            _ => Err(Error::NotFound(format!("Items/{path:?}")).into()),
        }
    }
}
