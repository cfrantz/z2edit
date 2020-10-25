use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

use crate::errors::*;
use crate::zelda2::palette;
use crate::nes::{Layout, Segment};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub layout: Layout,
    pub palette: palette::config::Config,
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
        }
    }
}

/*
static CONFIGS: Lazy<RefCell<HashMap<String, Config>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("vanilla".to_owned(), Config::default());
    RefCell::new(m)
});
*/

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

/*
    pub fn get(name: &str) -> Result<&'static Config> {
        let configs = CONFIGS.borrow();
        let result = configs.get(name).ok_or_else(
            || ErrorKind::ConfigNotFound(name.to_owned()).into());
        result
    }
    */
}
