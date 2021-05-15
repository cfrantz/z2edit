use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::convert::From;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::enemyattr::EnemyGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    pub use crate::zelda2::items::config::Sprite;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct TownSpriteTable {
        pub mapping: Address,
        pub mapping2: Vec<Address>,
        pub palette: Address,
        pub table: Address,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EnemyGroup {
        pub id: IdPath,
        #[serde(default)]
        pub alias: IdPath,
        pub name: String,
        pub world: u8,
        pub overworld: u8,
        pub hp: Address,
        pub xp: Address,
        pub table_len: u8,
        #[serde(default)]
        pub town_table: Option<TownSpriteTable>,
        pub enemy: Vec<Sprite>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config(pub Vec<EnemyGroup>);

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<(&EnemyGroup, &Sprite)> {
            path.check_len("enemy", 3)?;
            for group in self.0.iter() {
                if path.prefix(&group.id) || path.prefix(&group.alias) {
                    for enemy in group.enemy.iter() {
                        if path.at(2) == enemy.id {
                            return Ok((group, enemy));
                        }
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn find_group(&self, path: &IdPath) -> Result<&EnemyGroup> {
            for group in self.0.iter() {
                if path.prefix(&group.id) || path.prefix(&group.alias) {
                    return Ok(group);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn find_by_index(&self, path: &IdPath, index: u8) -> Result<(&EnemyGroup, &Sprite)> {
            for group in self.0.iter() {
                if path.prefix(&group.id) || path.prefix(&group.alias) {
                    for enemy in group.enemy.iter() {
                        if index == enemy.offset {
                            return Ok((group, enemy));
                        }
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/enemies.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub id: IdPath,
    pub hp: i32,
    pub palette: usize,
    pub steal_xp: bool,
    pub need_fire: bool,
    pub xp: usize,
    pub drop_group: usize,
    pub no_beam: bool,
    pub unknown1: bool,
    pub damage: usize,
    pub no_thunder: bool,
    pub regenerate: bool,
    pub unknown2: bool,
    pub no_sword: bool,
    pub unknown3: usize,
}

impl Enemy {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if let Some(id) = id {
            Ok(Box::new(Enemy {
                id: IdPath::from(id),
                ..Default::default()
            }))
        } else {
            Err(ErrorKind::IdPathError("id required".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for Enemy {
    fn name(&self) -> String {
        "Enemy".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let (egrp, ecfg) = config.enemy.find(&self.id)?;
        let rom = edit.rom.borrow();
        let hp = rom.read(egrp.hp + ecfg.offset)?;
        let xp0 = rom.read(egrp.xp + ecfg.offset)?;
        let xp1 = rom.read(egrp.xp + ecfg.offset + egrp.table_len * 1)?;
        let xp3 = rom.read(egrp.xp + ecfg.offset + egrp.table_len * 3)?;

        self.hp = hp as i32;
        self.palette = (xp0 >> 6) as usize;
        self.need_fire = (xp0 & 0x20) != 0;
        self.steal_xp = (xp0 & 0x10) != 0;
        self.xp = (xp0 & 0x0F) as usize;

        self.drop_group = (xp1 >> 6) as usize;
        self.no_beam = (xp1 & 0x20) != 0;
        self.unknown1 = (xp1 & 0x10) != 0;
        self.damage = (xp1 & 0x0F) as usize;

        self.no_thunder = (xp3 & 0x80) != 0;
        self.regenerate = (xp3 & 0x40) != 0;
        self.no_sword = (xp3 & 0x20) != 0;
        self.unknown2 = (xp3 & 0x10) != 0;
        self.unknown3 = (xp3 & 0x0F) as usize;
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let (egrp, ecfg) = config.enemy.find(&self.id)?;

        let xp0 = (self.palette << 6) as u8
            | if self.need_fire { 0x20 } else { 0x00 }
            | if self.steal_xp { 0x10 } else { 0x00 }
            | self.xp as u8;

        let xp1 = (self.drop_group << 6) as u8
            | if self.no_beam { 0x20 } else { 0x00 }
            | if self.unknown1 { 0x10 } else { 0x00 }
            | self.damage as u8;

        let xp3 = 0 as u8
            | if self.no_thunder { 0x80 } else { 0x00 }
            | if self.regenerate { 0x40 } else { 0x00 }
            | if self.no_sword { 0x20 } else { 0x00 }
            | if self.unknown2 { 0x10 } else { 0x00 }
            | self.unknown3 as u8;

        let mut rom = edit.rom.borrow_mut();
        rom.write(egrp.hp + ecfg.offset, self.hp as u8)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 0, xp0)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 1, xp1)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 3, xp3)?;
        Ok(())
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyGroup {
    pub data: Vec<Enemy>,
}

impl EnemyGroup {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(EnemyGroup::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for EnemyGroup {
    fn name(&self) -> String {
        "Enemy Attributes".to_owned()
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
        EnemyGui::new(project, Some(project.get_commit(commit_index)?))
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
