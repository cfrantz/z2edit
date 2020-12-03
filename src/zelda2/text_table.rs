use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::text_table::TextTableGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_encoding::Text;

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct TextTable {
        pub id: String,
        pub name: String,
        pub index: usize,
        pub length: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub pointer: Address,
        pub table: Vec<TextTable>,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::TextTable> {
            path.check_len("text table", 2)?;
            for table in self.table.iter() {
                if path.at(0) == table.id {
                    return Ok(table);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/text_table.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextItem {
    pub id: IdPath,
    pub text: String,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextTable {
    pub data: Vec<TextItem>,
}

impl TextTable {
    fn merge(&mut self, other: &TextTable) {
        for o in other.data.iter() {
            for d in self.data.iter_mut() {
                if d.id == o.id {
                    d.text = o.text.clone();
                }
            }
        }
    }
}

#[typetag::serde]
impl RomData for TextTable {
    fn name(&self) -> String {
        "TextTable".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let rom = edit.rom.borrow();

        self.data.clear();
        for tcfg in config.text_table.table.iter() {
            let table = rom.read_pointer(config.text_table.pointer + tcfg.index * 2)?;
            for i in 0..tcfg.length {
                let str_ptr = rom.read_pointer(table + i * 2)?;
                self.data.push(TextItem {
                    id: IdPath(vec![tcfg.id.clone(), i.to_string()]),
                    text: Text::from_zelda2(rom.read_terminated(str_ptr, 0xff)?),
                });
            }
        }
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let mut all = TextTable::default();
        all.unpack(edit)?;
        let config = Config::get(&edit.meta.borrow().config)?;
        let mut rom = edit.rom.borrow_mut();
        let mut memory = edit.memory.borrow_mut();

        // Free the entire block of text.
        for item in all.data.iter() {
            let tcfg = config.text_table.find(&item.id)?;
            let table = rom.read_pointer(config.text_table.pointer + tcfg.index * 2)?;
            let index = item.id.usize_at(1)?;
            let str_ptr = rom.read_pointer(table + index * 2)?;
            memory.free(str_ptr, item.text.len() as u16 + 1);
        }
        // Merge in the changed strings.
        all.merge(self);

        // Re-allocate and write the strings into rom.
        for item in all.data.iter() {
            let tcfg = config.text_table.find(&item.id)?;
            let table = rom.read_pointer(config.text_table.pointer + tcfg.index * 2)?;
            let index = item.id.usize_at(1)?;
            let str_ptr = rom.read_pointer(table + index * 2)?;
            let str_ptr = memory.alloc_near(str_ptr, item.text.len() as u16 + 1)?;
            info!(
                "Writing {:?} to {:x?}: '{}'",
                item.id.to_string(),
                str_ptr,
                item.text
            );
            rom.write_word(table + index * 2, str_ptr.raw() as u16)?;
            rom.write_terminated(str_ptr, &Text::to_zelda2(&item.text), 0xff)?;
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        TextTableGui::new(project, commit_index)
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
