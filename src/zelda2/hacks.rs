use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::hacks::HacksGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::python::PythonScript;

pub mod config {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HackItem {
        pub id: String,
        pub name: String,
        pub code: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Hack {
        pub id: String,
        pub name: String,
        pub item: Vec<HackItem>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub walk_anywhere: Address,
        pub item_pickup_delay: Address,
        pub text_delay: Vec<Address>,
        pub beam_sword_time: Address,
        pub beam_sword_speed: Address,
        pub elevator_speed: Address,
        pub fairy_speed: Address,
        pub hack: Vec<Hack>,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/misc_hacks.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hack {
    pub id: String,
    pub selected: usize,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hacks {
    pub walk_anywhere: bool,
    pub item_pickup_delay: i32,
    pub text_delay: i32,
    pub beam_sword_time: i32,
    pub beam_sword_speed: i32,
    pub elevator_speed: i32,
    pub fairy_speed: i32,
    pub item: Vec<Hack>,
}

impl Hacks {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }

    pub fn find_item(&self, id: &str) -> Option<&Hack> {
        for item in self.item.iter() {
            if item.id == id {
                return Some(item);
            }
        }
        None
    }
}

#[typetag::serde]
impl RomData for Hacks {
    fn name(&self) -> String {
        "Miscellaneous Hacks".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let cfg = &config.misc.hacks;
        let rom = edit.rom.borrow();

        self.walk_anywhere = rom.read(cfg.walk_anywhere)? == 0;
        self.item_pickup_delay = rom.read(cfg.item_pickup_delay)? as i32;
        self.text_delay = rom.read(cfg.text_delay[0])? as i32;
        self.beam_sword_time = 256 - rom.read(cfg.beam_sword_time)? as i32;
        self.beam_sword_speed = rom.read(cfg.beam_sword_speed)? as i32;
        self.elevator_speed = rom.read(cfg.elevator_speed + 1)? as i32;
        self.fairy_speed = rom.read(cfg.fairy_speed + 1)? as i32;
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let cfg = &config.misc.hacks;

        {
            let mut rom = edit.rom.borrow_mut();
            rom.write(cfg.walk_anywhere, if self.walk_anywhere { 0 } else { 2 })?;
            rom.write(cfg.item_pickup_delay, self.item_pickup_delay as u8)?;

            let delay0 = self.text_delay;
            let delay1 = if delay0 - 0x1f < 0 { 0 } else { delay0 - 0x1f };
            let delay2 = if delay0 - 0x25 < 0 { 0 } else { delay0 - 0x25 };
            rom.write(cfg.text_delay[0], delay0 as u8)?;
            rom.write(cfg.text_delay[1], delay1 as u8)?;
            rom.write(cfg.text_delay[2], delay2 as u8)?;

            rom.write(cfg.beam_sword_time, (256 - self.beam_sword_time) as u8)?;
            rom.write(cfg.beam_sword_speed, self.beam_sword_speed as u8)?;
            rom.write(cfg.elevator_speed + 1, self.elevator_speed as u8)?;
            rom.write(cfg.elevator_speed + 2, -self.elevator_speed as u8)?;

            rom.write(cfg.fairy_speed + 1, self.fairy_speed as u8)?;
            rom.write(cfg.fairy_speed + 2, -self.fairy_speed as u8)?;
            rom.write(cfg.fairy_speed + 4 + 1, self.fairy_speed as u8)?;
            rom.write(cfg.fairy_speed + 4 + 2, -self.fairy_speed as u8)?;
            rom.write(cfg.fairy_speed + 8 + 1, self.fairy_speed as u8)?;
            rom.write(cfg.fairy_speed + 8 + 2, -self.fairy_speed as u8)?;
        }

        for item in self.item.iter() {
            for h in cfg.hack.iter() {
                if item.id == h.id {
                    if let Some(hackitem) = &h.item.get(item.selected) {
                        info!("Applying hackitem '{}'", hackitem.name);
                        let p = PythonScript {
                            file: None,
                            code: hackitem.code.clone(),
                        };
                        p.pack(edit)?;
                    } else {
                        error!("No hackitem[{}] in '{}'", item.selected, h.name);
                    }
                }
            }
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        HacksGui::new(project, commit_index)
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
