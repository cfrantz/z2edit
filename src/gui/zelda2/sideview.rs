use std::collections::HashMap;
use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::{ImStr, ImString, MouseButton};

use crate::errors::*;
use crate::gui::fa;
use crate::gui::util::tooltip;
use crate::gui::util::{DragHelper, SelectBox};
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::{Selector, Visibility};
use crate::nes::IdPath;
use crate::nes::MemoryAccess;
use crate::util::clamp;
use crate::zelda2::config::Config;
use crate::zelda2::objects::config::ObjectKind;
use crate::zelda2::project::{Edit, EditAction, Project, RomData};
use crate::zelda2::sideview::{Decompressor, MapCommand, Sideview};

pub struct SideviewGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    ids: Vec<IdPath>,
    background: TileCache,
    item: TileCache,
    enemies: TileCache,
    selector: Selector,
    scale: f32,
    sideview: Sideview,
    decomp: Decompressor,
    drag_helper: DragHelper,
    objects: Vec<(ImString, u8)>,
    objects_map: HashMap<usize, usize>,
    extras: Vec<(ImString, u8)>,
    extras_map: HashMap<usize, usize>,
    items: Vec<(ImString, u8)>,
    items_map: HashMap<usize, usize>,
}

impl SideviewGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let config = Config::get(&edit.meta.borrow().config)?;

        let mut sideview = if commit_index == -1 {
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
        let north_palace = IdPath::from("west_hyrule/0");
        for group in config.sideview.group.iter() {
            for i in 0..group.length {
                let name = if let Some(pet_name) = group.pet_names.get(&i) {
                    format!("{:02}: {} {} ({})", i, group.name, i, pet_name)
                } else {
                    format!("{:02}: {} {}", i, group.name, i)
                };
                let name = ImString::from(name);
                names.push(name);
                let idpath = IdPath(vec![group.id.clone(), i.to_string()]);
                if sideview.id == idpath {
                    selected = ids.len() - 1;
                }
                if idpath == north_palace {
                    default_select = ids.len();
                }
                ids.push(idpath);
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

        let background = TileCache::new(
            &edit,
            Schema::MetaTile(
                edit.meta.borrow().config.clone(),
                sideview.id.clone(),
                sideview.map.background_palette,
            ),
        );
        let item = TileCache::new(&edit, Schema::Item(edit.meta.borrow().config.clone()));
        let enemies = TileCache::new(
            &edit,
            Schema::Enemy(edit.meta.borrow().config.clone(), sideview.id.clone()),
        );

        let win_id = edit.win_id(commit_index);
        let mut ret = Box::new(SideviewGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            names: names,
            ids: ids,
            background: background,
            item: item,
            enemies: enemies,
            selector: Selector::new(selected),
            scale: 1.0,
            sideview: sideview,
            decomp: decomp,
            drag_helper: DragHelper::default(),
            objects: Vec::new(),
            objects_map: HashMap::new(),
            extras: Vec::new(),
            extras_map: HashMap::new(),
            items: Vec::new(),
            items_map: HashMap::new(),
        });
        ret.list_object_names()?;
        Ok(ret)
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let edit = Box::new(self.sideview.clone());
        let i = project.commit(self.commit_index, edit, None)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }

    pub fn reset_caches(&mut self) {
        self.background.reset(Schema::MetaTile(
            self.edit.meta.borrow().config.clone(),
            self.sideview.id.clone(),
            self.sideview.map.background_palette,
        ));
        self.item
            .reset(Schema::Item(self.edit.meta.borrow().config.clone()));
        self.enemies.reset(Schema::Enemy(
            self.edit.meta.borrow().config.clone(),
            self.sideview.id.clone(),
        ));
    }

    fn list_object_names(&mut self) -> Result<()> {
        let config = Config::get(&self.edit.meta.borrow().config)?;
        let scfg = config.sideview.find(&self.sideview.id)?;

        self.objects.clear();
        self.extras.clear();
        self.objects_map.clear();
        self.extras_map.clear();
        for (i, obj) in config
            .objects
            .list(&scfg.kind, &ObjectKind::Small)
            .iter()
            .enumerate()
        {
            self.objects
                .push((im_str!("{:02x}: {}", obj.id, obj.name), obj.id));
            self.objects_map.insert(obj.id as usize, i);
        }
        let delta = self.objects.len();
        for (i, obj) in config
            .objects
            .list(&scfg.kind, &ObjectKind::Objset(self.sideview.map.objset))
            .iter()
            .enumerate()
        {
            self.objects
                .push((im_str!("{:02x}: {}", obj.id, obj.name), obj.id));
            self.objects_map.insert(obj.id as usize, i + delta);
        }
        for (i, obj) in config
            .objects
            .list(&scfg.kind, &ObjectKind::Extra)
            .iter()
            .enumerate()
        {
            self.extras
                .push((im_str!("{:02x}: {}", obj.id, obj.name), obj.id));
            self.extras_map.insert(obj.id as usize, i);
        }
        for (i, obj) in config.items.item.iter().enumerate() {
            self.items
                .push((im_str!("{:02x}: {}", obj.offset, obj.name), obj.offset));
            self.items_map.insert(obj.offset as usize, i);
        }

        Ok(())
    }

    fn draw_map(&mut self, config: &Config, ui: &imgui::Ui) -> bool {
        let origin = ui.cursor_pos();
        let scr_origin = ui.cursor_screen_pos();
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
            if ui.button(im_str!("Copy"), [0.0, 0.0]) {
                action.set(EditAction::CopyAt(index));
            }
            ui.same_line(0.0);
            if ui.button(im_str!("Delete"), [0.0, 0.0]) {
                action.set(EditAction::Delete(index));
            }
            ui.end_popup();
        }
        id.pop(ui);
        Ok(action)
    }

    fn draw_map_command_tab(&mut self, config: &Config, ui: &imgui::Ui) -> bool {
        let mut action = EditAction::None;
        action.set(
            self.draw_map_command_header(config, ui)
                .expect("map_command_header"),
        );
        ui.text("\nMap Commands:");
        ui.separator();
        for i in 0..self.sideview.map.data.len() {
            action.set(self.draw_map_command(i, false, ui).expect("map_command"));
        }
        self.process_map_action(action, &config)
    }

    fn process_map_action(&mut self, action: EditAction, config: &Config) -> bool {
        if action != EditAction::None {
            info!("action = {:?}", action);
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
            self.decomp.decompress(
                &self.sideview,
                self.sideview.background_layer_from_rom(&self.edit).as_ref(),
                &config.sideview,
                &config.objects,
            );
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
            if ui.button(&im_str!("{}", fa::ICON_COPY), [0.0, 0.0]) {
                action.set(EditAction::NewAt(index));
            }
            tooltip("Insert a new Map Command", ui);
            ui.same_line(0.0);
            if ui.button(&im_str!("{}", fa::ICON_ARROW_UP), [0.0, 0.0]) {
                if index > 0 {
                    action.set(EditAction::Swap(index, index - 1));
                }
            }
            tooltip("Move Up", ui);
            ui.same_line(0.0);
            if ui.button(&im_str!("{}", fa::ICON_ARROW_DOWN), [0.0, 0.0]) {
                if index < self.sideview.map.data.len() - 1 {
                    action.set(EditAction::Swap(index, index + 1));
                }
            }
            tooltip("Move Down", ui);
        }

        if !popup {
            ui.same_line(120.0);
        }
        let y = &mut self.sideview.map.data[index].y;
        let label = match y {
            13 => im_str!("New floor "),
            14 => im_str!("X-skip    "),
            15 => im_str!("Extra obj "),
            _ => im_str!("Y position"),
        };

        let width = ui.push_item_width(100.0);
        if ui.input_int(label, y).build() {
            *y = clamp(*y, 0, 15);
            action.set(EditAction::Update);
        }
        let y = self.sideview.map.data[index].y;

        if !popup {
            ui.same_line(320.0);
        }
        {
            let x = &mut self.sideview.map.data[index].x;
            if ui.input_int(im_str!("X position"), x).build() {
                *x = clamp(*x, 0, 63);
                action.set(EditAction::Update);
            }
        }
        width.pop(ui);

        let objects = self
            .objects
            .iter()
            .map(|s| s.0.as_ref())
            .collect::<Vec<&ImStr>>();
        let extras = self
            .extras
            .iter()
            .map(|s| s.0.as_ref())
            .collect::<Vec<&ImStr>>();

        let width = ui.push_item_width(200.0);
        if y < 13 {
            if !popup {
                ui.same_line(510.0);
            }
            let kind = &mut self.sideview.map.data[index].kind;
            let mut sel = self.objects_map[kind];
            if imgui::ComboBox::new(im_str!("Object")).build_simple_string(ui, &mut sel, &objects) {
                *kind = self.objects[sel].1 as usize;
                action.set(EditAction::Update);
            }
        } else if y == 15 {
            if !popup {
                ui.same_line(510.0);
            }
            let kind = &mut self.sideview.map.data[index].kind;
            let mut sel = self.extras_map[kind];
            if imgui::ComboBox::new(im_str!("Object")).build_simple_string(ui, &mut sel, &extras) {
                *kind = self.extras[sel].1 as usize;
                action.set(EditAction::Update);
            }
        }
        width.pop(ui);

        if y != 14 {
            if !popup {
                ui.same_line(770.0);
            }
            if self.sideview.map.data[index].kind == 0x0f {
                let width = ui.push_item_width(200.0);
                let item = &mut self.sideview.map.data[index].param;
                let mut sel = self.items_map[&(*item as usize)];
                let items = self
                    .items
                    .iter()
                    .map(|s| s.0.as_ref())
                    .collect::<Vec<&ImStr>>();
                if imgui::ComboBox::new(im_str!("Item")).build_simple_string(ui, &mut sel, &items) {
                    *item = self.objects[sel].1 as i32;
                    action.set(EditAction::Update);
                }
                width.pop(ui);
            } else {
                let width = ui.push_item_width(100.0);
                let p = &mut self.sideview.map.data[index].param;
                if ui.input_int(im_str!("Param"), p).build() {
                    *p = clamp(*p, 0, if y == 13 { 255 } else { 15 });
                    action.set(EditAction::Update);
                }
                width.pop(ui);
            }
        }
        if !popup {
            ui.same_line(1020.0);
            if ui.button(&im_str!("{}", fa::ICON_TRASH), [0.0, 0.0]) {
                action.set(EditAction::Delete(index));
            }
            tooltip("Delete", ui);
        }

        id0.pop(ui);
        Ok(action)
    }

    fn draw_map_command_header(&mut self, config: &Config, ui: &imgui::Ui) -> Result<EditAction> {
        let scfg = config.sideview.find(&self.sideview.id)?;
        let index = self.sideview.id.usize_at(1)?;
        let rom = self.edit.rom.borrow();

        let ptr = scfg.address + index * 2;
        let addr = rom.read_pointer(ptr)?;

        let mut action = EditAction::None;

        ui.text(im_str!("Map pointer at {:x?}", ptr));
        ui.text(im_str!("Map address is {:x?}", addr));

        ui.separator();
        let width = ui.push_item_width(100.0);
        ui.text("Map Properties");

        ui.same_line(120.0);
        if ui
            .input_int(im_str!("Width"), &mut self.sideview.map.width)
            .build()
        {
            self.sideview.map.width = clamp(self.sideview.map.width, 1, 4);
            action.set(EditAction::Update);
        }
        ui.same_line(320.0);
        if ui
            .input_int(im_str!("Object Set"), &mut self.sideview.map.objset)
            .build()
        {
            self.sideview.map.objset = clamp(self.sideview.map.objset, 0, 1);
            action.set(EditAction::CacheInvalidate);
        }
        ui.same_line(540.0);
        if ui.checkbox(
            im_str!("Cursor moves left"),
            &mut self.sideview.map.cursor_moves_left,
        ) {
            action.set(EditAction::Update);
        }
        ui.separator();

        ui.text("Flags");
        ui.same_line(120.0);
        if ui.checkbox(im_str!("Ceiling"), &mut self.sideview.map.ceiling) {
            action.set(EditAction::Update);
        }
        ui.same_line(320.0);
        if ui.checkbox(im_str!("Grass"), &mut self.sideview.map.grass) {
            action.set(EditAction::Update);
        }
        ui.same_line(540.0);
        if ui.checkbox(im_str!("Bushes"), &mut self.sideview.map.bushes) {
            action.set(EditAction::Update);
        }
        ui.separator();

        ui.text("Features");
        ui.same_line(120.0);
        if ui
            .input_int(im_str!("Tile Set"), &mut self.sideview.map.tileset)
            .build()
        {
            self.sideview.map.tileset = clamp(self.sideview.map.tileset, 0, 7);
            action.set(EditAction::Update);
        }
        ui.same_line(320.0);
        if ui
            .input_int(im_str!("Floor Position"), &mut self.sideview.map.floor)
            .build()
        {
            self.sideview.map.floor = clamp(self.sideview.map.floor, 0, 15);
            action.set(EditAction::Update);
        }
        ui.same_line(540.0);
        if ui
            .input_int(im_str!("BG Map"), &mut self.sideview.map.background_map)
            .build()
        {
            self.sideview.map.background_map = clamp(self.sideview.map.background_map, 0, 7);
            action.set(EditAction::Update);
        }

        ui.separator();
        ui.text("Palettes");
        ui.same_line(120.0);
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
        ui.same_line(320.0);
        if ui
            .input_int(im_str!("Sprite"), &mut self.sideview.map.sprite_palette)
            .build()
        {
            self.sideview.map.sprite_palette = clamp(self.sideview.map.sprite_palette, 0, 7);
            action.set(EditAction::PaletteChanged);
        }
        width.pop(ui);
        ui.separator();

        Ok(action)
    }
}

