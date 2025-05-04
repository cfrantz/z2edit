use anyhow::{ensure, Result};
use pathdiff::diff_paths;
use pyo3::prelude::*;
use python_gui::{Color, Image};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error::Error;
use crate::gui::Gui;
use crate::nes::{Address, NesFile};
use crate::zelda2::config::get_config;
use crate::zelda2::edit::{Edit, EditList, GameData};
use crate::zelda2::project::Project;

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
    Mmc5_1k,
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
    pub overlay: Vec<String>,
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
    fn fixup_paths(&mut self, project_path: &Path) -> Result<()> {
        for overlay in self.overlay.iter_mut() {
            if let Some(path) = diff_paths(&*overlay, project_path) {
                *overlay = path.to_string_lossy().into();
            }
            *overlay = overlay.replace('\\', "/");
        }
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
    pub fn unpack(
        &self,
        rrom: &Bound<'_, NesFile>,
        path: &str,
        edits: &mut EditList,
    ) -> Result<()> {
        log::debug!("ChrMemory::unpack {path}");
        let rom = rrom.borrow();
        let length = rom.chr_banks() * 8192;
        let data = rom.read_bytes(Address::Chr(0, 0), length)?.to_vec();
        let orig = Arc::new(Mutex::new(data.clone()));
        let data = Arc::new(Mutex::new(data));
        let chr = ChrMemory {
            data: Arc::clone(&data),
            orig: Arc::clone(&orig),
        };
        edits.insert(path.into(), Edit::new(chr.into()));
        let banksz = match self.schema {
            ChrSchema::Mmc1_4k => 4096,
            ChrSchema::Mmc5_1k => 1024,
        };

        ensure!(
            self.banks == length / banksz,
            Error::Configuration(format!(
                "ChrMemory::banks incorrect (got {} but expecting {})",
                self.banks,
                length / banksz
            ))
        );

        for bank in 0..self.banks {
            let address = match self.schema {
                ChrSchema::Mmc1_4k => Address::Chr(bank as i16, 0),
                ChrSchema::Mmc5_1k => Address::Chr1k(bank as i16, 0),
            };
            let chr = ChrBank {
                schema: self.schema,
                address,
                data: Arc::clone(&data),
                orig: Arc::clone(&orig),
                ..Default::default()
            };
            log::debug!("ChrMemory::unpack {path}/{bank}");
            edits.insert(format!("{path}/{bank}"), Edit::new(chr.into()));
        }
        Ok(())
    }

    pub fn pack(&self, rrom: &Bound<'_, NesFile>, path: &str, edits: &EditList) -> Result<()> {
        log::debug!("ChrMemory::pack {path}");
        if let Some(edit) = edits.get(path) {
            for bank in 0..self.banks {
                if let Some(edit) = edits.get(&format!("{path}/{bank}")) {
                    log::debug!("ChrMemory::pack {path}/{bank}");
                    let chr = edit.data_ref::<ChrBank>()?;
                    chr.apply()?;
                }
            }

            let mem = edit.data_ref::<ChrMemory>()?;
            let mut rom = rrom.borrow_mut();
            rom.write_bytes(Address::Chr(0, 0), mem.data.lock().unwrap().as_slice())?;
        } else {
            log::warn!("No data for {path:?}");
        }
        Ok(())
    }

    pub fn post_unpack_fixup(&self, path: &str, edits: &mut EditList) -> Result<()> {
        // ChrBank objects are special: they share their CHR data with a single
        // ChrMemory object and they don't save the CHR data into the save file.
        // Upon reloading, we have to re-establish the shared data structure with the
        // original ChrMemory object.
        if let Some(edit) = edits.get(path) {
            let (orig, data) = {
                let chrmem = edit.data_ref::<ChrMemory>()?;
                (Arc::clone(&chrmem.orig), Arc::clone(&chrmem.data))
            };
            for bank in 0..self.banks {
                if let Some(edit) = edits.get_mut(&format!("{path}/{bank}")) {
                    let chrbank = edit.data_mut::<ChrBank>()?;
                    let empty = chrbank.orig.lock().unwrap().is_empty()
                        || chrbank.data.lock().unwrap().is_empty();
                    if empty {
                        chrbank.orig = Arc::clone(&orig);
                        chrbank.data = Arc::clone(&data);
                    }
                    chrbank.apply()?;
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
            [n] if n.parse::<usize>()? < self.banks => get_config::<T>(self),
            _ => Err(Error::NotFound(format!("ChrBank/{path:?}")).into()),
        }
    }
}

impl ChrBank {
    fn copy_orig(&self) {
        let len = match self.schema {
            ChrSchema::Mmc1_4k => 4096,
            ChrSchema::Mmc5_1k => 1024,
        };
        let base = self.address.norm_offset().expect("chr address");
        let orig = self.orig.lock().unwrap();
        let mut data = self.data.lock().unwrap();
        data[base..base + len].copy_from_slice(&orig[base..base + len]);
    }

    pub fn apply(&self) -> Result<()> {
        self.copy_orig();
        for overlay in self.overlay.iter() {
            if !overlay.is_empty() {
                self.import(overlay)?;
            }
        }
        Ok(())
    }

    pub fn create_image(&self, border: u32, layout: Layout) -> Result<Image> {
        let (nw, nh, tw, th, mult) = match (self.schema, layout) {
            (ChrSchema::Mmc1_4k, Layout::Tile) => (16, 16, 8, 8, 1),
            (ChrSchema::Mmc1_4k, Layout::Sprite) => (16, 8, 8, 16, 2),
            (ChrSchema::Mmc5_1k, Layout::Tile) => (16, 4, 8, 8, 1),
            (ChrSchema::Mmc5_1k, Layout::Sprite) => (16, 2, 8, 16, 2),
        };
        let mut image = Image::with_color(
            border + nw * (tw + border),
            border + nh * (th + border),
            Color::new(0xFFFF00FF),
        );
        let colors = [
            Color::new(0xFF000000),
            Color::new(0xFF666666),
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
        Ok(image)
    }

    pub fn parse_image(&self, border: u32, layout: Layout, image: &Image) {
        let (nw, nh, tw, th, mult) = match (self.schema, layout) {
            (ChrSchema::Mmc1_4k, Layout::Tile) => (16, 16, 8, 8, 1),
            (ChrSchema::Mmc1_4k, Layout::Sprite) => (16, 8, 8, 16, 2),
            (ChrSchema::Mmc5_1k, Layout::Tile) => (16, 4, 8, 8, 1),
            (ChrSchema::Mmc5_1k, Layout::Sprite) => (16, 2, 8, 16, 2),
        };
        let base = self.address.norm_offset().expect("chr address");
        let mut data = self.data.lock().unwrap();
        for y in 0..nh {
            for x in 0..nw {
                let mut tile = mult * (y * nw + x);
                for yy in 0..th {
                    if layout == Layout::Sprite && yy == 8 {
                        tile += 1;
                    }
                    let mut lo = data[base + (tile * 16 + yy % 8) as usize];
                    let mut hi = data[base + (tile * 16 + 8 + yy % 8) as usize];
                    for xx in 0..tw {
                        let color = image.get_pixel(
                            border + x * (tw + border) + xx,
                            border + y * (th + border) + yy,
                        );
                        if color.a != 0 {
                            let avg = color.average();
                            let mask = 1u8 << ((tw - 1) - xx);
                            let (hibit, lobit) = if avg > 0xAA {
                                (mask, mask)
                            } else if avg > 0x66 {
                                (mask, 0)
                            } else if avg > 0x00 {
                                (0, mask)
                            } else {
                                (0, 0)
                            };
                            lo = (lo & !mask) | lobit;
                            hi = (hi & !mask) | hibit;
                        }
                    }
                    data[base + (tile * 16 + yy % 8) as usize] = lo;
                    data[base + (tile * 16 + 8 + yy % 8) as usize] = hi;
                }
            }
        }
    }

    pub fn import<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // FIXME: Naughty use of global variable to know the project path.
        let path = Project::path().join(path.as_ref());
        let image = Image::load_bmp(path)?;
        let (border, layout) = match (self.schema, image.width, image.height) {
            (ChrSchema::Mmc1_4k, 128, 128) => (0, self.layout),
            (ChrSchema::Mmc1_4k, 145, 145) => (1, Layout::Tile),
            (ChrSchema::Mmc1_4k, 145, 137) => (1, Layout::Sprite),
            (ChrSchema::Mmc1_4k, 162, 162) => (2, Layout::Tile),
            (ChrSchema::Mmc1_4k, 162, 146) => (2, Layout::Sprite),
            (ChrSchema::Mmc5_1k, 128, 32) => (0, self.layout),
            (ChrSchema::Mmc5_1k, 145, 37) => (1, Layout::Tile),
            (ChrSchema::Mmc5_1k, 145, 35) => (1, Layout::Sprite),
            (ChrSchema::Mmc5_1k, 162, 41) => (2, Layout::Tile),
            (ChrSchema::Mmc5_1k, 162, 38) => (2, Layout::Sprite),
            _ => {
                return Err(Error::NotImplemented(format!(
                    "Cannot process image size {}x{} for schema {:?}",
                    image.width, image.height, self.schema,
                ))
                .into())
            }
        };

        self.parse_image(border, layout, &image);
        Ok(())
    }
}
