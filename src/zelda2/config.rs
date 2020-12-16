use once_cell::sync::Lazy;
use ron;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::errors::*;
use crate::nes::freespace;
use crate::nes::{Address, Layout, Segment};
use crate::zelda2::enemyattr;
use crate::zelda2::hacks;
use crate::zelda2::objects;
use crate::zelda2::overworld;
use crate::zelda2::palette;
use crate::zelda2::sideview;
use crate::zelda2::start;
use crate::zelda2::text_table;
use crate::zelda2::xp_spells;

impl freespace::config::Config {
    pub fn vanilla() -> Self {
        ron::de::from_bytes(include_bytes!("../../config/vanilla/freespace.ron")).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miscellaneous {
    pub start: start::config::Config,
    pub freespace: freespace::config::Config,
    pub hacks: hacks::config::Config,
    pub world_to_bank: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub layout: Layout,
    pub misc: Miscellaneous,
    pub palette: palette::config::Config,
    pub enemy: enemyattr::config::Config,
    pub experience: xp_spells::config::Config,
    pub objects: objects::config::Config,
    pub overworld: overworld::config::Config,
    pub sideview: sideview::config::Config,
    pub text_table: text_table::config::Config,
}

fn zelda2_nesfile_layout() -> Layout {
    Layout(vec![
        Segment::Raw {
            name: "header".to_owned(),
            offset: 0,
            length: 16,
            fill: 0xff,
        },
        Segment::Banked {
            name: "prg".to_owned(),
            offset: 16,
            length: 131072,
            banksize: 16384,
            mask: 0x3fff,
            fill: 0xff,
        },
        Segment::Banked {
            name: "chr".to_owned(),
            offset: 16 + 131072,
            length: 131072,
            banksize: 4096,
            mask: 0xfff,
            fill: 0xff,
        },
    ])
}

impl Config {
    pub fn vanilla() -> Self {
        Config {
            layout: zelda2_nesfile_layout(),
            misc: Miscellaneous {
                start: start::config::Config::vanilla(),
                freespace: freespace::config::Config::vanilla(),
                hacks: hacks::config::Config::vanilla(),
                world_to_bank: Address::Prg(-1, 0xc4b7),
            },
            palette: palette::config::Config::vanilla(),
            enemy: enemyattr::config::Config::vanilla(),
            experience: xp_spells::config::Config::vanilla(),
            objects: objects::config::Config::vanilla(),
            overworld: overworld::config::Config::vanilla(),
            sideview: sideview::config::Config::vanilla(),
            text_table: text_table::config::Config::vanilla(),
        }
    }
}

static mut CONFIGS: Lazy<Mutex<HashMap<String, Arc<Config>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("vanilla".to_owned(), Arc::new(Config::vanilla()));
    Mutex::new(map)
});

impl Config {
    pub fn get(name: &str) -> Result<Arc<Config>> {
        let configs = unsafe { CONFIGS.lock().unwrap() };
        match configs.get(name) {
            Some(v) => Ok(Arc::clone(v)),
            _ => Err(ErrorKind::ConfigNotFound(name.to_owned()).into()),
        }
    }

    pub fn put(name: &str, config: Config) {
        let configs = unsafe { CONFIGS.get_mut().unwrap() };
        configs.insert(name.to_owned(), Arc::new(config));
    }
}