impl Gui for SideviewGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let config = Config::get(&self.edit.meta.borrow().config).unwrap();
        imgui::Window::new(&im_str!("Sideview##{}", self.win_id))
            .opened(&mut visible)
            .build(ui, || {
                let names = self
                    .names
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&ImStr>>();
                let width = ui.push_item_width(400.0);
                if self.commit_index == -1 {
                    imgui::ComboBox::new(im_str!("Area")).build_simple_string(
                        ui,
                        self.selector.as_mut(),
                        &names,
                    );
                } else {
                    ui.label_text(im_str!("Area"), &self.names[self.selector.value()]);
                }
                width.pop(ui);

                ui.same_line(0.0);
                let width = ui.push_item_width(100.0);
                if imgui::InputFloat::new(ui, im_str!("Scale"), &mut self.scale)
                    .step(0.25)
                    .build()
                {
                    self.scale = clamp(self.scale, 0.25, 4.0);
                }
                width.pop(ui);

                ui.same_line(0.0);
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => error!("SideviewGui: commit error {:?}", e),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = false;

                imgui::ChildWindow::new(1)
                    .movable(false)
                    .size([65.0 * 16.0 * self.scale, 16.0 * 16.0 * self.scale])
                    .always_vertical_scrollbar(true)
                    .always_horizontal_scrollbar(true)
                    .build(ui, || {
                        changed |= self.draw_map(&config, ui);
                    });
                imgui::TabBar::new(im_str!("Map Editor")).build(ui, || {
                    imgui::TabItem::new(im_str!("Map Commands")).build(ui, || {
                        changed |= self.draw_map_command_tab(&config, ui);
                    });
                    imgui::TabItem::new(im_str!("Enemies")).build(ui, || {});
                    imgui::TabItem::new(im_str!("Connections")).build(ui, || {});
                    imgui::TabItem::new(im_str!("Item Availability")).build(ui, || {});
                });
                if changed {
                    // stuff.
                }
                self.changed |= changed;
            });
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
                    error!("Error loading map: {:?}", e);
                    Sideview::new(id.clone())
                }
            };
            self.decomp.decompress(
                &self.sideview,
                self.sideview.background_layer_from_rom(&self.edit).as_ref(),
                &config.sideview,
                &config.objects,
            );

            //            self.undo.reset(self.overworld.clone());
            self.reset_caches();
        }
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.win_id
    }
}
