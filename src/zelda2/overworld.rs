use std::any::Any;
use std::rc::Rc;
use std::convert::{From, TryFrom};
use ron;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::zelda2::overworld::OverworldGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, Buffer, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Overworld {
        pub id: String,
        pub name: String,
        pub overworld: usize,
        pub subworld: usize,
        pub pointer: Address,
        pub connector: Address,
        pub encounter: Address,
        pub objtable: Address,
        pub objtable_len: usize,
        pub tile_palette: Address,
        pub palette: Address,
        pub chr: Address,
        pub width: usize,
        pub height: usize,
        pub palace_to_stone_table: (Address, usize),
        pub raft_connector: usize,
        pub raft_table: Address,
    }

    #[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum HiddenKind {
        None,
        Palace,
        Town,
    }

    impl Default for HiddenKind {
        fn default() -> Self { HiddenKind::None }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HiddenSpot {
        pub id: String,
        pub kind: HiddenKind,
        pub connector: Address,
        pub overworld: Address,
        pub x: Address,
        pub y: Address,
        pub return_y: Address,
        pub ppu_macro: Address,
        pub discriminator: Address,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub mason_dixon_line: Address,
        pub y_offset: i32,
        pub map: Vec<config::Overworld>,
        pub hidden: Vec<config::HiddenSpot>,
        pub terrain_name: Vec<String>,
        pub palace_connectors: Vec<usize>,
        pub overworld_ram: u16,
        pub overworld_len: usize,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::Overworld> {
            path.check_range("overworld", 1..3)?;
            for overworld in self.map.iter() {
                if path.at(0) == overworld.id {
                    return Ok(overworld);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn find_hidden(&self, path: &IdPath) -> Result<&config::HiddenSpot> {
            path.check_len("hidden", 1)?;
            for hidden in self.hidden.iter() {
                if path.at(0) == hidden.id {
                    return Ok(hidden);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/overworld.ron")).unwrap()
        }
    }
}

use config::HiddenKind;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(try_from = "JsonMap")]
#[serde(into = "JsonMap")]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<u8>>,
}

struct CompressedMap {
    data: Vec<u8>,
    palace_offset: [u16; 4],
    hidden_palace: Option<u8>,
    hidden_town: Option<u8>,
}

impl Default for CompressedMap {
    fn default() -> Self {
        CompressedMap {
            data: Vec::new(),
            palace_offset: [0xffffu16; 4],
            hidden_palace: None,
            hidden_town: None,
        }
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Map {
            width: width,
            height: height,
            ..Default::default()
        }
    }

    // Why can't I use 'rom: &dyn MemoryAccess' ?
    fn decompress(&mut self, addr: Address, rom: &Buffer) -> Result<usize> {
        self.data = vec![vec![0xf as u8; self.width]; self.height];
        let mut y = 0;
        let mut index = 0;
        while y < self.height {
            let mut x = 0;
            while x < self.width {
                let val = rom.read(addr + index)?;
                let tile = val & 0x0F;
                let count = (val >> 4) as usize + 1;
                for _ in 0..count {
                    self.data[y][x] = tile;
                    x += 1;
                }
                index += 1;
            }
            y += 1;
        }
        Ok(index)
    }

    fn compress(&self, overworld: &Overworld, config: &config::Config) -> CompressedMap {
        let mut map = CompressedMap::default();
        for (y, row) in self.data.iter().enumerate() {
            let mut x = 0;
            while x < self.width {
                let mut count = 0u8;
                let mut want_compress = true;
                let tile = row[x];
                if let Some(conn) = overworld.connector_at(x, y) {
                    let index = conn.id.usize_at(1).unwrap();
                    if config.palace_connectors.contains(&index) {
                        let palace = index - config.palace_connectors[0];
                        want_compress = false;
                        map.palace_offset[palace] = map.data.len() as u16;
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                map.hidden_palace = Some(tile);
                            }
                        }
                    } else {
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                want_compress = false;
                                map.hidden_town = Some(tile);
                            }
                        }
                    }
                }
                if tile == 0x0E || tile == 0x0F {
                    want_compress = false;
                }

                while want_compress &&
                      count < 15 &&
                      x+1 < self.width &&
                      tile == row[x+1] &&
                      !overworld.skip_compress(x+1, y) {
                    x += 1;
                    count += 1;
                }
                map.data.push(tile | count << 4);
                x += 1;
            }
        }
        map
    }
}

