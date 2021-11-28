use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString, MouseButton, TableColumnFlags, TableFlags};

use crate::errors::*;
use crate::gui::fa;
use crate::gui::util::tooltip;
use crate::gui::util::KeyAction;
use crate::gui::util::{DragHelper, SelectBox};
use crate::gui::zelda2::emulate;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::{Selector, Visibility};
use crate::idpath;
use crate::nes::Address;
use crate::nes::IdPath;
use crate::nes::MemoryAccess;
use crate::util::clamp;
use crate::util::undo::UndoStack;
use crate::zelda2::config::Config;
use crate::zelda2::objects::config::{ObjectArea, ObjectKind};
use crate::zelda2::overworld::Connector;
use crate::zelda2::project::{Edit, EditAction, Project, RomData};
use crate::zelda2::sideview::{Decompressor, Enemy, MapCommand, Sideview};
use crate::zelda2::text_table::TextTable;

pub struct SideviewGui {
    visible: Visibility,
    changed: bool,
    is_new: bool,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    ids: Vec<IdPath>,
    background: TileCache,
    item: TileCache,
    enemy: TileCache,
    selector: Selector,
    scale: f32,
    sideview: Sideview,
    enemy_list: usize,
    undo: UndoStack<Sideview>,
    selectbox: SelectBox,
    decomp: Decompressor,
    drag_helper: DragHelper,
    objects: Vec<(ImString, u8)>,
    objects_map: HashMap<usize, usize>,
    extras: Vec<(ImString, u8)>,
    extras_map: HashMap<usize, usize>,
    items: Vec<(ImString, u8)>,
    items_map: HashMap<usize, usize>,
    enemies: Vec<(ImString, u8)>,
    enemies_map: HashMap<usize, usize>,
    connections: Vec<ImString>,
    text_table: TextTable,
    error: ErrorDialog,
    town_code: [u32; 4],
}

// This bit of silliness is to return constant ImStr's depedning on whether
// the caller is in a popup or not.  The non-popup strings are turned into
// ImGui IDs because we're displaying the widget in a table with a table
// header with the relevant column labels.
macro_rules! str_id {
    ($popup:expr, $label:literal) => {
        if $popup == false {
            const __INPUT: &str = concat!("##", $label, "\0");
            const RESULT: &'static ImStr = unsafe {
                core::mem::transmute::<&'static [u8], &'static ImStr>(__INPUT.as_bytes())
            };
            RESULT
        } else {
            im_str!($label)
        }
    };
}

impl SideviewGui {
    pub fn new(
        project: &Project,
        edit: Option<Rc<Edit>>,
        which: Option<IdPath>,
    ) -> Result<Box<dyn Gui>> {
        if let Some(edit) = &edit {
            let e = edit.edit.borrow();
            if !e.as_any().is::<Sideview>() {
                return Err(ErrorKind::RomDataError(format!(
                    "Expected edit '{}' to contain a Sideview edit",
                    edit.label()
                ))
                .into());
            }
        }
        let is_new = edit.is_none();
        let edit = edit.unwrap_or_else(|| {
            project
                .create_edit("Sideview", Some("west_hyrule/0"))
                .unwrap()
        });
        let config = Config::get(&edit.config())?;

        let mut sideview = if is_new {
            Sideview::default()
        } else {
            let edit = edit.edit.borrow();
            let obj = edit.as_any().downcast_ref::<Sideview>().unwrap();
            obj.clone()
        };

        let mut names = Vec::new();
        let mut ids = Vec::new();
        let mut selected = usize::MAX;
        let mut default_select = 0;
        let which = which.unwrap_or(IdPath::from("west_hyrule/0"));
        for group in config.sideview.group.iter() {
            for i in 0..group.length {
                let name = if let Some(pet_name) = group.pet_names.get(&i) {
                    format!("{:02}: {} {} ({})", i, group.name, i, pet_name)
                } else {
                    format!("{:02}: {} {}", i, group.name, i)
                };
                let name = ImString::from(name);
                names.push(name);
                let path = idpath!(group.id, i);
                if sideview.id == path {
                    selected = ids.len();
                }
                if path == which {
                    default_select = ids.len();
                }
                ids.push(path);
            }
        }

        if selected == usize::MAX {
            selected = default_select;
            sideview.id = ids[selected].clone();
            sideview.unpack(&edit)?;
        }

        let mut decomp = Decompressor::new();
        decomp.decompress(
            &sideview,
            sideview.background_layer_from_rom(&edit).as_ref(),
            &config.sideview,
            &config.objects,
        );

        let background = TileCache::new(&edit, Schema::None);
        let item = TileCache::new(&edit, Schema::None);
        let enemy = TileCache::new(&edit, Schema::None);
        let mut undo = UndoStack::new(1000);
        undo.push(sideview.clone());
        let mut ret = Box::new(SideviewGui {
            visible: Visibility::Visible,
            changed: false,
            is_new: is_new,
            edit: edit,
            names: names,
            ids: ids,
            background: background,
            item: item,
            enemy: enemy,
            selector: Selector::new(selected),
            scale: 1.0,
            sideview: sideview,
            enemy_list: 0,
            undo: undo,
            selectbox: SelectBox::default(),
            decomp: decomp,
            drag_helper: DragHelper::default(),
            objects: Vec::new(),
            objects_map: HashMap::new(),
            extras: Vec::new(),
            extras_map: HashMap::new(),
            items: Vec::new(),
            items_map: HashMap::new(),
            enemies: Vec::new(),
            enemies_map: HashMap::new(),
            connections: Vec::new(),
            text_table: TextTable::default(),
            error: ErrorDialog::default(),
            town_code: [0u32; 4],
        });
        ret.list_object_names()?;
        ret.reset_caches();
        Ok(ret)
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let romdata = Box::new(self.sideview.clone());
        let config = Config::get(&self.edit.config())?;
        let group = config.sideview.find(&self.sideview.id)?;
        let area = self.sideview.id.usize_last()?;
        if self.edit.label().is_empty() {
            let suffix = if let Some(pet_name) = group.pet_names.get(&area) {
                format!("{} {} ({})", group.name, area, pet_name)
            } else {
                format!("{} {}", group.name, area)
            };
            self.edit.set_label_suffix(&suffix);
        }

        project.commit(&self.edit, romdata)?;
        self.is_new = false;
        Ok(())
    }

