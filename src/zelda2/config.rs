use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::errors::*;
use crate::nes::{Layout, Segment};
use crate::zelda2::enemyattr;
use crate::zelda2::palette;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub layout: Layout,
    pub palette: palette::config::Config,
    pub enemy: enemyattr::config::Config,
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

impl Default for Config {
    fn default() -> Self {
        Config {
            layout: zelda2_nesfile_layout(),
            palette: palette::config::Config::default(),
            enemy: enemyattr::config::Config::default(),
        }
    }
}

static mut CONFIGS: Lazy<Mutex<HashMap<String, Arc<Config>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("vanilla".to_owned(), Arc::new(Config::default()));
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