const JSONMAP_TRANSFORM: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_*".as_bytes();

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<String>,
}

impl From<&Map> for JsonMap {
    fn from(a: &Map) -> Self {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = String::new();
            for col in row.iter() {
                s.push(JSONMAP_TRANSFORM[*col as usize] as char);
            }
            map.push(s);
        }
        JsonMap {
            width: a.width,
            height: a.height,
            data: map,
        }

    }
}

impl From<Map> for JsonMap {
    fn from(a: Map) -> Self {
        JsonMap::from(&a)
    }
}

impl TryFrom<JsonMap> for Map {
    type Error = Error;
    fn try_from(a: JsonMap) -> Result<Self> {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = Vec::new();
            for col in row.chars() {
                let v = JSONMAP_TRANSFORM.iter().position(|&x| col as u8 == x).ok_or_else(
                    || -> Error { ErrorKind::TransformationError(format!(
                            "Map transform could not convert '{}'", col)).into() } )?;
                s.push(v as u8);
            }
            map.push(s);
        }
        Ok(Map {
            width: a.width,
            height: a.height,
            data: map,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hidden {
    pub hidden: bool,
    pub id: IdPath,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: IdPath,

    pub x: i32,
    pub y: i32,
    pub dest_map: i32,
    pub dest_world: usize,
    pub dest_overworld: usize,

    pub external: bool,
    pub second: bool,
    pub exit_2_lower: bool,
    pub entry: usize,
    pub entry_right: bool,
    pub passthru: bool,
    pub fall: bool,
    pub hidden: Option<Hidden>,
    pub palace: Option<usize>,
}

impl Connector {
    fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut connector = Connector::default();
        connector.id = id;
        connector.unpack(edit)?;
        Ok(connector)
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let ocfg = config.overworld.find(&self.id)?;
        let index = self.id.usize_at(1)?;
        let rom = edit.rom.borrow();

        let y = rom.read(ocfg.connector + index + 0x00)?;
        let x = rom.read(ocfg.connector + index + 0x3f)?;
        let z = rom.read(ocfg.connector + index + 0x7e)?;
        let w = rom.read(ocfg.connector + index + 0xbd)?;

        self.set_y(y, &config.overworld);

        self.x = (x & 0x3f) as i32;
        self.second = (x & 0x40) != 0;
        self.exit_2_lower = (x & 0x80) != 0;

        self.dest_map = (z & 0x3f) as i32;
        self.entry = (z >> 6) as usize;

        self.dest_overworld = (w & 0x03) as usize;
        self.dest_world = (w & 0x1c) as usize >> 2;
        self.entry_right = (w & 0x20) != 0;
        self.passthru = (w & 0x40) != 0;
        self.fall = (w & 0x80) != 0;

        self.palace = if config.overworld.palace_connectors.contains(&index) {
            Some(index - config.overworld.palace_connectors[0])
        } else {
            None
        };

        self.hidden =
            if let Some(spot) = self.hidden_index(edit, &config.overworld)? {
                self.set_y(rom.read(spot.connector + 2)?, &config.overworld);
                Some(Hidden {
                    hidden: y == 0,
                    id: IdPath(vec![spot.id.clone()]),
                })
            } else {
                None
            };

        Ok(())
    }

    fn hidden_index<'a>(&self, edit: &Rc<Edit>, config: &'a config::Config) -> Result<Option<&'a config::HiddenSpot>> {
        let ocfg = config.find(&self.id)?;
        let index = self.id.usize_at(1)?;
        let rom = edit.rom.borrow();
        for h in config.hidden.iter() {
            if rom.read(h.connector)? == index as u8 &&
               rom.read(h.overworld)? == ocfg.overworld as u8 {
                return Ok(Some(h));
            }
        }
        Ok(None)
    }