    pub fn reset_caches(&mut self) {
        let config = Config::get(&self.edit.config()).expect("reset_caches");
        self.background.reset(Schema::MetaTile(
            self.edit.config().clone(),
            self.sideview.id.clone(),
            self.sideview.map.background_palette,
        ));
        self.item.reset(Schema::Item(
            self.edit.config().clone(),
            self.sideview.id.clone(),
        ));
        self.enemy.reset(Schema::Enemy(
            self.edit.config().clone(),
            self.sideview.enemy_group_id(),
            self.sideview.map.sprite_palette,
        ));
        // Since we're just looking for palette info, we just get the overworld
        // connector for screen 0 of this room.
        if let Some(id) = self.edit.overworld_connector(&self.sideview.id.extend(0)) {
            let conn = Connector::from_rom(&self.edit, id).expect("reset_caches");
            if let Some(palace) = conn.palace {
                let chr = Address::Chr(palace.chr_bank as isize, 0);
                let pal = match config.palette.find_palace(
                    &self.sideview.id,
                    palace.palette as usize + 1,
                    self.sideview.map.background_palette == 0,
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Palace palette lookup: {:?}", e);
                        config
                            .palette
                            .find_palace(
                                &self.sideview.id,
                                1,
                                self.sideview.map.background_palette == 0,
                            )
                            .unwrap()
                    }
                };
                self.background.set_chr_override(Some(chr.add_bank(1)));
                self.item.set_chr_override(Some(chr));
                self.enemy.set_chr_override(Some(chr));
                if self.sideview.id.at(0) != "great_palace" {
                    self.background.set_pal_override(Some(pal.address));
                }
            }
        }
        self.text_table = TextTable::from_rom(&self.edit).expect("reset_caches");
        for i in 0..4 {
            if let Some(id) = self.edit.overworld_connector(&self.sideview.id.extend(i)) {
                self.town_code[i] = config
                    .overworld
                    .town_code(id.usize_last().expect("reset_caches"))
                    .unwrap_or(0) as u32;
            } else {
                self.town_code[i] = 0u32;
            }
        }
        info!("town_code = {:?}", self.town_code);
    }

    fn list_object_names(&mut self) -> Result<()> {
        let config = Config::get(&self.edit.config())?;
        let scfg = config.sideview.find(&self.sideview.id)?;

        self.objects.clear();
        self.objects_map.clear();
        self.extras.clear();
        self.extras_map.clear();
        self.items.clear();
        self.items_map.clear();
        self.enemies.clear();
        self.enemies_map.clear();
        self.connections.clear();

        for i in 0..31 {
            let id = if i < 0x10 { i } else { 0x10 + ((i & 0xF) << 4) };
            self.objects
                .push((im_str!("{:02x}: Undefined Object", id), id));
            self.objects_map.insert(id as usize, i as usize);
            debug!("init {:02x} -> {}", id, i);

            if scfg.kind == ObjectArea::Town {
                // Hack: only Towns have extra objects in the 00..0F range.
                // All other extra objects in the game are in the 10..F0 range.
                self.extras
                    .push((im_str!("{:02x}: Undefined Extra", id), id));
                self.extras_map.insert(id as usize, i as usize);
            }
        }
        if scfg.kind != ObjectArea::Town {
            for i in 0..15 {
                let id = 0x10 + (i << 4);
                self.extras
                    .push((im_str!("{:02x}: Undefined Extra", id), id));
                self.extras_map.insert(id as usize, i as usize);
            }
        }

        for obj in config.objects.list(&scfg.kind, &ObjectKind::Small).iter() {
            self.objects[obj.id as usize] = (im_str!("{:02x}: {}", obj.id, obj.name), obj.id);
        }
        for obj in config
            .objects
            .list(&scfg.kind, &ObjectKind::Objset(self.sideview.map.objset))
            .iter()
        {
            let index = 0x10 + (obj.id >> 4) - 1;
            self.objects[index as usize] = (im_str!("{:02x}: {}", obj.id, obj.name), obj.id);
        }
        for obj in config.objects.list(&scfg.kind, &ObjectKind::Extra).iter() {
            let index = if scfg.kind == ObjectArea::Town {
                if obj.id < 0x10 {
                    obj.id
                } else {
                    0x10 + (obj.id >> 4) - 1
                }
            } else {
                (obj.id >> 4) - 1
            };
            self.extras[index as usize] = (im_str!("{:02x}: {}", obj.id, obj.name), obj.id);
        }
        for (i, obj) in config.items.item.iter().enumerate() {
            self.items
                .push((im_str!("{:02x}: {}", obj.offset, obj.name), obj.offset));
            self.items_map.insert(obj.offset as usize, i);
        }
        if !scfg.background_layer {
            // No enemies for background layers.
            let enemy_group = config.enemy.find_group(&self.sideview.enemy_group_id())?;
            for (i, obj) in enemy_group.enemy.iter().enumerate() {
                self.enemies
                    .push((im_str!("{:02x}: {}", obj.offset, obj.name), obj.offset));
                self.enemies_map.insert(obj.offset as usize, i);
            }
        }

        let world = config.sideview.find(&self.sideview.id)?;
        for i in 0..world.length {
            let name = if let Some(pet_name) = world.pet_names.get(&i) {
                im_str!("{:02}: {} {} ({})", i, world.name, i, pet_name)
            } else {
                im_str!("{:02}: {} {}", i, world.name, i)
            };
            self.connections.push(name);
        }
        self.connections.push(ImString::new("Outside"));

        Ok(())
    }

    fn decompress(&mut self, config: &Config) {
        self.decomp.decompress(
            &self.sideview,
            self.sideview.background_layer_from_rom(&self.edit).as_ref(),
            &config.sideview,
            &config.objects,
        );
    }

    fn draw_selectbox(&mut self, changed: bool, scr_origin: [f32; 2], ui: &imgui::Ui) {
        let scale = self.scale * 16.0;
        let io = ui.io();
        let mx = io.mouse_pos[0] - scr_origin[0];
        let my = io.mouse_pos[1] - scr_origin[1];
        let tx = if mx >= 0.0 { (mx / scale) as isize } else { -1 };
        let ty = if my >= 0.0 { (my / scale) as isize } else { -1 };
        let modifier = io.key_ctrl | io.key_shift | io.key_alt | io.key_super;

        let wmin = ui.window_content_region_min();
        let wmax = ui.window_content_region_max();
        let bounds = [
            wmax[0] - wmin[0] + ui.scroll_x(),
            wmax[1] - wmin[1] + ui.scroll_y(),
        ];

        if !changed
            && tx >= 0
            && tx < Decompressor::WIDTH as isize
            && ty >= 0
            && ty < Decompressor::HEIGHT as isize + 1
            && ui.is_window_hovered()
            && mx < bounds[0]
            && my < bounds[1]
        {
            if (!modifier && ui.is_mouse_clicked(MouseButton::Right))
                || (io.key_shift && ui.is_mouse_clicked(MouseButton::Left))
            {
                self.selectbox.init(tx, ty);
            }
            if (!modifier && ui.is_mouse_dragging(MouseButton::Right))
                || (io.key_shift && ui.is_mouse_dragging(MouseButton::Left))
            {
                self.selectbox.drag(
                    clamp(tx, 0, Decompressor::WIDTH as isize - 1),
                    clamp(ty, 0, Decompressor::HEIGHT as isize),
                );
            }
        }

        if self.selectbox.valid() {
            let norm = self.selectbox.normalized();
            let x0 = scr_origin[0] + norm.x0 as f32 * scale;
            let y0 = scr_origin[1] + norm.y0 as f32 * scale;
            let x1 = scr_origin[0] + norm.x1 as f32 * scale + scale;
            let y1 = scr_origin[1] + norm.y1 as f32 * scale + scale;
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect([x0, y0], [x1, y1], [1.0, 1.0, 1.0, 0.25])
                .filled(true)
                .build();
            draw_list
                .add_rect([x0, y0], [x1, y1], [0.0, 0.0, 0.0, 1.0])
                .build();
        }
    }

    pub fn copy_to_clipboard(&mut self, cut: bool, ui: &imgui::Ui) {
        if self.selectbox.valid() {
            let mut norm = self.selectbox.normalized();
            if norm.y1 == Decompressor::HEIGHT as isize {
                norm.y1 = 15;
            }
            let mut sv = Sideview::default();
            let mut i = 0;
            while i < self.sideview.map.data.len() {
                let x = self.sideview.map.data[i].x as isize;
                let y = self.sideview.map.data[i].y as isize;
                if x >= norm.x0 && x <= norm.x1 && y >= norm.y0 && y < norm.y1 {
                    if cut {
                        let mut command = self.sideview.map.data.remove(i);
                        command.x -= norm.x0 as i32;
                        command.y -= norm.y0 as i32;
                        sv.map.data.push(command);
                        continue;
                    } else {
                        let mut command = self.sideview.map.data[i].clone();
                        command.x -= norm.x0 as i32;
                        command.y -= norm.y0 as i32;
                        sv.map.data.push(command);
                    }
                }
                i += 1;
            }
            for el in 0..self.sideview.enemy.data.len() {
                sv.enemy.data.push(Vec::new());
                let mut i = 0;
                while i < self.sideview.enemy.data[el].len() {
                    let x = self.sideview.enemy.data[el][i].x as isize;
                    let y = self.sideview.enemy.data[el][i].y as isize;
                    if x >= norm.x0 && x <= norm.x1 && y >= norm.y0 && y < norm.y1 {
                        if cut {
                            let mut enemy = self.sideview.enemy.data[el].remove(i);
                            enemy.x -= norm.x0 as i32;
                            enemy.y -= norm.y0 as i32;
                            sv.enemy.data[el].push(enemy);
                            continue;
                        } else {
                            let mut enemy = self.sideview.enemy.data[el][i].clone();
                            enemy.x -= norm.x0 as i32;
                            enemy.y -= norm.y0 as i32;
                            sv.enemy.data[el].push(enemy);
                        }
                    }
                    i += 1;
                }
            }
            let text = ImString::new(serde_json::to_string_pretty(&sv).expect("copy to clipboard"));
            ui.set_clipboard_text(&text);
        }
    }

    pub fn decode_clipboard(&self, ui: &imgui::Ui) -> Result<Sideview> {
        if let Some(text) = ui.clipboard_text() {
            let sv = serde_json::from_str::<Sideview>(text.to_str())?;
            Ok(sv)
        } else {
            Ok(Sideview::default())
        }
    }

    pub fn paste_from_clipboard(&mut self, ui: &imgui::Ui) {
        if !self.selectbox.valid() {
            return;
        }
        let norm = self.selectbox.normalized();
        let x0 = norm.x0 as i32;
        let y0 = norm.y0 as i32;
        match self.decode_clipboard(ui) {
            Ok(sv) => {
                for c in sv.map.data.iter() {
                    let mut command = c.clone();
                    command.x += x0;
                    command.y += y0;
                    self.sideview.map.data.push(command);
                }
                for (i, elist) in sv.enemy.data.iter().enumerate() {
                    for e in elist.iter() {
                        let mut enemy = e.clone();
                        enemy.x += x0;
                        enemy.y += y0;
                        if i < self.sideview.enemy.data.len() {
                            self.sideview.enemy.data[i].push(enemy);
                        } else {
                            warn!(
                                "Sideview map {} is not an encounter.  Skipping enemy paste.",
                                self.sideview.id
                            );
                        }
                    }
                }
            }
            Err(e) => {
                warn!("SideviewGui: clipboard {:?}", e);
            }
        }
    }

    fn draw_availability_tab(&mut self, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        for (i, mut a) in self.sideview.availability.iter_mut().enumerate() {
            changed |= ui.checkbox(&im_str!("Screen {}", i + 1), &mut a);
        }
        changed
    }

    fn draw_connections_tab(&mut self, _config: &Config, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        let label = [
            im_str!("Screen 1 (Left) "),
            im_str!("Screen 2 (Down) "),
            im_str!("Screen 3 (Up)   "),
            im_str!("Screen 4 (Right)"),
        ];
        let screen = [
            im_str!("Screen 1"),
            im_str!("Screen 2"),
            im_str!("Screen 3"),
            im_str!("Screen 4"),
        ];
        if !self.sideview.connection.is_empty() {
            ui.text("Connection Table:");
            ui.separator();
            for (i, c) in self.sideview.connection.iter_mut().enumerate() {
                let id0 = ui.push_id(0xDD00 | i as i32);
                let width = ui.push_item_width(200.0);
                changed |= imgui::ComboBox::new(label[i]).build_simple(
                    ui,
                    &mut c.dest_map,
                    &self.connections,
                    &|x| Cow::Borrowed(&x),
                );
                width.pop(ui);
                ui.same_line();
                let width = ui.push_item_width(100.0);
                changed |= imgui::ComboBox::new(im_str!("Destination")).build_simple(
                    ui,
                    &mut c.entry,
                    &screen,
                    &|x| Cow::Borrowed(&x),
                );
                width.pop(ui);

                if c.dest_map != 63 && (i == 0 || i == 3) {
                    let mut target = c.point_target_back.is_some();
                    ui.same_line();
                    if ui.checkbox(im_str!("Adjust target connection"), &mut target) {
                        if target {
                            c.point_target_back = Some(3 - i);
                        } else {
                            c.point_target_back = None;
                        }
                    }
                }

                if c.dest_map != 63 && (i == 1 || i == 2) && self.sideview.map.elevator().is_some()
                {
                    let mut target = c.point_target_back.is_some();
                    ui.same_line();
                    if ui.checkbox(im_str!("Adjust target connection"), &mut target) {
                        if target {
                            c.point_target_back =
                                Some(self.sideview.map.elevator().unwrap() as usize / 16);
                        } else {
                            c.point_target_back = None;
                        }
                    }
                }
                id0.pop();
            }
        } else {
            ui.text(im_str!("No connections for {}", self.sideview.id));
        }

        if !self.sideview.door.is_empty() {
            ui.text("\n\nDoor Table:");
            ui.separator();
            for (i, c) in self.sideview.door.iter_mut().enumerate() {
                let id0 = ui.push_id(0xD400 | i as i32);
                let width = ui.push_item_width(200.0);
                changed |= imgui::ComboBox::new(screen[i]).build_simple(
                    ui,
                    &mut c.dest_map,
                    &self.connections,
                    &|x| Cow::Borrowed(&x),
                );
                width.pop(ui);
                ui.same_line();
                let width = ui.push_item_width(100.0);
                changed |= imgui::ComboBox::new(im_str!("Destination")).build_simple(
                    ui,
                    &mut c.entry,
                    &screen,
                    &|x| Cow::Borrowed(&x),
                );
                width.pop(ui);
                if c.dest_map != 63 {
                    let mut target = c.point_target_back.is_some();
                    ui.same_line();
                    if ui.checkbox(im_str!("Adjust target connection"), &mut target) {
                        if target {
                            c.point_target_back = Some(i);
                        } else {
                            c.point_target_back = None;
                        }
                    }
                }
                id0.pop();
            }
        }
        changed
    }

    fn draw_enemies(
        &mut self,
        config: &Config,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
    ) -> bool {
        let mut action = EditAction::None;
        let el = self.enemy_list;
        for index in 0..self.sideview.enemy.data[el].len() {
            action.set(
                self.draw_enemy_entity(el, index, origin, scr_origin, ui)
                    .expect("enemy_entity"),
            );
        }
        self.process_enemy_action(el, action, &config)
    }

    fn draw_enemies_tab(&mut self, el: usize, config: &Config, ui: &imgui::Ui) -> bool {
        let mut action = EditAction::None;

        ui.begin_table_with_flags(
            im_str!("##enemy_list"),
            8,
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        );
        ui.table_setup_column_with_weight(im_str!("New"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(im_str!("Up"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(im_str!("Dn"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(
            im_str!("Y Position"),
            TableColumnFlags::WIDTH_FIXED,
            100.0,
        );
        ui.table_setup_column_with_weight(
            im_str!("X Position"),
            TableColumnFlags::WIDTH_FIXED,
            100.0,
        );
        ui.table_setup_column_with_weight(im_str!("Enemy"), TableColumnFlags::WIDTH_FIXED, 400.0);
        ui.table_setup_column_with_weight(im_str!("Del"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_headers_row();

        for i in 0..self.sideview.enemy.data[el].len() {
            ui.table_next_row();
            action.set(self.draw_enemy_item(el, i, false, ui).expect("enemy_item"));
        }
        ui.end_table();
        if self.sideview.enemy.data[el].len() == 0 {
            if ui.button(&im_str!("{}", fa::ICON_COPY)) {
                action.set(EditAction::NewAt(0));
            }
            tooltip("Insert a new Enemy", ui);
        }
        self.process_enemy_action(el, action, &config)
    }

    fn process_enemy_action(&mut self, el: usize, action: EditAction, _config: &Config) -> bool {
        if action != EditAction::None {
            info!("enemy action = {:?}", action);
        }
        let changed = match action {
            EditAction::None => false,
            EditAction::Swap(i, j) => {
                self.sideview.enemy.data[el].swap(i, j);
                true
            }
            EditAction::NewAt(i) => {
                self.sideview.enemy.data[el].insert(i, Enemy::default());
                true
            }
            EditAction::CopyAt(i) => {
                let item = self.sideview.enemy.data[el][i].clone();
                self.sideview.enemy.data[el].insert(i, item);
                true
            }
            EditAction::Delete(i) => {
                self.sideview.enemy.data[el].remove(i);
                true
            }
            EditAction::Drag => false,
            EditAction::Update => true,
            EditAction::PaletteChanged | EditAction::CacheInvalidate => {
                self.reset_caches();
                true
            }
            _ => {
                info!("map_commands: unhandled edit action {:?}", action);
                false
            }
        };
        changed
    }

    fn draw_enemy_dialog(&mut self, el: usize, index: usize, ui: &imgui::Ui) -> EditAction {
        let mut action = EditAction::None;

        if self.sideview.enemy.data[el][index].dialog.len() != 0 {
            ui.begin_table_with_flags(
                im_str!("##enemy_dialog"),
                2,
                TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
            );
            ui.table_setup_column_with_weight(
                im_str!("Text ID"),
                TableColumnFlags::WIDTH_FIXED,
                100.0,
            );
            ui.table_setup_column_with_weight(
                im_str!("Dialog"),
                TableColumnFlags::WIDTH_FIXED,
                300.0,
            );
            ui.table_headers_row();

            for d in 0..self.sideview.enemy.data[el][index].dialog.len() {
                ui.table_next_row();
                ui.table_next_column();
                let dialog = &mut self.sideview.enemy.data[el][index].dialog[d];
                let mut dlg = dialog.usize_last().expect("draw_enemy_dialog") as i32;
                let width = ui.push_item_width(100.0);
                if ui.input_int(&im_str!("##dialog{}", d), &mut dlg).build() {
                    dlg = clamp(dlg, 0, 255);
                    dialog.set(-1, dlg).unwrap();
                    action.set(EditAction::Update);
                }
                width.pop(ui);
                ui.table_next_column();
                if let Some(textitem) = self.text_table.get(dialog) {
                    ui.text(&textitem.text);
                } else {
                    ui.text(im_str!("No dialog {}", dialog));
                }
            }
            ui.end_table();
        }

        if let Some(condition) = self.sideview.enemy.data[el][index].condition.as_mut() {
            ui.begin_table_with_flags(
                im_str!("##enemy_dialog_cond"),
                9,
                TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
            );
            ui.table_setup_column_with_weight(
                im_str!("Conditions"),
                TableColumnFlags::WIDTH_FIXED,
                100.0,
            );
            ui.table_setup_column_with_weight(im_str!("b7"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b6"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b5"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b4"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b3"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b2"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b1"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_setup_column_with_weight(im_str!("b0"), TableColumnFlags::WIDTH_FIXED, 16.0);
            ui.table_headers_row();

            ui.table_next_row();
            ui.table_next_column();

            for i in 0..8 {
                ui.table_next_column();
                let mask = 1u8 << (7 - i);
                let mut bit = (*condition & mask) != 0;
                if ui.checkbox(&im_str!("##b{}", i), &mut bit) {
                    *condition &= !mask;
                    *condition |= if bit { mask } else { 0 };
                    action.set(EditAction::Update);
                }
            }
            ui.end_table();
        }
        action
    }

    fn draw_enemy_item(
        &mut self,
        el: usize,
        index: usize,
        popup: bool,
        ui: &imgui::Ui,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let id0 = ui.push_id(0xEE00 | index as i32);

        if !popup {
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_COPY)) {
                action.set(EditAction::NewAt(index));
            }
            tooltip("Insert a new Enemy", ui);
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_ARROW_UP)) {
                if index > 0 {
                    action.set(EditAction::Swap(index, index - 1));
                }
            }
            tooltip("Move Up", ui);
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_ARROW_DOWN)) {
                if index < self.sideview.enemy.data[el].len() - 1 {
                    action.set(EditAction::Swap(index, index + 1));
                }
            }
            tooltip("Move Down", ui);
        }

        if !popup {
            ui.table_next_column();
        }
        let width = ui.push_item_width(100.0);
        let y = &mut self.sideview.enemy.data[el][index].y;
        if ui.input_int(str_id!(popup, "Y position"), y).build() {
            *y = clamp(*y, 0, 15);
            action.set(EditAction::Update);
        }
        width.pop(ui);

        if !popup {
            ui.table_next_column();
        }
        let width = ui.push_item_width(100.0);
        {
            let x = &mut self.sideview.enemy.data[el][index].x;
            if ui.input_int(str_id!(popup, "X position"), x).build() {
                *x = clamp(*x, 0, 63);
                action.set(EditAction::Update);
            }
        }
        width.pop(ui);

        if !popup {
            ui.table_next_column();
        }
        let width = ui.push_item_width(400.0);
        let kind = &mut self.sideview.enemy.data[el][index].kind;
        let mut sel = self.enemies_map[kind];
        if imgui::ComboBox::new(str_id!(popup, "Enemy")).build_simple(
            ui,
            &mut sel,
            &self.enemies,
            &|x| Cow::Borrowed(&x.0),
        ) {
            *kind = self.enemies[sel].1 as usize;
            action.set(EditAction::Update);
        }
        width.pop(ui);
        action.set(self.draw_enemy_dialog(el, index, ui));

        if !popup {
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_TRASH)) {
                action.set(EditAction::Delete(index));
            }
            tooltip("Delete", ui);
        }
        id0.pop();
        Ok(action)
    }

    fn draw_enemy_entity(
        &mut self,
        el: usize,
        index: usize,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let draw_list = ui.get_window_draw_list();
        let scale = self.scale * 16.0;
        let id = ui.push_id(0xEE00 | index as i32);

        let ox = self.sideview.enemy.data[el][index].x;
        let oy = self.sideview.enemy.data[el][index].y;
        let x = ox as f32 * scale;
        let y = oy as f32 * scale;

        {
            let screen = (ox >> 4) as usize;
            let image = self
                .enemy
                ._get_cached(
                    (self.town_code[screen] << 8)
                        | (self.sideview.enemy.data[el][index].kind as u32),
                    None,
                    None,
                )
                .expect("draw_enemy_entity");
            image.draw_at([x + origin[0], y + origin[1]], self.scale, ui);
        }
        draw_list
            .add_rect(
                [x + scr_origin[0], y + scr_origin[1]],
                [x + scr_origin[0] + scale, y + scr_origin[1] + scale],
                [1.0, 0.0, 0.0, 1.0],
            )
            .thickness(2.0)
            .build();

        ui.set_cursor_pos([x + origin[0], y + origin[1]]);
        ui.invisible_button(&im_str!("edit"), [scale, scale]);
        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                let mp = ui.io().mouse_pos;
                let mp = [mp[0] - scr_origin[0], mp[1] - scr_origin[1]];
                // Use a constant to make enemy drag and map drags unique.
                self.drag_helper.start(0xEE00 | index);
                self.drag_helper.position(0xEE00 | index, mp);
                let x = clamp((mp[0] / scale) as i32, 0, 64);
                let y = clamp((mp[1] / scale) as i32, 0, 12);

                if x != ox {
                    self.sideview.enemy.data[el][index].x = x;
                    action.set(EditAction::Drag);
                }
                if y != oy {
                    self.sideview.enemy.data[el][index].y = y;
                    action.set(EditAction::Drag);
                }
            }
        } else {
            if let Some(_) = self.drag_helper.finalize(0xEE00 | index) {
                action.set(EditAction::Update);
            }
        }
        if ui.popup_context_item(im_str!("properties")) {
            action.set(
                self.draw_enemy_item(el, index, true, ui)
                    .expect("enemy_list"),
            );
            if ui.button(im_str!("Copy")) {
                action.set(EditAction::CopyAt(index));
            }
            ui.same_line();
            if ui.button(im_str!("Delete")) {
                action.set(EditAction::Delete(index));
            }
            ui.end_popup();
        }
        id.pop();
        Ok(action)
    }

    fn draw_map(
        &mut self,
        config: &Config,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
    ) -> bool {
        let scale = 16.0 * self.scale;

        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let image = self.background.get(self.decomp.data[y][x]);
                let xo = origin[0] + x as f32 * scale;
                let yo = origin[1] + y as f32 * scale;
                image.draw_at([xo, yo], self.scale, ui);
            }
        }
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let item = self.decomp.item[y][x];
                if item != 0xFF {
                    let image = self.item.get(item);
                    let xo = origin[0] + x as f32 * scale;
                    let yo = origin[1] + y as f32 * scale;
                    image.draw_at([xo, yo], self.scale, ui);
                    let aindex = x / 16;
                    if item != 0xee && self.sideview.availability.get(aindex) == Some(&false) {
                        let radius = std::cmp::max(image.width, image.height) as f32 * self.scale
                            / 2.0
                            - 2.0;
                        let draw_list = ui.get_window_draw_list();
                        let xc =
                            scr_origin[0] + (x as u32 * 16 + image.width / 2) as f32 * self.scale;
                        let yc =
                            scr_origin[1] + (y as u32 * 16 + image.height / 2) as f32 * self.scale;
                        draw_list
                            .add_circle([xc, yc], radius, [1.0, 0.0, 0.0, 1.0])
                            .thickness(2.0)
                            .build();
                        draw_list
                            .add_line(
                                [xc - radius, yc - radius],
                                [xc + radius, yc + radius],
                                [1.0, 0.0, 0.0, 1.0],
                            )
                            .thickness(2.0)
                            .build();
                    }
                }
            }
        }

        let mut action = EditAction::None;
        for i in 0..self.sideview.map.data.len() {
            action.set(
                self.draw_map_entity(i, origin, scr_origin, ui)
                    .expect("map_entity"),
            );
        }
        self.process_map_action(action, &config)
    }

    fn draw_map_entity(
        &mut self,
        index: usize,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let draw_list = ui.get_window_draw_list();
        let scale = self.scale * 16.0;
        let id = ui.push_id(index as i32);

        let ox = self.sideview.map.data[index].x;
        let oy = self.sideview.map.data[index].y;
        let x = ox as f32 * scale;
        let y = if oy < 13 {
            oy as f32 * scale
        } else {
            13.0 * scale
        };
        draw_list
            .add_rect(
                [x + scr_origin[0], y + scr_origin[1]],
                [x + scr_origin[0] + scale, y + scr_origin[1] + scale],
                [1.0, 1.0, 1.0, 1.0],
            )
            .thickness(2.0)
            .build();
        if oy == 13 {
            for i in 0..4 {
                let f = i as f32;
                draw_list
                    .add_line(
                        [
                            x + scr_origin[0] + (f + 1.0) * 4.0 * self.scale,
                            y + scr_origin[1] + 8.0 * self.scale,
                        ],
                        [
                            x + scr_origin[0] + f * 4.0 * self.scale,
                            y + scr_origin[1] + 14.0 * self.scale,
                        ],
                        [1.0, 1.0, 1.0, 1.0],
                    )
                    .build();
            }
        } else if oy == 14 {
            draw_list
                .add_triangle(
                    [
                        x + scr_origin[0] + 4.0 * self.scale,
                        y + scr_origin[1] + 4.0 * self.scale,
                    ],
                    [
                        x + scr_origin[0] + 12.0 * self.scale,
                        y + scr_origin[1] + 8.0 * self.scale,
                    ],
                    [
                        x + scr_origin[0] + 4.0 * self.scale,
                        y + scr_origin[1] + 12.0 * self.scale,
                    ],
                    [1.0, 1.0, 1.0, 1.0],
                )
                .filled(true)
                .build();
        }

        ui.set_cursor_pos([x + origin[0], y + origin[1]]);
        ui.invisible_button(&im_str!("edit"), [scale, scale]);
        if ui.is_item_hovered() {
            let delta = ui.io().mouse_wheel as i32;
            if delta != 0 {
                self.sideview.map.data[index].param =
                    clamp(self.sideview.map.data[index].param - delta, 0, 15);
                action.set(EditAction::Update);
            }
        }
        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                let mp = ui.io().mouse_pos;
                let mp = [mp[0] - scr_origin[0], mp[1] - scr_origin[1]];
                self.drag_helper.start(index);
                self.drag_helper.position(index, mp);
                let x = clamp((mp[0] / scale) as i32, 0, 64);
                let y = clamp((mp[1] / scale) as i32, 0, 12);

                if x != ox {
                    self.sideview.map.data[index].x = x;
                    action.set(EditAction::Drag);
                }
                if oy < 13 && y != oy {
                    self.sideview.map.data[index].y = y;
                    action.set(EditAction::Drag);
                }
            }
        } else {
            if let Some(_) = self.drag_helper.finalize(index) {
                action.set(EditAction::Update);
            }
        }
        if ui.popup_context_item(im_str!("properties")) {
            action.set(self.draw_map_command(index, true, ui).expect("map_command"));
            if ui.button(im_str!("Copy")) {
                action.set(EditAction::CopyAt(index));
            }
            ui.same_line();
            if ui.button(im_str!("Delete")) {
                action.set(EditAction::Delete(index));
            }
            ui.end_popup();
        }
        id.pop();
        Ok(action)
    }

    fn draw_map_command_tab(&mut self, config: &Config, ui: &imgui::Ui) -> bool {
        let mut action = EditAction::None;
        action.set(
            self.draw_map_command_header(config, ui)
                .expect("map_command_header"),
        );
        ui.text("\nMap Commands:");

        ui.begin_table_with_flags(
            im_str!("##map_commands"),
            8,
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        );
        ui.table_setup_column_with_weight(im_str!("New"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(im_str!("Up"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(im_str!("Dn"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_setup_column_with_weight(
            im_str!("Y Position"),
            TableColumnFlags::WIDTH_FIXED,
            100.0,
        );
        ui.table_setup_column_with_weight(
            im_str!("X Position"),
            TableColumnFlags::WIDTH_FIXED,
            100.0,
        );
        ui.table_setup_column_with_weight(im_str!("Object"), TableColumnFlags::WIDTH_FIXED, 200.0);
        ui.table_setup_column_with_weight(
            im_str!("Parameter"),
            TableColumnFlags::WIDTH_FIXED,
            200.0,
        );
        ui.table_setup_column_with_weight(im_str!("Del"), TableColumnFlags::WIDTH_FIXED, 20.0);
        ui.table_headers_row();

        for i in 0..self.sideview.map.data.len() {
            ui.table_next_row();
            action.set(self.draw_map_command(i, false, ui).expect("map_command"));
        }
        ui.end_table();
        if self.sideview.map.data.len() == 0 {
            if ui.button(&im_str!("{}", fa::ICON_COPY)) {
                action.set(EditAction::NewAt(0));
            }
            tooltip("Insert a new Map Command", ui);
        }
        self.process_map_action(action, &config)
    }

    fn process_map_action(&mut self, action: EditAction, config: &Config) -> bool {
        if action != EditAction::None {
            info!("map action = {:?}", action);
        }
        let changed = match action {
            EditAction::None => false,
            EditAction::Swap(i, j) => {
                self.sideview.map.data.swap(i, j);
                true
            }
            EditAction::NewAt(i) => {
                self.sideview.map.data.insert(i, MapCommand::default());
                true
            }
            EditAction::CopyAt(i) => {
                let item = self.sideview.map.data[i].clone();
                self.sideview.map.data.insert(i, item);
                true
            }
            EditAction::Delete(i) => {
                self.sideview.map.data.remove(i);
                true
            }
            EditAction::Drag => false,
            EditAction::Update => true,
            EditAction::PaletteChanged | EditAction::CacheInvalidate => {
                self.reset_caches();
                true
            }
            _ => {
                info!("map_commands: unhandled edit action {:?}", action);
                false
            }
        };

        if changed || action == EditAction::Drag {
            self.decompress(config);
        }
        changed
    }

    fn draw_map_command(
        &mut self,
        index: usize,
        popup: bool,
        ui: &imgui::Ui,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let id0 = ui.push_id(index as i32);

        if !popup {
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_COPY)) {
                action.set(EditAction::NewAt(index));
            }
            tooltip("Insert a new Map Command", ui);
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_ARROW_UP)) {
                if index > 0 {
                    action.set(EditAction::Swap(index, index - 1));
                }
            }
            tooltip("Move Up", ui);
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_ARROW_DOWN)) {
                if index < self.sideview.map.data.len() - 1 {
                    action.set(EditAction::Swap(index, index + 1));
                }
            }
            tooltip("Move Down", ui);
        }

        if !popup {
            ui.table_next_column();
        }
        let y = &mut self.sideview.map.data[index].y;
        let (label, tip) = match y {
            13 => (str_id!(popup, "New floor "), "New Floor"),
            14 => (str_id!(popup, "X-skip    "), "X-Skip"),
            15 => (str_id!(popup, "Extra obj "), "Extra Object"),
            _ => (str_id!(popup, "Y position"), "Y Position"),
        };

        let width = ui.push_item_width(100.0);
        if ui.input_int(label, y).build() {
            *y = clamp(*y, 0, 15);
            action.set(EditAction::Update);
        }
        let y = self.sideview.map.data[index].y;

        if !popup {
            tooltip(tip, ui);
            ui.table_next_column();
        }
        {
            let x = &mut self.sideview.map.data[index].x;
            if ui.input_int(str_id!(popup, "X position"), x).build() {
                *x = clamp(*x, 0, 63);
                action.set(EditAction::Update);
            }
        }
        width.pop(ui);

        if !popup {
            ui.table_next_column();
        }
        if y < 13 {
            let width = ui.push_item_width(200.0);
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(sel) = self.objects_map.get(kind) {
                let mut sel = *sel;
                if imgui::ComboBox::new(str_id!(popup, "Object")).build_simple(
                    ui,
                    &mut sel,
                    &self.objects,
                    &|x| Cow::Borrowed(&x.0),
                ) {
                    *kind = self.objects[sel].1 as usize;
                    action.set(EditAction::Update);
                }
            } else {
                ui.text(im_str!("Unknown: Object/{:x?}", kind));
            }
            width.pop(ui);
        } else if y == 15 {
            let width = ui.push_item_width(200.0);
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(sel) = self.extras_map.get(kind) {
                let mut sel = *sel;
                if imgui::ComboBox::new(str_id!(popup, "Object")).build_simple(
                    ui,
                    &mut sel,
                    &self.extras,
                    &|x| Cow::Borrowed(&x.0),
                ) {
                    *kind = self.extras[sel].1 as usize;
                    action.set(EditAction::Update);
                }
            } else {
                ui.text(im_str!("Unknown: Extra/{:x?}", kind));
            }
            width.pop(ui);
        }

        if !popup {
            ui.table_next_column();
        }
        if y != 14 {
            if self.sideview.map.data[index].kind == 0x0f {
                let width = ui.push_item_width(200.0);
                let item = &mut self.sideview.map.data[index].param;
                let mut sel = self.items_map[&(*item as usize)];
                if imgui::ComboBox::new(str_id!(popup, "Item")).build_simple(
                    ui,
                    &mut sel,
                    &self.items,
                    &|x| Cow::Borrowed(&x.0),
                ) {
                    *item = self.items[sel].1 as i32;
                    action.set(EditAction::Update);
                }
                width.pop(ui);
            } else {
                let width = ui.push_item_width(100.0);
                let p = &mut self.sideview.map.data[index].param;
                if ui.input_int(str_id!(popup, "Param"), p).build() {
                    *p = clamp(*p, 0, if y == 13 { 255 } else { 15 });
                    action.set(EditAction::Update);
                }
                width.pop(ui);
            }
        }
        if !popup {
            ui.table_next_column();
            if ui.button(&im_str!("{}", fa::ICON_TRASH)) {
                action.set(EditAction::Delete(index));
            }
            tooltip("Delete", ui);
        }

        id0.pop();
        Ok(action)
    }

    fn draw_map_command_header(&mut self, config: &Config, ui: &imgui::Ui) -> Result<EditAction> {
        let scfg = config.sideview.find(&self.sideview.id)?;
        let index = self.sideview.id.usize_last()?;
        let rom = self.edit.rom.borrow();

        let ptr = scfg.address + index * 2;
        let addr = rom.read_pointer(ptr)?;

        let mut action = EditAction::None;

        ui.text(im_str!("Map pointer at {:x?}", ptr));
        ui.text(im_str!("Map address is {:x?}", addr));

        ui.begin_table_with_flags(
            im_str!("##map_properties"),
            4,
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        );
        ui.table_setup_column_with_weight(
            im_str!("Map Header"),
            TableColumnFlags::WIDTH_FIXED,
            120.0,
        );
        ui.table_setup_column_with_weight(im_str!("##b"), TableColumnFlags::WIDTH_FIXED, 260.0);
        ui.table_setup_column_with_weight(im_str!("##c"), TableColumnFlags::WIDTH_FIXED, 220.0);
        ui.table_setup_column_with_weight(im_str!("##d"), TableColumnFlags::WIDTH_FIXED, 200.0);
        ui.table_headers_row();

        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text("Map Properties");

        ui.table_next_column();
        if ui
            .input_int(im_str!("Width"), &mut self.sideview.map.width)
            .build()
        {
            self.sideview.map.width = clamp(self.sideview.map.width, 1, 4);
            action.set(EditAction::Update);
        }
        ui.table_next_column();
        if ui
            .input_int(im_str!("Object Set"), &mut self.sideview.map.objset)
            .build()
        {
            self.sideview.map.objset = clamp(self.sideview.map.objset, 0, 1);
            action.set(EditAction::CacheInvalidate);
        }
        ui.table_next_column();
        if ui.checkbox(
            im_str!("Cursor moves left"),
            &mut self.sideview.map.cursor_moves_left,
        ) {
            action.set(EditAction::Update);
        }

        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text("Flags");

        ui.table_next_column();
        if ui.checkbox(im_str!("Ceiling"), &mut self.sideview.map.ceiling) {
            action.set(EditAction::Update);
        }
        ui.table_next_column();
        if ui.checkbox(im_str!("Grass"), &mut self.sideview.map.grass) {
            action.set(EditAction::Update);
        }
        ui.table_next_column();
        if ui.checkbox(im_str!("Bushes"), &mut self.sideview.map.bushes) {
            action.set(EditAction::Update);
        }

        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text("Features");

        ui.table_next_column();
        if ui
            .input_int(im_str!("Floor Pos"), &mut self.sideview.map.floor)
            .build()
        {
            self.sideview.map.floor = clamp(self.sideview.map.floor, 0, 15);
            action.set(EditAction::Update);
        }
        ui.table_next_column();
        if ui
            .input_int(im_str!("Tile Set"), &mut self.sideview.map.tileset)
            .build()
        {
            self.sideview.map.tileset = clamp(self.sideview.map.tileset, 0, 7);
            action.set(EditAction::Update);
        }
        ui.table_next_column();
        if ui
            .input_int(im_str!("BG Map"), &mut self.sideview.map.background_map)
            .build()
        {
            self.sideview.map.background_map = clamp(self.sideview.map.background_map, 0, 7);
            action.set(EditAction::Update);
        }

        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text("Palettes");

        ui.table_next_column();
        if ui
            .input_int(
                im_str!("Background"),
                &mut self.sideview.map.background_palette,
            )
            .build()
        {
            self.sideview.map.background_palette =
                clamp(self.sideview.map.background_palette, 0, 7);
            action.set(EditAction::PaletteChanged);
        }
        ui.table_next_column();
        if ui
            .input_int(im_str!("Sprite"), &mut self.sideview.map.sprite_palette)
            .build()
        {
            self.sideview.map.sprite_palette = clamp(self.sideview.map.sprite_palette, 0, 7);
            action.set(EditAction::PaletteChanged);
        }
        ui.end_table();
        Ok(action)
    }
}

