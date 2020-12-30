use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::sideview::SideviewGui;
use crate::gui::zelda2::Gui;
use crate::idpath;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::objects::config::Config as ObjectConfig;
use crate::zelda2::objects::config::{Object, ObjectArea, ObjectKind, Renderer};
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_table::config::TextTable;

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Background {
        pub name: String,
        pub ceiling: [u8; 2],
        pub floor: [u8; 2],
        #[serde(default)]
        pub alternate: u8,
        pub background: u8,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct SideviewGroup {
        pub id: IdPath,
        pub name: String,
        pub kind: ObjectArea,
        pub world: usize,
        pub overworld: usize,
        pub subworld: usize,
        pub length: usize,
        pub address: Address,
        pub enemies: Address,
        pub palette: Address,
        pub connections: Address,
        pub max_connectable_index: usize,
        pub doors: Address,
        pub max_door_index: usize,
        pub availability: Address,
        pub metatile_table: Address,
        pub metatile_lengths: Vec<usize>,
        pub chr: Address,

        pub pet_names: HashMap<usize, String>,
        #[serde(default)]
        pub background_id: Option<IdPath>,
        #[serde(default)]
        pub background_layer: bool,
        pub background: Vec<Background>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EnemyList {
        pub bank: isize,
        pub ids: Vec<IdPath>,
        pub address: Address,
        pub length: u16,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub group: Vec<SideviewGroup>,
        pub enemy_list: Vec<EnemyList>,
        pub enemy_list_ram: u16,
        pub enemy_list_rom: u16,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::SideviewGroup> {
            path.check_range("sideview", 1..=3)?;
            for group in self.group.iter() {
                if path.prefix(&group.id) {
                    return Ok(group);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn name(&self, path: &IdPath) -> Result<String> {
            let i = path.usize_last()?;
            let group = self.find(path)?;
            let name = if let Some(pet_name) = group.pet_names.get(&i) {
                format!("{:02}: {} {} ({})", i, group.name, i, pet_name)
            } else {
                format!("{:02}: {} {}", i, group.name, i)
            };
            Ok(name)
        }

        pub fn find_by_world(
            &self,
            world: usize,
            overworld: usize,
            subworld: usize,
        ) -> Result<&config::SideviewGroup> {
            for group in self.group.iter() {
                if group.background_layer {
                    continue;
                }
                if world != 0 {
                    if world == group.world {
                        return Ok(group);
                    }
                } else {
                    if overworld == group.overworld && subworld == group.subworld {
                        return Ok(group);
                    }
                }
            }
            Err(ErrorKind::NotFound(format!(
                "Sideview group with world={}, overworld={}, subworld={}",
                world, overworld, subworld
            ))
            .into())
        }

        pub fn enemy_list_for(&self, id: &IdPath) -> Result<&config::EnemyList> {
            for el in self.enemy_list.iter() {
                for i in el.ids.iter() {
                    if id.at(0) == i.at(0) {
                        return Ok(el);
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(id.into()).into())
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/sideview.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapCommand {
    pub x: i32,
    pub y: i32,
    pub kind: usize,
    pub param: i32,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Map {
    pub objset: i32,
    pub width: i32,
    pub grass: bool,
    pub bushes: bool,
    pub ceiling: bool,
    pub floor: i32,
    pub tileset: i32,
    pub sprite_palette: i32,
    pub background_palette: i32,
    pub background_map: i32,
    pub cursor_moves_left: bool,
    pub data: Vec<MapCommand>,
}

impl Map {
    fn from_bytes(data: &[u8]) -> Self {
        let len = data[0] as usize;
        let mut commands = Vec::new();
        let mut i = 4;
        let mut xpos = 0;
        let mut cursor_moves_left = false;
        while i < len {
            let xy = data[i + 0];
            let kp = data[i + 1];

            // While reading in the map, we convert from relative-X to
            // absolute-X coordinates.
            let x = (xy & 0x0F) as i32;
            let y = (xy >> 4) as i32;
            if y == 14 {
                // Y-position 14 means "X-Skip".
                if x * 16 < xpos {
                    cursor_moves_left = true;
                }
                xpos = x * 16;
            } else {
                xpos += x;
            }

            let (kind, param) = if y == 13 || y == 14 {
                // Y-position 13 means new floor position.
                // Y-position 14 means "X-Skip".
                (0, kp)
            } else if kp < 0x0f {
                (kp, 0)
            } else if kp == 0x0f {
                i += 1;
                (kp, data[i + 1])
            } else {
                (kp & 0xF0, kp & 0x0F)
            };

            commands.push(MapCommand {
                x: xpos,
                y: y,
                kind: kind as usize,
                param: param as i32,
            });
            i += 2;
        }
        Map {
            objset: if data[1] & 0x80 != 0 { 1 } else { 0 },
            width: ((data[1] >> 5) & 3) as i32 + 1,
            grass: data[1] & 8 == 8,
            bushes: data[1] & 4 == 4,
            ceiling: data[2] & 0x80 == 0x00,
            floor: (data[2] & 0x0F) as i32,
            tileset: ((data[2] >> 4) & 7) as i32,
            sprite_palette: ((data[3] >> 6) & 3) as i32,
            background_palette: ((data[3] >> 3) & 7) as i32,
            background_map: (data[3] & 7) as i32,
            cursor_moves_left: cursor_moves_left,
            data: commands,
        }
    }
    fn sort_data(data: &mut Vec<MapCommand>) {
        data.sort_by(|a, b| {
            if a.x == b.x {
                // We wrap the y coordinate around so that meta-ops will
                // sort before regular ops (new floor, extra items)
                let ya = (a.y + 3) % 16;
                let yb = (b.y + 3) % 16;
                ya.cmp(&yb)
            } else {
                a.x.cmp(&b.x)
            }
        });
    }

    pub fn elevator(&self) -> Option<i32> {
        for item in self.data.iter() {
            // "extra" object of kind 0x50 is an elevator.
            if item.y == 15 && item.kind == 0x50 {
                return Some(item.x);
            }
        }
        None
    }

    pub fn sort(&mut self) {
        Map::sort_data(&mut self.data);
    }

    fn optimize(&self) -> Vec<MapCommand> {
        let mut data = self.data.clone();
        if !self.cursor_moves_left {
            Map::sort_data(&mut data);
        };
        data.retain(|elem| elem.y != 14);

        let mut x = 0;
        let mut i = 0;
        while i < data.len() {
            let delta = data[i].x - x;
            if delta < 0 || delta > 15 {
                let newx = data[i].x & !15;
                data.insert(
                    i,
                    MapCommand {
                        y: 14,
                        x: newx / 16,
                        ..Default::default()
                    },
                );
                x = newx;
                i += 1;
            } else {
                x = data[i].x;
            }
            i += 1;
        }
        data
    }

    fn to_relative(&self) -> Vec<MapCommand> {
        let mut result = Vec::new();
        let mut x = 0;
        for item in self.optimize().iter() {
            let mut command = item.clone();
            if item.y != 14 {
                command.x -= x;
                x = item.x;
            } else {
                x = item.x / 16;
            }
            result.push(command);
        }
        result
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.push(0); // length (filled in later).
        result.push(
            // flags
            ((self.objset as u8) << 7)
                | ((self.width - 1) as u8 & 3) << 5
                | if self.grass { 8 } else { 0 }
                | if self.bushes { 4 } else { 0 },
        );
        result.push(
            // floor & tileset.
            if self.ceiling { 0x00 } else { 0x80 }
                | self.floor as u8 & 0x0F
                | (self.tileset as u8 & 7) << 4,
        );
        result.push(
            // palettes.
            ((self.sprite_palette as u8 & 3) << 6)
                | ((self.background_palette as u8 & 7) << 3)
                | (self.background_map as u8 & 7),
        );

        let commands = self.to_relative();
        for c in commands.iter() {
            let xy = (c.x as u8 & 0xF) | (c.y as u8 & 0xF) << 4;
            let kind = match c.y {
                13 => c.param as u8,
                14 => 0,
                _ => {
                    if c.kind < 0x10 {
                        c.kind as u8
                    } else {
                        c.kind as u8 | c.param as u8 & 0x0F
                    }
                }
            };
            result.push(xy);
            result.push(kind);
            if c.kind == 0x0F {
                // Collectables are 3 bytes.
                result.push(c.param as u8);
            }
        }
        result[0] = result.len() as u8;
        result
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub kind: usize,
    #[serde(default)]
    pub dialog: Vec<IdPath>,
    #[serde(default)]
    pub condition: Option<u8>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyList {
    pub data: Vec<Vec<Enemy>>,
    pub is_encounter: bool,
    pub valid: bool,
    #[serde(skip)]
    pub ram_address: u16,
}

impl EnemyList {
    pub fn from_rom(edit: &Rc<Edit>, id: &IdPath, config: &Config) -> Result<Self> {
        let scfg = config.sideview.find(id)?;
        let index = id.usize_last()?;
        if index >= scfg.length {
            return Err(ErrorKind::IndexError(index).into());
        }

        let rom = edit.rom.borrow();
        // Enemy list.  Pointers are stored as RAM addresses.
        let delta = config.sideview.enemy_list_rom - config.sideview.enemy_list_ram;
        let ram_address = rom.read_pointer(scfg.enemies + index * 2)?;
        let addr = ram_address + delta;
        let length = rom.read(addr)? as usize;
        let mut total = length;
        let mut data = Vec::new();
        data.push(EnemyList::list_from_bytes(rom.read_bytes(addr, length)?));
        let is_encounter = config.overworld.is_encounter(edit, id)?;
        if is_encounter {
            // For encounters, the large-encounter enemy list immediately
            // follows the small-encounter list.
            let addr = addr + length;
            let length = rom.read(addr)? as usize;
            total += length;
            data.push(EnemyList::list_from_bytes(rom.read_bytes(addr, length)?));
        }

        debug!("EnemyList: {} read from {:x?} ({} bytes)", id, addr, total);
        let mut list = EnemyList {
            data: data,
            is_encounter: is_encounter,
            valid: true,
            ram_address: ram_address.raw() as u16,
        };
        list.read_text(edit, id, config)?;
        Ok(list)
    }

    pub fn write(&self, edit: &Rc<Edit>, id: &IdPath, config: &Config) -> Result<()> {
        if !self.valid {
            return Ok(());
        }
        let (el, n, mut all) = self.read_all(edit, id, config)?;
        let index = id.usize_last()?;

        all[n * 63 + index] = self.clone();
        self.write_text(edit, id, config)?;
        // This list has been edited, so make it unique.
        all[n * 63 + index].ram_address = 0;

        let mut rom = edit.rom.borrow_mut();
        let mut scfg = config.sideview.find(&el.ids[0])?;
        // Write a pair of empty enemy lists at the beginning.  All occurances
        // of empty lists will refer to these empty lists.
        let rom_addr = scfg
            .enemies
            .set_val(config.sideview.enemy_list_rom as usize);
        rom.write_bytes(rom_addr, &[1, 1])?;
        // Mapping of old enemy list RAM addrs to new RAM addrs.
        let mut dedup = HashMap::<u16, u16>::new();

        let mut length = 2;
        for (i, enemy) in all.iter().enumerate() {
            if i == 63 {
                scfg = config.sideview.find(&el.ids[i / 63])?;
            }
            let i = i % 63;
            let list = enemy.to_bytes();
            if list.len() <= 2 {
                // An empty enemy list, or empty pair will refer back to the
                // empty token list at the beginning.
                rom.write_word(scfg.enemies + i * 2, config.sideview.enemy_list_ram)?;
                info!(
                    "EnemyList: writing {}/{} as {:x?} (empty)",
                    scfg.id, i, config.sideview.enemy_list_ram
                );
                continue;
            }

            if let Some(ram_addr) = dedup.get(&enemy.ram_address) {
                rom.write_word(scfg.enemies + i * 2, *ram_addr)?;
                info!(
                    "EnemyList: writing {}/{} as {:x?} (dedup)",
                    scfg.id, i, ram_addr
                );
            } else {
                let ram_addr = config.sideview.enemy_list_ram + length;
                dedup.insert(enemy.ram_address, ram_addr);
                let rom_addr = scfg
                    .enemies
                    .set_val(config.sideview.enemy_list_rom as usize)
                    + length;
                rom.write_bytes(rom_addr, &list)?;
                rom.write_word(scfg.enemies + i * 2, ram_addr)?;
                info!(
                    "EnemyList: writing {}/{} at {:x?} -> {:x?} ({} bytes)",
                    scfg.id,
                    i,
                    ram_addr,
                    rom_addr,
                    list.len(),
                );
                length += list.len() as u16;

                if length > el.length {
                    return Err(ErrorKind::LengthError(format!(
                        "Enemy list for {} is too long ({} > {})",
                        scfg.id, length, el.length
                    ))
                    .into());
                }
            }
        }
        info!(
            "EnemyList: Packed into {} out of {} bytes",
            length, el.length
        );
        while length < el.length {
            let rom_addr = scfg
                .enemies
                .set_val(config.sideview.enemy_list_rom as usize)
                + length;
            rom.write(rom_addr, 0xFF)?;
            length += 1;
        }
        Ok(())
    }

    fn read_all<'a>(
        &self,
        edit: &Rc<Edit>,
        id: &IdPath,
        config: &'a Config,
    ) -> Result<(&'a config::EnemyList, usize, Vec<EnemyList>)> {
        let mut ret = Vec::new();
        let el = config.sideview.enemy_list_for(id)?;
        let mut n = 0;
        for (i, world) in el.ids.iter().enumerate() {
            if id.at(0) == world.at(0) {
                n = i;
            }
            let scfg = config.sideview.find(world)?;
            for index in 0..scfg.length {
                let area = world.extend(index);
                ret.push(EnemyList::from_rom(edit, &area, config)?);
            }
        }
        Ok((el, n, ret))
    }

    fn list_from_bytes(data: &[u8]) -> Vec<Enemy> {
        let length = data[0] as usize;
        let mut list = Vec::new();
        let mut i = 1;
        while i < length {
            let xy = data[i + 0];
            let y = (xy >> 4) as i32;
            let y = if y == 0 { y + 1 } else { y + 2 };
            let kind = data[i + 1];
            list.push(Enemy {
                x: ((xy & 0x0F) | (kind >> 2) & 0x30) as i32,
                y: y,
                kind: (kind & 0x3F) as usize,
                dialog: Vec::new(),
                condition: None,
            });
            i += 2;
        }
        list
    }

    fn get_text_info<'a>(
        enemy: &Enemy,
        edit: &Rc<Edit>,
        id: &IdPath,
        config: &'a Config,
    ) -> Option<(&'a TextTable, usize, usize)> {
        if enemy.kind < 10 {
            return None;
        }
        let connection = if let Some(c) = edit.overworld_connector(&id) {
            c
        } else {
            return None;
        };
        // This should turn into something like "west_hyrule/town".
        let text_id = idpath!(connection.at(0), id.at(0));
        let town_text = match config.text_table.find(&text_id) {
            Ok(table) => table,
            Err(_) => {
                // Text table not found, probably because we're asking for a
                // table which doesn't exist (e.g. a palace).
                return None;
            }
        };
        if let Some(town_code) = config
            .overworld
            .town_code(connection.usize_last().expect("get_text_info"))
        {
            let townsperson = enemy.kind - 10;
            Some((town_text, townsperson, town_code))
        } else {
            None
        }
    }

    fn read_text_one(
        enemy: &mut Enemy,
        edit: &Rc<Edit>,
        id: &IdPath,
        config: &Config,
    ) -> Result<()> {
        if let Some((town_text, townsperson, town_code)) =
            EnemyList::get_text_info(enemy, edit, id, config)
        {
            let rom = edit.rom.borrow();
            let i = townsperson * 4 + town_code;
            for index in town_text.index.iter() {
                if i < index.length {
                    let dialog = rom.read(index.address + i)?;
                    enemy.dialog.push(town_text.id.extend(dialog));
                }
            }
            if townsperson >= 9 && townsperson < 13 {
                let i = (townsperson - 9) * 8 + town_text.offset * 4 + town_code;
                enemy.condition = Some(rom.read(config.text_table.dialog_conditions + i)?);
            }
        }
        Ok(())
    }

    fn read_text(&mut self, edit: &Rc<Edit>, id: &IdPath, config: &Config) -> Result<()> {
        for list in self.data.iter_mut() {
            for enemy in list.iter_mut() {
                EnemyList::read_text_one(enemy, edit, id, config)?
            }
        }
        Ok(())
    }

    fn write_text_one(enemy: &Enemy, edit: &Rc<Edit>, id: &IdPath, config: &Config) -> Result<()> {
        if let Some((town_text, townsperson, town_code)) =
            EnemyList::get_text_info(enemy, edit, id, config)
        {
            let mut rom = edit.rom.borrow_mut();
            let i = townsperson * 4 + town_code;
            for (index, dialog) in town_text.index.iter().zip(enemy.dialog.iter()) {
                if i < index.length {
                    if !town_text.id.prefix(dialog) {
                        return Err(ErrorKind::IdPathError(format!(
                            "Expected prefix of {} but saw {}",
                            town_text.id, dialog
                        ))
                        .into());
                    }
                    rom.write(index.address + i, dialog.usize_last()? as u8)?;
                }
            }
            if townsperson >= 9 && townsperson < 13 {
                if let Some(cond) = enemy.condition {
                    let i = (townsperson - 9) * 8 + town_text.offset * 4 + town_code;
                    rom.write(config.text_table.dialog_conditions + i, cond)?;
                }
            }
        }
        Ok(())
    }

    fn write_text(&self, edit: &Rc<Edit>, id: &IdPath, config: &Config) -> Result<()> {
        for list in self.data.iter() {
            for enemy in list.iter() {
                EnemyList::write_text_one(enemy, edit, id, config)?
            }
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        for data in self.data.iter() {
            ret.push(data.len() as u8 * 2 + 1);
            for enemy in data.iter() {
                let y = if enemy.y < 2 { 0 } else { enemy.y - 2 };
                ret.push((y << 4) as u8 | (enemy.x as u8 & 0x0F));
                ret.push((enemy.x as u8 & 0x30) << 2 | (enemy.kind as u8));
            }
        }
        ret
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub dest_map: usize,
    pub entry: usize,
    pub point_target_back: Option<usize>,
}

impl From<u8> for Connection {
    fn from(a: u8) -> Connection {
        Connection {
            dest_map: (a >> 2) as usize,
            entry: (a & 3) as usize,
            point_target_back: None,
        }
    }
}

impl From<&Connection> for u8 {
    fn from(a: &Connection) -> u8 {
        (a.dest_map << 2) as u8 | (a.entry & 3) as u8
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sideview {
    pub id: IdPath,
    pub map: Map,
    pub enemy: EnemyList,
    pub connection: Vec<Connection>,
    pub door: Vec<Connection>,
    pub availability: Vec<bool>,
}

impl Sideview {
    pub fn new(id: IdPath) -> Self {
        Self {
            id: id,
            ..Default::default()
        }
    }

    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if let Some(id) = id {
            Ok(Box::new(Sideview::new(IdPath::from(id))))
        } else {
            Err(ErrorKind::IdPathError("id required".to_string()).into())
        }
    }

    pub fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut sideview = Sideview::default();
        sideview.id = id;
        sideview.unpack(edit)?;
        Ok(sideview)
    }

    pub fn background_layer_from_rom(&self, edit: &Rc<Edit>) -> Option<Self> {
        let config = Config::get(&edit.config()).unwrap();
        let scfg = config.sideview.find(&self.id).unwrap();
        if self.map.background_map != 0 {
            if let Some(bg_id) = &scfg.background_id {
                let bg_id = bg_id.extend(self.map.background_map - 1);
                match Sideview::from_rom(edit, bg_id) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        error!("Background map error: {:?}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn enemy_group_id(&self) -> IdPath {
        idpath!(self.id.at(0), "enemy")
    }
}

#[typetag::serde]
impl RomData for Sideview {
    fn name(&self) -> String {
        "Sideview".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        SideviewGui::new(project, commit_index, None)
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let scfg = config.sideview.find(&self.id)?;
        let index = self.id.usize_last()?;
        if index >= scfg.length {
            return Err(ErrorKind::IndexError(index).into());
        }
        let rom = edit.rom.borrow();

        // Map commands list
        let addr = rom.read_pointer(scfg.address + index * 2)?;
        let length = rom.read(addr)? as usize;
        debug!(
            "Sidview::unpack: reading {} from ROM @ {:x?} for {} bytes",
            self.id, addr, length
        );
        if addr.raw() == 0 || addr.raw() == 0xFFFF {
            return Err(ErrorKind::NotFound(format!(
                "Sideview::unpack: {}: map pointer {:x?} is invalid.",
                self.id, addr
            ))
            .into());
        }
        let length = if length < 4 { 4 } else { length };
        self.map = Map::from_bytes(rom.read_bytes(addr, length)?);

        self.connection.clear();
        self.door.clear();
        if !scfg.background_layer {
            self.enemy = EnemyList::from_rom(edit, &self.id, &config)?;

            if index < scfg.max_connectable_index {
                let table = rom.read_bytes(scfg.connections + index * 4, 4)?;
                self.connection.push(Connection::from(table[0]));
                self.connection.push(Connection::from(table[1]));
                self.connection.push(Connection::from(table[2]));
                self.connection.push(Connection::from(table[3]));
            }

            if index < scfg.max_door_index {
                let table = rom.read_bytes(scfg.doors + index * 4, 4)?;
                self.door.push(Connection::from(table[0]));
                self.door.push(Connection::from(table[1]));
                self.door.push(Connection::from(table[2]));
                self.door.push(Connection::from(table[3]));
            }

            let a = rom.read(scfg.availability + index / 2)?;
            let a = if index & 1 == 0 { a >> 4 } else { a & 0xF };
            self.availability = vec![a & 8 == 8, a & 4 == 4, a & 2 == 2, a & 1 == 1];
        }

        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let scfg = config.sideview.find(&self.id)?;
        let index = self.id.usize_last()?;
        if index >= scfg.length {
            return Err(ErrorKind::IndexError(index).into());
        }

        // Enemy Lists.  Doing this first because the enemy list functions
        // want to borrow 'rom', so take care of it before our own borrow.
        self.enemy.write(edit, &self.id, &config)?;

        let mut rom = edit.rom.borrow_mut();
        let mut memory = edit.memory.borrow_mut();

        // Map commands list
        let addr = rom.read_pointer(scfg.address + index * 2)?;
        let length = rom.read(addr)? as u16;
        memory.free(addr, length);
        let map_bytes = self.map.to_bytes();
        let addr = memory.alloc_near(addr, map_bytes.len() as u16)?;
        rom.write_bytes(addr, &map_bytes)?;
        rom.write_pointer(scfg.address + index * 2, addr)?;

        // Room connections
        if index < scfg.max_connectable_index {
            if self.connection.len() != 4 {
                return Err(ErrorKind::LengthError(
                    "Room connection table must be exactly 4 entries".to_string(),
                )
                .into());
            }
            for (i, c) in self.connection.iter().enumerate() {
                rom.write(scfg.connections + index * 4 + i, u8::from(c))?;
                if let Some(target) = c.point_target_back {
                    let target_conn = Connection {
                        dest_map: index,
                        entry: target,
                        point_target_back: None,
                    };
                    rom.write(
                        scfg.connections + c.dest_map * 4 + c.entry,
                        u8::from(&target_conn),
                    )?;
                }
            }
        }

        // Room doors
        if index < scfg.max_door_index {
            if self.door.len() != 4 {
                return Err(ErrorKind::LengthError(
                    "Room door table must be exactly 4 entries".to_string(),
                )
                .into());
            }
            for (i, c) in self.door.iter().enumerate() {
                rom.write(scfg.doors + index * 4 + i, u8::from(c))?;
                if let Some(target) = c.point_target_back {
                    let target_conn = Connection {
                        dest_map: index,
                        entry: target,
                        point_target_back: None,
                    };
                    rom.write(
                        scfg.connections + c.dest_map * 4 + c.entry,
                        u8::from(&target_conn),
                    )?;
                }
            }
        }

        if self.availability.len() == 4 {
            let avail = if self.availability[0] { 8 } else { 0 }
                | if self.availability[1] { 4 } else { 0 }
                | if self.availability[2] { 2 } else { 0 }
                | if self.availability[3] { 1 } else { 0 };

            let a = rom.read(scfg.availability + index / 2)?;
            let a = if index & 1 == 0 {
                (a & 0x0F) | (avail << 4)
            } else {
                (a & 0xF0) | (avail << 0)
            };
            rom.write(scfg.availability + index / 2, a)?;
        }
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

#[derive(Clone)]
pub struct Decompressor {
    pub layers: [[[u8; 64]; 13]; 3],
    pub data: [[u8; 64]; 13],
    pub item: [[u8; 64]; 13],
    pub bgtile: u8,
    pub layer: usize,
}

impl Decompressor {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 13;

    pub fn new() -> Self {
        Decompressor {
            layers: [[[0; 64]; 13]; 3],
            data: [[0; 64]; 13],
            item: [[0xFF; 64]; 13],
            bgtile: 0,
            layer: 0,
        }
    }

    pub fn decompress(
        &mut self,
        sideview: &Sideview,
        bg_map: Option<&Sideview>,
        config: &config::Config,
        object: &ObjectConfig,
    ) {
        let map = &sideview.map;
        let mut xcursor = 0;
        let mut floor = map.floor as usize;
        let mut ceiling = map.ceiling;
        let svcfg = config.find(&sideview.id).unwrap();
        let background = &svcfg.background[map.tileset as usize];
        let width = Decompressor::WIDTH as i32;

        info!("Render {}:", sideview.id);
        info!(
            "ObjSet={} Width={} Grass={} Bushes={}",
            map.objset, map.width, map.grass, map.bushes
        );
        info!(
            "Ceiling={} Floor={} Tileset={}",
            map.ceiling, map.floor, map.tileset
        );
        info!(
            "SprPal={} BgPal={} BgMap={}",
            map.sprite_palette, map.background_palette, map.background_map
        );

        self.bgtile = background.background;
        self.clear();
        if map.background_map != 0 {
            if let Some(bg_map) = bg_map {
                self.decompress(bg_map, None, config, object);
                self.layer += 1;
            } else {
                error!(
                    "Map {} has background map, but no bg_map supplied.",
                    sideview.id
                );
            }
        }

        let mut data = map.data.clone();
        if !sideview.map.cursor_moves_left {
            Map::sort_data(&mut data);
        }
        for command in data.iter() {
            let mut extra = false;
            if command.y == 13 {
                while xcursor < command.x && xcursor < width {
                    self.draw_floor(xcursor, floor, ceiling, map, background);
                    xcursor += 1;
                }
                floor = command.param as usize & 0x0F;
                ceiling = command.param & 0x80 == 0;
                info!(
                    "Render NewFloor @ y={:02} x={:02}: floor={} ceiling={}",
                    command.y, command.x, floor, ceiling
                );
                continue;
            } else if command.y == 14 {
                // Y Position 14 means "X-Skip".  The Map reader will already
                // have adjusted the X coordinate from screen number to X position.
                while xcursor < command.x && xcursor < width {
                    self.draw_floor(xcursor, floor, ceiling, map, background);
                    xcursor += 1;
                }
                info!(
                    "Render X-Skip   @ y={:02} x={:02}: to_screen={}",
                    command.y,
                    command.x,
                    command.x / 16
                );
                continue;
            } else if command.y == 15 {
                extra = true;
            }

            while xcursor < command.x && xcursor < width {
                self.draw_floor(xcursor, floor, ceiling, map, background);
                xcursor += 1;
            }

            // Cursor might move left.
            xcursor = command.x;
            self.draw_floor(xcursor, floor, ceiling, map, background);

            let obj_kind = if extra {
                ObjectKind::Extra
            } else {
                if command.kind < 0x10 {
                    ObjectKind::Small
                } else {
                    ObjectKind::Objset(map.objset)
                }
            };

            info!(
                "Render Object   @ y={:02} x={:02}: {:?}/{:?}/{:02x}",
                command.y, command.x, svcfg.kind, obj_kind, command.kind
            );

            if let Some(obj) = object.find(command.kind as u8, &svcfg.kind, &obj_kind) {
                match &obj.render {
                    Renderer::Grid => self.draw_grid(xcursor, command.y, command.param, obj),
                    Renderer::Horizontal => {
                        self.draw_horizontal(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Vertical => {
                        self.draw_vertical(xcursor, command.y, command.param, obj)
                    }
                    Renderer::TopUnique => {
                        self.draw_top_unique(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Building => {
                        self.draw_building(xcursor, command.y, command.param, obj)
                    }
                    Renderer::Window => self.draw_window(xcursor, command.y, command.param, obj),
                    Renderer::Item => self.draw_item(xcursor, command.y, command.param, obj),
                    _ => {
                        error!(
                            "No Renderer for {:?}: {:?}",
                            sideview.id.to_string(),
                            obj.render
                        );
                    }
                }
            } else {
                error!(
                    "Cannot render {:?}: {:?}/{:?}/{:02x}",
                    sideview.id.to_string(),
                    svcfg.kind,
                    obj_kind,
                    command.kind
                );
            }
        }
        // Finish rendering to end of room.
        while xcursor < width {
            self.draw_floor(xcursor, floor, ceiling, map, background);
            xcursor += 1;
        }
        self.collapse_layers();
    }

    fn clear(&mut self) {
        for z in 0..3 {
            for y in 0..Decompressor::HEIGHT {
                for x in 0..Decompressor::WIDTH {
                    self.layers[z][y][x] = if z == 0 { self.bgtile } else { 0 };
                    self.item[y][x] = 0xFF;
                }
            }
        }
        // Layer 0 is the background tile.
        // Layers 1 & 2 are map data.
        self.layer = 1;
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        if x < Decompressor::WIDTH && y < Decompressor::HEIGHT {
            self.layers[self.layer][y][x] = val;
        }
    }

    fn set_if_bg(&mut self, x: usize, y: usize, val: u8) {
        if x < Decompressor::WIDTH && y < Decompressor::HEIGHT {
            if self.layers[self.layer][y][x] == 0 {
                self.layers[self.layer][y][x] = val;
            }
        }
    }

    fn collapse_layers(&mut self) {
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                for z in 0..3 {
                    if self.layers[z][y][x] != 0 {
                        self.data[y][x] = self.layers[z][y][x];
                    }
                }
            }
        }
    }

    fn draw_floor(
        &mut self,
        x: i32,
        floor: usize,
        ceiling: bool,
        map: &Map,
        background: &config::Background,
    ) {
        let x = x as usize;
        if map.grass {
            self.set_if_bg(x, 10, background.alternate);
        }
        if map.bushes {
            self.set_if_bg(x, 9, background.alternate);
        }

        if floor < 8 {
            let fy = Decompressor::HEIGHT - floor - 2;
            self.set_if_bg(x, fy, background.floor[0]);
            for y in (fy + 1)..Decompressor::HEIGHT {
                self.set_if_bg(x, y, background.floor[1]);
            }
            if ceiling {
                self.set_if_bg(x, 0, background.ceiling[1]);
            }
        } else if floor < 15 {
            if ceiling {
                // Ceiling height is "floor - 6".  Minus 1 more for inclusive range.
                let cy = (floor - 6) - 1;
                for y in 0..cy {
                    self.set_if_bg(x, y, background.ceiling[0]);
                }
                self.set_if_bg(x, cy, background.ceiling[1]);
            }
            self.set_if_bg(x, 11, background.floor[0]);
            self.set_if_bg(x, 12, background.floor[1]);
        } else {
            for y in 0..Decompressor::HEIGHT {
                self.set_if_bg(x, y, background.floor[1]);
            }
        }
    }

    fn calculate_y(y0: i32, param: i32, object: &Object) -> usize {
        if let Some(top) = object.fixed_y_minus_param {
            top - (param as usize + 1)
        } else {
            object.fixed_y.unwrap_or(y0 as usize)
        }
    }

    fn draw_grid(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        for y in 0..object.height {
            for x in 0..object.width {
                let metatile = object.metatile[y * object.width + x];
                if metatile != 0 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    fn draw_horizontal(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..object.height {
            for x in 0..(object.width * param) {
                self.set(
                    x0 + x,
                    y0 + y,
                    object.metatile[y * object.width + (x % object.width)],
                );
            }
        }
    }

    fn draw_vertical(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..(object.height * param) {
            for x in 0..object.width {
                self.set(
                    x0 + x,
                    y0 + y,
                    object.metatile[(y % object.height) * object.width + x],
                );
            }
        }
    }

    fn draw_top_unique(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let height = param as usize + 1;
        for y in 0..height {
            for x in 0..object.width {
                if y == 0 {
                    self.set(x0 + x, y0 + y, object.metatile[0 * object.width + x]);
                } else {
                    self.set(x0 + x, y0 + y, object.metatile[1 * object.width + x]);
                }
            }
        }
    }

    fn draw_item(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        if x0 < Decompressor::WIDTH && y0 < Decompressor::HEIGHT {
            self.item[y0][x0] = if object.id == 15 {
                // ID 15 is "collectable object"
                param as u8
            } else {
                object.metatile[0]
            };
        }
    }

    fn draw_building(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let width = param as usize + 1;
        for y in 0..Decompressor::HEIGHT {
            for x in 0..width {
                let metatile = if x < width - 1 {
                    object.metatile[0]
                } else {
                    object.metatile[1]
                };
                if metatile != 0 && y0 + y < 11 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    fn draw_window(&mut self, x0: i32, y0: i32, param: i32, object: &Object) {
        let x0 = x0 as usize;
        let y0 = Decompressor::calculate_y(y0, param, object);
        let param = param as usize + 1;
        for y in 0..(object.height * param) {
            for x in 0..object.width {
                // Windows stop rendering at tile y-coordinate 10.
                let metatile = object.metatile[(y % object.height) * object.width + x];
                if metatile != 0 && y0 + y < 10 {
                    self.set(x0 + x, y0 + y, metatile);
                }
            }
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for y in 0..Decompressor::HEIGHT {
            let mut row = String::new();
            for x in 0..Decompressor::WIDTH {
                let mut ch = self.data[y][x];
                if ch == self.bgtile {
                    ch = 0x20;
                } else {
                    ch &= 0x7F;
                    if ch < 0x20 {
                        ch |= 0x20;
                    }
                }
                row.push(ch as char);
            }
            result.push(row);
        }
        result
    }
}
