use std::any::Any;
use std::rc::Rc;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::gui::zelda2::import_chr::ImportChrBankGui;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::nes::Address;
use crate::util::relative_path::PathConverter;
use crate::zelda2::project::{Edit, Project, RomData};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub enum ChrBankKind {
    MMC1_4k,
    MMC5_1k,
    VBanks,
}

impl Default for ChrBankKind {
    fn default() -> Self {
        ChrBankKind::MMC1_4k
    }
}

#[derive(Debug, SmartDefault, Clone, Serialize, Deserialize)]
pub struct ImportChrBank {
    #[serde(default)]
    pub kind: ChrBankKind,
    pub bank: usize,
    pub file: PathConverter,
    #[default(_code = "[0xFF000000, 0xFF666666, 0xFFAAAAAA, 0xFFFFFFFF]")]
    pub palette: [u32; 4],
    #[default = true]
    pub sprite_layout: bool,
    #[default = 2]
    pub border: i32,
}

impl ImportChrBank {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }

    pub fn schema(&self) -> Schema {
        match self.kind {
            ChrBankKind::MMC1_4k => Schema::MMC1_4k,
            ChrBankKind::MMC5_1k => Schema::MMC5_1k,
            ChrBankKind::VBanks => Schema::VBanks,
        }
    }
}

#[typetag::serde]
impl RomData for ImportChrBank {
    fn name(&self) -> String {
        "ImportChrBank".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn gui(&self, project: &Project, edit: &Rc<Edit>) -> Result<Box<dyn Gui>> {
        ImportChrBankGui::new(project, Some(Rc::clone(edit)))
    }

    fn unpack(&mut self, _edit: &Rc<Edit>) -> Result<()> {
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let cache = TileCache::new(edit, self.schema());
        let chr = Address::Chr(self.bank as isize, 0);
        let mut bank =
            cache.get_bank(chr, &self.palette, self.sprite_layout, self.border as u32)?;

        let filepath = edit.subdir.path(&self.file.as_ref());
        let graphics = Image::load_bmp(&filepath)?;
        bank.overlay(&graphics, 0, 0);
        cache.put_bank(
            &bank,
            chr,
            &self.palette,
            self.sprite_layout,
            self.border as u32,
        )?;
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
