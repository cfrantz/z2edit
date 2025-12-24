use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::ffi::CString;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use nes::{Address, NesFile};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Miscellaneous {
    pub walk_anywhere: bool,
    pub item_pickup_delay: u8,
    pub text_delay: i8,
    pub beam_sword_time: u8,
    pub beam_sword_speed: u8,
    pub elevator_speed: i8,
    pub fairy_speed: i8,
    pub hack: IndexMap<String, String>,
}

#[typetag::serde]
impl GameData for Miscellaneous {
    fn name(&self) -> String {
        "Miscellaneous".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::misc_hacks::MiscellaneousEditor::new(self, name)
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

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HackDetail {
        pub name: String,
        #[serde(default)]
        pub code: String,
    }

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Hack {
        pub name: String,
        pub detail: IndexMap<String, HackDetail>,
    }

    #[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Miscellaneous {
        pub walk_anywhere: Address,
        pub item_pickup_delay: Address,
        pub text_delay: Vec<Address>,
        pub beam_sword_time: Address,
        pub beam_sword_speed: Address,
        pub elevator_speed: Address,
        pub fairy_speed: Address,
        pub hack: IndexMap<String, Hack>,
    }
}

impl config::Miscellaneous {
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("Miscellaneous::unpack {path}");
        let mut misc = Miscellaneous::default();
        let rom = rrom.borrow();
        misc.walk_anywhere = rom.read(self.walk_anywhere)? == 0;
        misc.item_pickup_delay = rom.read(self.item_pickup_delay)?;
        misc.text_delay = rom.read(self.text_delay[0])? as i8;
        misc.beam_sword_time = 255 - rom.read(self.beam_sword_time)?;
        misc.beam_sword_speed = rom.read(self.beam_sword_speed)?;
        misc.elevator_speed = rom.read(self.elevator_speed + 1)? as i8;
        misc.fairy_speed = rom.read(self.fairy_speed + 1)? as i8;

        for (name, hack) in self.hack.iter() {
            if let Some((which, _detail)) = hack.detail.get_index(0) {
                misc.hack.insert(name.clone(), which.clone());
            } else {
                log::error!("Miscellaneous::hack {name} has no details");
            }
        }
        edits.insert(path.into(), Edit::new(misc.into()));
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("Miscellaneous::pack {path}");
        if let Some(edit) = edits.get(path) {
            let misc = edit.data_ref::<Miscellaneous>()?;
            {
                let mut rom = rrom.borrow_mut();
                rom.write(self.walk_anywhere, if misc.walk_anywhere { 0 } else { 2 })?;
                rom.write(self.item_pickup_delay, misc.item_pickup_delay as u8)?;

                let delay0 = misc.text_delay;
                let delay1 = if delay0 - 0x1f < 0 { 0 } else { delay0 - 0x1f };
                let delay2 = if delay0 - 0x25 < 0 { 0 } else { delay0 - 0x25 };
                rom.write(self.text_delay[0], delay0 as u8)?;
                rom.write(self.text_delay[1], delay1 as u8)?;
                rom.write(self.text_delay[2], delay2 as u8)?;

                rom.write(self.beam_sword_time, (255 - misc.beam_sword_time) as u8)?;
                rom.write(self.beam_sword_speed, misc.beam_sword_speed as u8)?;
                rom.write(self.elevator_speed + 1, misc.elevator_speed as u8)?;
                rom.write(self.elevator_speed + 2, -misc.elevator_speed as u8)?;

                rom.write(self.fairy_speed + 1, misc.fairy_speed as u8)?;
                rom.write(self.fairy_speed + 2, -misc.fairy_speed as u8)?;
                rom.write(self.fairy_speed + 4 + 1, misc.fairy_speed as u8)?;
                rom.write(self.fairy_speed + 4 + 2, -misc.fairy_speed as u8)?;
                rom.write(self.fairy_speed + 8 + 1, misc.fairy_speed as u8)?;
                rom.write(self.fairy_speed + 8 + 2, -misc.fairy_speed as u8)?;
            }

            // FIXME: need to eval this in python with a ref to the project.
            for (name, which) in misc.hack.iter() {
                let hack = self
                    .hack
                    .get(name)
                    .ok_or(Error::NotFound(format!("Miscellaneous::hack[{name}]")))?;
                let detail = hack.detail.get(which).ok_or(Error::NotFound(format!(
                    "Miscellaneous::hack detail {which}"
                )))?;
                Python::attach(|py| -> Result<()> {
                    let locals = PyDict::new(py);
                    locals.set_item("rom", rrom)?;
                    let code = CString::new(detail.code.clone())?;
                    py.run(&code, None, Some(&locals))?;
                    Ok(())
                })?
            }
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }
    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("Miscellaneous/{path:?}")).into()),
        }
    }
}
