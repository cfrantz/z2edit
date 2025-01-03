use std::path::Path;

use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::nes::NesFile;
use crate::zelda2::palette::{self, PaletteGroup};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameBank {
    pub palette: IndexMap<String, PaletteGroup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GlobalBank {
    pub palette: IndexMap<String, PaletteGroup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Game {
    pub bank: IndexMap<String, GameBank>,
    pub global: GlobalBank,
}

pub mod config {
    use super::*;
    use palette::config::PaletteGroup;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GameBank {
        pub palette: IndexMap<String, PaletteGroup>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct GlobalBank {
        pub palette: IndexMap<String, PaletteGroup>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Game {
        pub name: String,
        pub include: Vec<String>,
        pub bank: IndexMap<String, GameBank>,
        pub global: GlobalBank,
    }

    impl Game {
        pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
            let mut path = path.as_ref().to_owned();
            let data = std::fs::read_to_string(&path)?;
            let mut game = serde_annotate::from_str::<Game>(&data)?;
            for i in game.include.iter() {
                path.set_file_name(i);
                let data = std::fs::read_to_string(&path)?;
                let data = serde_annotate::from_str::<Game>(&data)?;
                game.bank.extend(data.bank);
            }
            Ok(game)
        }
    }
}

impl config::GameBank {
    pub fn unpack(&self, rom: &NesFile) -> Result<GameBank> {
        let mut palette = IndexMap::default();
        for (k, v) in self.palette.iter() {
            palette.insert(k.into(), v.unpack(rom)?);
        }
        Ok(GameBank { palette })
    }

    pub fn pack(&self, rom: &mut NesFile, data: &GameBank) -> Result<()> {
        for (k, v) in self.palette.iter() {
            if let Some(palette) = data.palette.get(k) {
                v.pack(rom, palette)?;
            }
        }
        Ok(())
    }
}

impl config::GlobalBank {
    pub fn unpack(&self, rom: &NesFile) -> Result<GlobalBank> {
        let mut palette = IndexMap::default();
        for (k, v) in self.palette.iter() {
            palette.insert(k.into(), v.unpack(rom)?);
        }
        Ok(GlobalBank { palette })
    }

    pub fn pack(&self, rom: &mut NesFile, data: &GlobalBank) -> Result<()> {
        for (k, v) in self.palette.iter() {
            if let Some(palette) = data.palette.get(k) {
                v.pack(rom, palette)?;
            }
        }
        Ok(())
    }
}

impl config::Game {
    pub fn unpack(&self, rom: &NesFile) -> Result<Game> {
        let mut bank = IndexMap::default();
        for (k, v) in self.bank.iter() {
            bank.insert(k.into(), v.unpack(rom)?);
        }
        let global = self.global.unpack(rom)?;
        Ok(Game { bank, global })
    }

    pub fn pack(&self, rom: &mut NesFile, data: &Game) -> Result<()> {
        for (k, v) in self.bank.iter() {
            if let Some(bank) = data.bank.get(k) {
                v.pack(rom, bank)?;
            }
        }
        self.global.pack(rom, &data.global)?;
        Ok(())
    }
}