impl Gui for SideviewGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let config = Config::get(&self.edit.config()).unwrap();
        let scfg = config.sideview.find(&self.sideview.id).unwrap();
        let title = ImString::new(self.edit.title());
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let width = ui.push_item_width(400.0);
                if self.is_new {
                    imgui::ComboBox::new(im_str!("Area"))
                        .height(imgui::ComboBoxHeight::Large)
                        .build_simple(ui, self.selector.as_mut(), &self.names, &|x| {
                            Cow::Borrowed(x)
                        });
                } else {
                    ui.label_text(im_str!("Area"), &self.names[self.selector.value()]);
                }
                width.pop(ui);

                ui.same_line();
                let width = ui.push_item_width(100.0);
                if imgui::InputFloat::new(ui, im_str!("Scale"), &mut self.scale)
                    .step(0.25)
                    .build()
                {
                    self.scale = clamp(self.scale, 0.25, 4.0);
                }
                width.pop(ui);

                ui.same_line();
                match emulate::emulate_fourscreen(
                    im_str!("Emulate"),
                    ui,
                    project,
                    &self.edit,
                    &self.sideview.id,
                ) {
                    Ok(_) => {}
                    Err(e) => self.error.show("SideviewGui", "Emulator Error", Some(e)),
                };

                ui.same_line();
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self.error.show("SideviewGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = false;

                let size = ui.content_region_avail();
                imgui::ChildWindow::new(1)
                    .movable(false)
                    .size([size[0], 16.0 * 16.0 * self.scale])
                    .always_vertical_scrollbar(true)
                    .always_horizontal_scrollbar(true)
                    .build(ui, || {
                        let origin = ui.cursor_pos();
                        let scr_origin = ui.cursor_screen_pos();
                        changed |= self.draw_map(&config, origin, scr_origin, ui);
                        if !scfg.background_layer {
                            changed |= self.draw_enemies(&config, origin, scr_origin, ui);
                        }
                        self.draw_selectbox(changed, scr_origin, ui);
                    });
                imgui::TabBar::new(im_str!("Map Editor")).build(ui, || {
                    imgui::TabItem::new(im_str!("Map Commands")).build(ui, || {
                        changed |= self.draw_map_command_tab(&config, ui);
                    });
                    imgui::TabItem::new(im_str!("Enemies")).build(ui, || {
                        if !scfg.background_layer {
                            if self.sideview.enemy.is_encounter {
                                ui.radio_button(
                                    im_str!("Small Encounter"),
                                    &mut self.enemy_list,
                                    0,
                                );
                                changed |= self.draw_enemies_tab(0, &config, ui);
                                ui.separator();
                                ui.radio_button(
                                    im_str!("Large Encounter"),
                                    &mut self.enemy_list,
                                    1,
                                );
                                changed |= self.draw_enemies_tab(1, &config, ui);
                            } else {
                                changed |= self.draw_enemies_tab(0, &config, ui);
                            }
                        }
                    });
                    imgui::TabItem::new(im_str!("Connections")).build(ui, || {
                        changed |= self.draw_connections_tab(&config, ui);
                    });
                    imgui::TabItem::new(im_str!("Item Availability")).build(ui, || {
                        changed |= self.draw_availability_tab(ui);
                    });
                });

                match KeyAction::get(ui) {
                    KeyAction::Cut => {
                        self.copy_to_clipboard(true, ui);
                        self.decompress(&config);
                        changed = true;
                    }

                    KeyAction::Copy => {
                        self.copy_to_clipboard(false, ui);
                    }
                    KeyAction::Paste => {
                        self.paste_from_clipboard(ui);
                        self.decompress(&config);
                        changed = true;
                    }
                    KeyAction::SelectAll => {
                        self.selectbox.init(0, 0);
                        self.selectbox.drag(
                            Decompressor::WIDTH as isize - 1,
                            Decompressor::HEIGHT as isize,
                        );
                    }
                    KeyAction::Undo => {
                        if let Some(sideview) = self.undo.undo() {
                            self.sideview = sideview.clone();
                            self.decompress(&config);
                            self.reset_caches();
                        }
                    }
                    KeyAction::Redo => {
                        if let Some(sideview) = self.undo.redo() {
                            self.sideview = sideview.clone();
                            self.decompress(&config);
                            self.reset_caches();
                        }
                    }
                    KeyAction::None => {}
                }

                if changed {
                    self.undo.push(self.sideview.clone());
                }
                self.changed |= changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Sideview Changed"),
            "There are unsaved changes in the Sideview Editor.\nDo you want to discard them?",
            ui,
        );
        if self.selector.draw(
            self.changed,
            im_str!("Sideview Changed"),
            "There are unsaved changes in the Sideview Editor.\nDo you want to discard them?",
            ui,
        ) {
            let id = &self.ids[self.selector.value()];
            self.sideview = match Sideview::from_rom(&self.edit, id.clone()) {
                Ok(val) => val,
                Err(e) => {
                    self.error.show("SideviewGui", "Error loading map", Some(e));
                    Sideview::new(id.clone())
                }
            };
            self.enemy_list = 0;
            self.decompress(&config);
            self.reset_caches();
            self.list_object_names().expect("list object names");
            self.undo.reset(self.sideview.clone());
            self.changed = false;
        }
    }

    fn refresh(&mut self) {
        self.reset_caches();
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.edit.random_id
    }
}
