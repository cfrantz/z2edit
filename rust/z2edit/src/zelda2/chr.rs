use anyhow::{ensure, Result};
use python_gui::{Color, Image};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChrMemory {
    #[serde(skip)]
    pub data: Arc<Mutex<Vec<u8>>>,
    #[serde(skip)]
    pub orig: Arc<Mutex<Vec<u8>>>,
}

#[typetag::serde]
impl GameData for ChrMemory {
    fn name(&self) -> String {
        "ChrMemory".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    //fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
    //    crate::gui::zelda2::palette::PaletteGroupEditor::new(self, name)
    //}
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChrSchema {
    #[default]
    Mmc1_4k,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    #[default]
    Tile = 0,
    Sprite = 1,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChrBank {
    pub schema: ChrSchema,
    pub address: Address,
    pub layout: Layout,
    pub border: i32,
    #[serde(skip)]
    pub data: Arc<Mutex<Vec<u8>>>,
    #[serde(skip)]
    pub orig: Arc<Mutex<Vec<u8>>>,
}

#[typetag::serde]
impl GameData for ChrBank {
    fn name(&self) -> String {
        "ChrBank".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        crate::gui::zelda2::chr::ChrBankEditor::new(self, name)
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
    pub struct ChrMemory {
        pub schema: ChrSchema,
        pub banks: usize,
    }
}

impl config::ChrMemory {
    pub fn unpack(&self, rom: &NesFile, path: &str, edits: &mut EditList) -> Result<()> {
        log::debug!("ChrMemory::unpack {path}");
        let length = rom.chr_banks() * 8192;
        let data = rom.read_bytes(Address::Chr(0, 0), length)?.to_vec();
        let orig = Arc::new(Mutex::new(data.clone()));
        let data = Arc::new(Mutex::new(data));
        let chr = ChrMemory {
            data: Arc::clone(&data),
            orig: Arc::clone(&orig),
        };
        edits.insert(path.into(), Edit::new(chr.into()));
        match self.schema {
            ChrSchema::Mmc1_4k => {
                ensure!(
                    self.banks == length / 4096,
                    Error::Configuration(format!(
                        "ChrMemory::banks incorrect (got {} but expecting {})",
                        self.banks,
                        length / 4096
                    ))
                );
                for bank in 0..self.banks {
                    let chr = ChrBank {
                        schema: self.schema,
                        address: Address::Chr(bank as i16, 0),
                        data: Arc::clone(&data),
                        orig: Arc::clone(&orig),
                        ..Default::default()
                    };
                    log::debug!("ChrMemory::unpack {path}/{bank}");
                    edits.insert(format!("{path}/{bank}"), Edit::new(chr.into()));
                }
            }
        };
        Ok(())
    }

    pub fn pack(&self, rom: &mut NesFile, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("ChrMemory::pack {path}");
        if let Some(edit) = edits.get(path) {
            match self.schema {
                ChrSchema::Mmc1_4k => {
                    for bank in 0..self.banks {
                        if let Some(edit) = edits.get(&format!("{path}/{bank}")) {
                            log::debug!("ChrMemory::pack {path}/{bank}");
                            let _chr = edit.data_ref::<ChrBank>()?;
                            // Do it
                        }
                    }
                }
            }
            let chr = edit.data_ref::<ChrMemory>()?;
            rom.write_bytes(Address::Chr(0, 0), chr.data.lock().unwrap().as_slice())?;
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }

    pub fn get<T: Any>(&self, path: &[&str]) -> Result<&T> {
        match path {
            [] => get_config::<T>(self),
            [n] if n.parse::<usize>()? < self.banks => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("ChrBank/{path:?}")).into()),
        }
    }
}

impl ChrBank {
    pub fn create_image(&self, border: u32, layout: Layout) -> Result<Image> {
        let (nw, nh, tw, th, mult) = match (self.schema, layout) {
            (ChrSchema::Mmc1_4k, Layout::Tile) => (16, 16, 8, 8, 1),
            (ChrSchema::Mmc1_4k, Layout::Sprite) => (16, 8, 8, 16, 2),
        };
        let mut image = Image::with_color(
            border + nw * (tw + border),
            border + nh * (th + border),
            Color::new(0xFFFF00FF),
        );
        let colors = [
            Color::new(0xFF000000),
            Color::new(0xFF555555),
            Color::new(0xFFAAAAAA),
            Color::new(0xFFFFFFFF),
        ];
        let base = self.address.norm_offset()?;
        let data = self.data.lock().unwrap();
        for y in 0..nh {
            for x in 0..nw {
                let mut tile = mult * (y * nw + x);
                for yy in 0..th {
                    if layout == Layout::Sprite && yy == 8 {
                        tile += 1;
                    }
                    let mut lo = data[base + (tile * 16 + yy % 8) as usize].reverse_bits() as usize;
                    let mut hi =
                        data[base + (tile * 16 + 8 + yy % 8) as usize].reverse_bits() as usize;
                    for xx in 0..tw {
                        let color = (lo & 1) + ((hi & 1) << 1);
                        image.set_pixel(
                            border + x * (tw + border) + xx,
                            border + y * (th + border) + yy,
                            colors[color],
                        );
                        lo >>= 1;
                        hi >>= 1;
                    }
                }
            }
        }
        image.update();
        Ok(image)
    }
}
