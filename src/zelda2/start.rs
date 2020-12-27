use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::start::StartGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub values: Address,
        pub lives: Address,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/misc_start.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Levels {
    pub attack: i32,
    pub magic: i32,
    pub life: i32,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Spells {
    pub shield: bool,
    pub jump: bool,
    pub life: bool,
    pub fairy: bool,
    pub fire: bool,
    pub reflex: bool,
    pub spell: bool,
    pub thunder: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub heart: i32,
    pub magic: i32,
    pub crystals: i32,
    pub lives: i32,
    pub candle: bool,
    pub glove: bool,
    pub raft: bool,
    pub boots: bool,
    pub flute: bool,
    pub cross: bool,
    pub hammer: bool,
    pub magickey: bool,
    pub downstab: bool,
    pub upstab: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Start {
    pub level: Levels,
    pub spell: Spells,
    pub inventory: Inventory,
}

impl Start {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for Start {
    fn name(&self) -> String {
        "Start Values".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let start = config.misc.start.values;
        let rom = edit.rom.borrow();

        self.level.attack = rom.read(start + 0)? as i32;
        self.level.magic = rom.read(start + 1)? as i32;
        self.level.life = rom.read(start + 2)? as i32;

        self.spell.shield = rom.read(start + 4)? != 0;
        self.spell.jump = rom.read(start + 5)? != 0;
        self.spell.life = rom.read(start + 6)? != 0;
        self.spell.fairy = rom.read(start + 7)? != 0;
        self.spell.fire = rom.read(start + 8)? != 0;
        self.spell.reflex = rom.read(start + 9)? != 0;
        self.spell.spell = rom.read(start + 10)? != 0;
        self.spell.thunder = rom.read(start + 11)? != 0;

        self.inventory.magic = rom.read(start + 12)? as i32;
        self.inventory.heart = rom.read(start + 13)? as i32;
        self.inventory.candle = rom.read(start + 14)? != 0;
        self.inventory.glove = rom.read(start + 15)? != 0;
        self.inventory.raft = rom.read(start + 16)? != 0;
        self.inventory.boots = rom.read(start + 17)? != 0;
        self.inventory.flute = rom.read(start + 18)? != 0;
        self.inventory.cross = rom.read(start + 19)? != 0;
        self.inventory.hammer = rom.read(start + 20)? != 0;
        self.inventory.magickey = rom.read(start + 21)? != 0;
        self.inventory.crystals = rom.read(start + 29)? as i32;
        self.inventory.lives = rom.read(config.misc.start.lives)? as i32;
        let tech = rom.read(start + 31)?;
        self.inventory.downstab = (tech & 0x10) != 0;
        self.inventory.upstab = (tech & 0x04) != 0;
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let start = config.misc.start.values;
        let mut rom = edit.rom.borrow_mut();

        rom.write(start + 0, self.level.attack as u8)?;
        rom.write(start + 1, self.level.magic as u8)?;
        rom.write(start + 2, self.level.life as u8)?;

        rom.write(start + 4, self.spell.shield as u8)?;
        rom.write(start + 5, self.spell.jump as u8)?;
        rom.write(start + 6, self.spell.life as u8)?;
        rom.write(start + 7, self.spell.fairy as u8)?;
        rom.write(start + 8, self.spell.fire as u8)?;
        rom.write(start + 9, self.spell.reflex as u8)?;
        rom.write(start + 10, self.spell.spell as u8)?;
        rom.write(start + 11, self.spell.thunder as u8)?;

        rom.write(start + 12, self.inventory.magic as u8)?;
        rom.write(start + 13, self.inventory.heart as u8)?;
        rom.write(start + 14, self.inventory.candle as u8)?;
        rom.write(start + 15, self.inventory.glove as u8)?;
        rom.write(start + 16, self.inventory.raft as u8)?;
        rom.write(start + 17, self.inventory.boots as u8)?;
        rom.write(start + 18, self.inventory.flute as u8)?;
        rom.write(start + 19, self.inventory.cross as u8)?;
        rom.write(start + 20, self.inventory.hammer as u8)?;
        rom.write(start + 21, self.inventory.magickey as u8)?;
        rom.write(start + 29, self.inventory.crystals as u8)?;
        rom.write(
            start + 31,
            if self.inventory.downstab { 0x10 } else { 0 }
                | if self.inventory.upstab { 0x04 } else { 0 },
        )?;
        rom.write(config.misc.start.lives, self.inventory.lives as u8)?;
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        StartGui::new(project, commit_index)
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