    fn set_y(&mut self, y: u8, config: &config::Config) {
        self.y = (y & 0x7f) as i32 - config.y_offset;
        self.external = (y & 0x80) != 0;
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let ocfg = config.overworld.find(&self.id)?;
        let index = self.id.usize_at(1)?;
        let mut rom = edit.rom.borrow_mut();


        let y = if self.external { 0x80 } else { 0x00 }
              | (self.y + config.overworld.y_offset) as u8;
        let x = if self.second { 0x40 } else { 0x00 }
              | if self.external { 0x80 } else { 0x00 }
              | self.x as u8;
        let z = (self.entry << 6) as u8
              | self.dest_map as u8;

        let w = if self.entry_right { 0x20 } else { 0x00 }
              | if self.passthru { 0x40 } else { 0x00 }
              | if self.fall { 0x80 } else { 0x00 }
              | self.dest_overworld as u8
              | (self.dest_world << 2) as u8;
        
        rom.write(ocfg.connector + index + 0x00, y)?;
        rom.write(ocfg.connector + index + 0x3f, x)?;
        rom.write(ocfg.connector + index + 0x7e, z)?;
        rom.write(ocfg.connector + index + 0xbd, w)?;

        if let Some(hidden) = &self.hidden {
            let spot = config.overworld.find_hidden(&hidden.id)?;
            if spot.kind == HiddenKind::Palace {
                // Hidden palace Y coordinate includes the overworld_y_offset.
                // Furthermore, the "call" spot is 2 tiles north of the target
                // destination.
                if spot.ppu_macro.raw() == 0 {
                    return Err(ErrorKind::ConfigError(
                      format!("No ppu_macro address defined for {}", spot.id.to_string())).into());
                }
                rom.write(spot.connector + 2, y)?;
                rom.write(spot.y, (y & 0x7f) - 2)?;
                rom.write(spot.x, self.x as u8)?;
                rom.write(spot.return_y, y & 0x7f)?;

                // Compute the destination address in VRAM.
                let xx = self.x as u16;
                let yy = self.y as u16;
                let ppu_addr = 0x2000 + 2 * (32*(yy % 15) + (xx % 16)) +            
                               0x800 * (yy % 30 / 15);

                rom.write(spot.ppu_macro + 0, (ppu_addr >> 8) as u8)?;
                rom.write(spot.ppu_macro + 1, ppu_addr as u8)?;
                rom.write(spot.ppu_macro + 5, ((ppu_addr + 32)>> 8) as u8)?;
                rom.write(spot.ppu_macro + 6, (ppu_addr + 32) as u8)?;
                // Being lazy and not dealing with the color bits.
                rom.write(spot.ppu_macro + 10, 0xff)?;

                if hidden.hidden {
                    rom.write(ocfg.connector + index + 0x00, 0)?;
                }
            } else if spot.kind == HiddenKind::Town {
                // Hidden Town Y coordinate does not include the overworld_y_offset.     
                // Furthermore, the x location seems to be 1 more than the actual        
                // coordinate.                                                           
                if spot.discriminator.raw() == 0 {
                    return Err(ErrorKind::ConfigError(
                      format!("No discriminator address defined for {}", spot.id.to_string())).into());
                }
                rom.write(spot.connector + 2, y)?;
                rom.write(spot.y, self.y as u8)?;
                rom.write(spot.x, self.x as u8 + 1)?;
                rom.write(spot.return_y, y & 0x7f)?;
                rom.write(spot.discriminator, self.x as u8)?;
                if hidden.hidden {
                    rom.write(ocfg.connector + index + 0x00, 0)?;
                }
            } else {
                return Err(ErrorKind::ConfigError(
                  format!("Don't know how to pack hidden spot {}", spot.id.to_string())).into());
            }
        }
        if index == ocfg.raft_connector {
            // The raft table is stored as xpos, xpos, ypos, ypos.
            // The config stores a start location per overworld.
            rom.write(ocfg.raft_table + 0, x & 0x3f)?;
            rom.write(ocfg.raft_table + 2, y & 0x7f)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub dest_map: i32,
    pub entry: usize,
}

impl From<u8> for Encounter {
    fn from(a: u8) -> Encounter {
        Encounter {
            dest_map: (a & 0x3f) as i32,
            entry: (a >> 6) as usize,
        }
    }
}

impl From<&Encounter> for u8 {
    fn from(a: &Encounter) -> u8 {
        (a.dest_map & 0x3f) as u8 | (a.entry << 6) as u8
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Overworld {
    pub id: IdPath,
    pub mason_dixon_line: i32,
    pub map: Map,
    pub encounter: Vec<Encounter>,
    pub connector: Vec<Connector>,
}

impl Overworld {
    pub fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut overworld = Overworld::default();
        overworld.id = id;
        overworld.unpack(edit)?;
        Ok(overworld)
    }

    pub fn skip_compress(&self, x: usize, y: usize) -> bool {
        let x = x as i32;
        let y = y as i32;
        for c in self.connector.iter() {
            if let Some(h) = &c.hidden {
                if h.hidden && c.x == x && c.y == y {
                    return true;
                }
            }
            if c.palace.is_some() && c.x == x && c.y == y {
                return true;
            }
        }
        false
    }

    fn connector_at(&self, x: usize, y: usize) -> Option<&Connector> {
        let x = x as i32;
        let y = y as i32;
        for c in self.connector.iter() {
            if c.x == x && c.y == y {
                return Some(c);
            }
        }
        None
    }
}

#[typetag::serde]
impl RomData for Overworld {
    fn name(&self) -> String {
        "Overworld".to_owned()
    }
    fn as_any(&self) -> &dyn Any { self }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let ocfg = config.overworld.find(&self.id)?;
        let addr = edit.rom.borrow().read_pointer(ocfg.pointer)?;

        self.mason_dixon_line = edit.rom.borrow().read(config.overworld.mason_dixon_line)? as i32 - config.overworld.y_offset;
        self.map.width = ocfg.width;
        self.map.height = ocfg.height;
        self.map.decompress(addr, &edit.rom.borrow())?;
        self.encounter = edit.rom.borrow().read_bytes(ocfg.encounter, 14)?
            .iter()
            .map(|byte| Encounter::from(*byte))
            .collect();

        self.connector.clear();
        for index in 0..63 {
            self.connector.push(
                Connector::from_rom(edit, IdPath(vec![ocfg.id.clone(), index.to_string()]))?);
        }

        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let ocfg = config.overworld.find(&self.id)?;
        let map = self.map.compress(self, &config.overworld);
        if map.data.len() >= config.overworld.overworld_len {
            return Err(ErrorKind::LengthError(format!(
                        "Overworld too big: compressed size of {} bytes is larger than {} bytes",
                        map.data.len(), config.overworld.overworld_len)).into());
        }
        {
            let mut memory = edit.memory.borrow_mut();
            let mut rom = edit.rom.borrow_mut();
            let addr = rom.read_pointer(ocfg.pointer)?;

            let length = {
                let mut orig = Map::new(ocfg.width, ocfg.height);
                orig.decompress(addr, &rom)? as u16
            };

            memory.free(addr, length);
            let addr = memory.alloc_near(addr, map.data.len() as u16)?;
            info!("Overworld::pack: storing {} at {:x?} using {} bytes", self.id, addr, map.data.len());
            rom.write_bytes(addr, &map.data)?;

            let (addr, length) = ocfg.palace_to_stone_table;
            for i in 0..length {
                if map.palace_offset[i] != 0xFFFF {
                    rom.write_word(addr + i*2, config.overworld.overworld_ram + map.palace_offset[i])?
                }
            }

            for (i, encounter) in self.encounter.iter().enumerate() {
                rom.write(ocfg.encounter + i, u8::from(encounter))?;
            }
        }
        for connector in self.connector.iter() {
            connector.pack(edit)?;
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        OverworldGui::new(project, commit_index)
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => { *self = v; Ok(()) },
            Err(e) => Err(e.into()),
        }
    }
}
