use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::nes::{Address, NesFile};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Palette {
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaletteGroup {
    pub group: IndexMap<String, Palette>,
}

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Palette {
        pub name: String,
        pub address: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub magic_background: Option<Address>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct PaletteGroup {
        pub name: String,
        #[serde(flatten)]
        pub group: IndexMap<String, Palette>,
    }
}

impl config::Palette {
    pub fn unpack(&self, rom: &NesFile) -> Result<Palette> {
        let length = self.length.unwrap_or(16);
        let data = rom.read_bytes(self.address, length)?.to_vec();
        Ok(Palette { data })
    }

    pub fn pack(&self, rom: &mut NesFile, palette: &Palette) -> Result<()> {
        rom.write_bytes(self.address, &palette.data)?;
        if let Some(bg) = self.magic_background {
            rom.write(bg, palette.data[0])?;
        }
        Ok(())
    }
}

impl config::PaletteGroup {
    pub fn unpack(&self, rom: &NesFile) -> Result<PaletteGroup> {
        let mut result = PaletteGroup::default();
        for (name, pal) in self.group.iter() {
            result.group.insert(name.into(), pal.unpack(rom)?);
        }
        Ok(result)
    }

    pub fn pack(&self, rom: &mut NesFile, pg: &PaletteGroup) -> Result<()> {
        for (name, pal) in self.group.iter() {
            if let Some(p) = pg.group.get(name) {
                pal.pack(rom, p)?;
            }
        }
        Ok(())
    }
}
