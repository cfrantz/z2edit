use std::convert::{From, TryFrom};
use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::{ImStr, ImString, MouseButton};

use crate::errors::*;
use crate::gui::util::{text_outlined, KeyAction};
use crate::gui::util::{DragHelper, SelectBox};
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::{Selector, Visibility};
use crate::nes::IdPath;
use crate::nes::MemoryAccess;
use crate::util::clamp;
use crate::util::undo::UndoStack;
use crate::zelda2::config::Config;
use crate::zelda2::overworld::{Connector, Encounter, JsonMap, Map, Overworld};
use crate::zelda2::project::{Edit, EmulateAt, Project};

pub struct OverworldGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    overworld: Overworld,
    undo: UndoStack<Overworld>,
    cache: TileCache,
    selector: Selector,
    tile_selected: usize,
    scale: f32,
    selectbox: SelectBox,
    button_down: bool,
    select_drag: bool,
    conn_show: bool,
    conn_selected: usize,
    conn_drag: DragHelper,
    cursor: [isize; 2],
}

impl OverworldGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let config = Config::get(&edit.meta.borrow().config)?;

        let overworld = if commit_index == -1 {
            let which = &config.overworld.map[0];
            let id = IdPath(vec![which.id.clone()]);
            Overworld::from_rom(&edit, id)?
        } else {
            let obj = edit.edit.borrow();
            let ov = obj.as_any().downcast_ref::<Overworld>().unwrap();
            ov.clone()
        };

        let mut names = Vec::new();
        let mut selected = 0;
        for (i, map) in config.overworld.map.iter().enumerate() {
            names.push(ImString::from(map.name.clone()));
            if map.id == overworld.id.at(0) {
                selected = i;
            }
        }

        let win_id = edit.win_id(commit_index);
        let cache = TileCache::new(
            &edit,
            Schema::Overworld(edit.meta.borrow().config.clone(), overworld.id.clone()),
        );

        let mut undo = UndoStack::new(1000);
        undo.push(overworld.clone());
        Ok(Box::new(OverworldGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            names: names,
            overworld: overworld,
            undo: undo,
            cache: cache,
            selector: Selector::new(selected),
            tile_selected: 0,
            scale: 1.0,
            selectbox: SelectBox::default(),
            button_down: false,
            select_drag: false,
            conn_show: true,
            conn_selected: 0,
            conn_drag: DragHelper::default(),
            cursor: [0, 0],
        }))
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let config = Config::get(&self.edit.meta.borrow().config).unwrap();
        let overworld = &config.overworld.map[self.selector.value()];
        let i = project.commit(
            self.commit_index,
            Box::new(self.overworld.clone()),
            Some(&overworld.name),
        )?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }

    pub fn copy_to_clipboard(&self, ui: &imgui::Ui) {
        if self.selectbox.valid() {
            let mut map = Map::default();
            map.width = (self.selectbox.x1 - self.selectbox.x0 + 1) as usize;
            map.height = (self.selectbox.y1 - self.selectbox.y0 + 1) as usize;
            for y in self.selectbox.y0..=self.selectbox.y1 {
                let mut row = Vec::new();
                for x in self.selectbox.x0..=self.selectbox.x1 {
                    row.push(self.overworld.map.data[y as usize][x as usize]);
                }
                map.data.push(row);
            }

            let map = JsonMap::from(map);
            let text = ImString::new(serde_json::to_string_pretty(&map).unwrap());
            ui.set_clipboard_text(&text);
        }
    }

    pub fn decode_clipboard(&self, ui: &imgui::Ui) -> Result<Map> {
        if let Some(text) = ui.clipboard_text() {
            let map = serde_json::from_str::<JsonMap>(text.to_str())?;
            Map::try_from(map)
        } else {
            Ok(Map::default())
        }
    }

    pub fn paste_from_clipboard(&mut self, ui: &imgui::Ui) {
        let (x0, y0) = if self.selectbox.valid() {
            (self.selectbox.x0 as usize, self.selectbox.y0 as usize)
        } else {
            (self.cursor[0] as usize, self.cursor[1] as usize)
        };
        match self.decode_clipboard(ui) {
            Ok(map) => {
                if map.width > 0 && map.height > 0 {
                    for y in 0..map.height {
                        for x in 0..map.width {
                            let xp = x + x0;
                            let yp = y + y0;
                            if xp < self.overworld.map.width && yp < self.overworld.map.height {
                                self.overworld.map.data[yp][xp] = map.data[y][x];
                            }
                        }
                    }
                    self.selectbox.init(x0 as isize, y0 as isize);
                    self.selectbox.drag(
                        clamp(x0 + map.width, 0, self.overworld.map.width) as isize - 1,
                        clamp(y0 + map.height, 0, self.overworld.map.height) as isize - 1,
                    );
                }
            }
            Err(e) => {
                warn!("OverworldGui clipboard: {:?}", e);
            }
        }
    }

    fn draw_tile_selection(&mut self, ui: &imgui::Ui) {
        let config = Config::get(&self.edit.meta.borrow().config).unwrap();
        let overworld = &config.overworld.map[self.selector.value()];
        let group = ui.begin_group();
        ui.text(im_str!(
            "Map:\n  Width: {}\n  Height: {}",
            self.overworld.map.width,
            self.overworld.map.height
        ));

        ui.text(im_str!(""));
        ui.text(im_str!(
            "Cursor:\n  X: {}\n  Y: {}",
            self.cursor[0],
            self.cursor[1]
        ));

        ui.text(im_str!(""));
        ui.text(im_str!("Tiles:"));
        for i in 0..overworld.objtable_len {
            if i % 4 != 0 {
                ui.same_line(0.0);
            }
            let image = self.cache.get(i as u8);
            let token = if i == self.tile_selected {
                Some(ui.push_style_colors(&[
                    (imgui::StyleColor::Button, [0.9, 0.9, 0.9, 0.9]),
                    (imgui::StyleColor::ButtonHovered, [1.0, 1.0, 1.0, 1.0]),
                    (imgui::StyleColor::ButtonActive, [0.9, 0.9, 0.9, 0.9]),
                ]))
            } else {
                None
            };
            if imgui::ImageButton::new(image.id, [32.0, 32.0])
                .frame_padding(4)
                .build(ui)
            {
                self.tile_selected = i;
            }
            if let Some(token) = token {
                token.pop(ui);
            }
        }
        group.end(ui);
    }

    fn draw_map(&mut self, ui: &imgui::Ui) -> bool {
        let origin = ui.cursor_pos();
        let scr_origin = ui.cursor_screen_pos();
        let scale = 16.0 * self.scale;
        for (y, row) in self.overworld.map.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let image = self.cache.get(*col);
                let xo = origin[0] + x as f32 * scale;
                let yo = origin[1] + y as f32 * scale;
                image.draw_at([xo, yo], self.scale, ui);
            }
        }

        // Manage the connection list here so we can abort before all of
        // tile editing stuff.  This is important because we want to handle
        // connection mouse events first and skip processing tile-edit mouse
        // if we handled them for connections.
        let mut changed = false;
        if self.conn_show {
            let mut focused = false;
            for i in 0..self.overworld.connector.len() {
                let (f, c) = self.draw_connection(i, ui);
                focused |= f;
                changed |= c;
            }
            if focused {
                return changed;
            }
        }

        let draw_list = ui.get_window_draw_list();
        let io = ui.io();
        let mouse_pos = io.mouse_pos;
        let mx = mouse_pos[0] - scr_origin[0];
        let my = mouse_pos[1] - scr_origin[1];
        let tx = if mx >= 0.0 { (mx / scale) as isize } else { -1 };
        let ty = if my >= 0.0 { (my / scale) as isize } else { -1 };
        let modifier = io.key_ctrl | io.key_shift | io.key_alt | io.key_super;

        let wmin = ui.window_content_region_min();
        let wmax = ui.window_content_region_max();
        let bounds = [
            wmax[0] - wmin[0] + ui.scroll_x(),
            wmax[1] - wmin[1] + ui.scroll_y(),
        ];

        if tx >= 0
            && tx < self.overworld.map.width as isize
            && ty >= 0
            && ty < self.overworld.map.height as isize
            && ui.is_window_hovered()
            && mx < bounds[0]
            && my < bounds[1]
        {
            self.cursor = [tx, ty];
            let x = scr_origin[0] + tx as f32 * scale;
            let y = scr_origin[1] + ty as f32 * scale;
            draw_list
                .add_rect([x, y], [x + scale, y + scale], [1.0, 1.0, 1.0, 1.0])
                .thickness(2.0)
                .build();

            if ui.is_mouse_clicked(MouseButton::Left) {
                self.button_down = true;
            }
            if ui.is_mouse_released(MouseButton::Left) {
                self.button_down = false;
                self.select_drag = false;
            }
            if !modifier && !self.select_drag && self.button_down {
                if self.selectbox.contains(tx, ty) {
                    for y in self.selectbox.y0..=self.selectbox.y1 {
                        for x in self.selectbox.x0..=self.selectbox.x1 {
                            self.overworld.map.data[y as usize][x as usize] =
                                self.tile_selected as u8;
                        }
                    }
                    changed = true;
                } else {
                    let new = self.tile_selected as u8;
                    let orig = self.overworld.map.data[ty as usize][tx as usize];
                    self.overworld.map.data[ty as usize][tx as usize] = new;
                    self.selectbox = SelectBox::default();
                    changed = new != orig;
                }
            }
            if !modifier && ui.is_mouse_clicked(MouseButton::Right) {
                self.selectbox = SelectBox::default();
            }
            if io.key_shift && ui.is_mouse_clicked(MouseButton::Left) {
                self.selectbox.init(tx, ty);
            }
        }
        if io.key_shift && ui.is_mouse_dragging(MouseButton::Left) {
            self.selectbox.drag(
                clamp(tx, 0, self.overworld.map.width as isize - 1),
                clamp(ty, 0, self.overworld.map.height as isize - 1),
            );
            self.select_drag = true;
        }
        if self.selectbox.valid() {
            let norm = self.selectbox.normalized();
            let x0 = scr_origin[0] + norm.x0 as f32 * scale;
            let y0 = scr_origin[1] + norm.y0 as f32 * scale;
            let x1 = scr_origin[0] + norm.x1 as f32 * scale + scale;
            let y1 = scr_origin[1] + norm.y1 as f32 * scale + scale;
            draw_list
                .add_rect([x0, y0], [x1, y1], [1.0, 1.0, 1.0, 0.25])
                .filled(true)
                .build();
            draw_list
                .add_rect([x0, y0], [x1, y1], [0.0, 0.0, 0.0, 1.0])
                .build();
        }

        match KeyAction::get(ui) {
            KeyAction::Cut | KeyAction::Copy => {
                self.copy_to_clipboard(ui);
            }
            KeyAction::Paste => {
                self.paste_from_clipboard(ui);
                changed = true;
            }
            KeyAction::SelectAll => {
                self.selectbox.init(0, 0);
                self.selectbox.drag(
                    self.overworld.map.width as isize - 1,
                    self.overworld.map.height as isize - 1,
                );
            }
            KeyAction::Undo => {
                if let Some(overworld) = self.undo.undo() {
                    self.overworld = overworld.clone();
                }
            }
            KeyAction::Redo => {
                if let Some(overworld) = self.undo.redo() {
                    self.overworld = overworld.clone();
                }
            }
            KeyAction::None => {}
        }
        changed
    }

    fn draw_connection(&mut self, n: usize, ui: &imgui::Ui) -> (bool, bool) {
        let conn = self.overworld.connector.get_mut(n);
        if conn.is_none() {
            return (false, false);
        }
        let mut conn = conn.unwrap();
        let mut changed = false;
        let scale = self.scale * 16.0;

        let id0 = ui.push_id(imgui::Id::Str(&conn.id.at(0)));
        let id1 = ui.push_id(imgui::Id::Str(&conn.id.at(1)));
        let delta = self.conn_drag.delta(n);
        let pos = [
            conn.x as f32 * scale + delta[0],
            conn.y as f32 * scale + delta[1],
        ];
        ui.set_cursor_pos(pos);
        text_outlined(
            ui,
            [1.0, 0.0, 1.0, 1.0],
            &im_str!("{:02}", conn.id.usize_at(1).unwrap()),
        );
        ui.set_cursor_pos(pos);
        ui.invisible_button(&im_str!("edit"), [16.0, 16.0]);
        let focus = ui.is_item_focused();
        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                self.conn_drag.start(n);
                self.conn_drag.drag(n, ui.io().mouse_delta);
            }
        } else {
            if let Some(amount) = self.conn_drag.finalize(n) {
                conn.x += (amount[0] / scale) as i32;
                conn.y += (amount[1] / scale) as i32;
                changed = true;
            }
        }
        if ui.popup_context_item(im_str!("connection")) {
            if ui.button(im_str!("Emulate"), [0.0, 0.0]) {
                OverworldGui::emulate_at(conn, &self.edit, self.selector.value());
            }
            changed |= OverworldGui::connection_edit(&mut conn, ui);
            ui.end_popup();
        }
        id1.pop(ui);
        id0.pop(ui);
        (focus, changed)
    }

    fn emulate_at(conn: &Connector, edit: &Rc<Edit>, ov: usize) {
        let config = Config::get(&edit.meta.borrow().config).unwrap();
        let overworld = &config.overworld.map[ov];
        let mut index = conn.id.usize_at(1).unwrap();
        let mut at = EmulateAt::default();

        let (region, bank) = if conn.dest_world == 0 {
            if conn.external {
                // Overworld 1 can be either DM or MZ depending on whether you
                // are transfering from overworld 0 or overworld 2.
                // Z2Edit represents overworld 1 as a subworld.  We search
                // available maps for the configured (overworld, subworld)
                // combination.
                let (ovw, sub) = if conn.dest_overworld == 1 {
                    (overworld.overworld, 1)
                } else {
                    (conn.dest_overworld, 0)
                };
                let mut r = 0;
                let mut b = 1;
                for map in config.overworld.map.iter() {
                    if map.overworld == ovw && map.subworld == sub {
                        r = if map.subworld != 0 {
                            map.subworld as u8
                        } else {
                            map.overworld as u8
                        };
                        b = map.pointer.bank().unwrap().1 as u8;
                    }
                }
                (r, b)
            } else {
                let r = if overworld.subworld != 0 {
                    overworld.subworld as u8
                } else {
                    overworld.overworld as u8
                };
                let b = overworld.pointer.bank().unwrap().1 as u8;
                (r, b)
            }
        } else {
            let r = conn.dest_overworld as u8;
            let rom = edit.rom.borrow();
            let b = rom
                .read(config.misc.world_to_bank + conn.dest_world)
                .unwrap_or(0);
            (r, b)
        };

        let town_code = config.overworld.town_code(index);
        if town_code.is_some() {
            // Always use the even connector for towns.
            index &= 0xFE;
        }

        at.bank = bank;
        at.region = region;
        at.world = conn.dest_world as u8;
        at.town_code = town_code.unwrap_or(0) as u8;
        at.palace_code = config.overworld.palace_code(index).unwrap_or(0) as u8;
        at.connector = index as u8;
        at.room = conn.dest_map as u8;
        at.page = conn.entry as u8;
        at.prev_region = overworld.overworld as u8;

        match edit.emulate(Some(at)) {
            Err(e) => error!("Error starting emulator: {:?}", e),
            Ok(_) => {}
        };
    }

    fn draw_connection_dialog(&mut self, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        ui.popup(im_str!("connections"), || {
            #[rustfmt::skip]
            imgui::ComboBox::new(im_str!("Connection"))
                .build_simple_string(ui, &mut self.conn_selected, &[
                &im_str!("00"), &im_str!("01"), &im_str!("02"), &im_str!("03"), &im_str!("04"), &im_str!("05"), &im_str!("06"), &im_str!("07"), &im_str!("08"), &im_str!("09"),
                &im_str!("10"), &im_str!("11"), &im_str!("12"), &im_str!("13"), &im_str!("14"), &im_str!("15"), &im_str!("16"), &im_str!("17"), &im_str!("18"), &im_str!("19"),
                &im_str!("20"), &im_str!("21"), &im_str!("22"), &im_str!("23"), &im_str!("24"), &im_str!("25"), &im_str!("26"), &im_str!("27"), &im_str!("28"), &im_str!("29"),
                &im_str!("30"), &im_str!("31"), &im_str!("32"), &im_str!("33"), &im_str!("34"), &im_str!("35"), &im_str!("36"), &im_str!("37"), &im_str!("38"), &im_str!("39"),
                &im_str!("40"), &im_str!("41"), &im_str!("42"), &im_str!("43"), &im_str!("44"), &im_str!("45"), &im_str!("46"), &im_str!("47"), &im_str!("48"), &im_str!("49"),
                &im_str!("50"), &im_str!("51"), &im_str!("52"), &im_str!("53"), &im_str!("54"), &im_str!("55"), &im_str!("56"), &im_str!("57"), &im_str!("58"), &im_str!("59"),
                &im_str!("60"), &im_str!("61"), &im_str!("62")]);
            ui.separator();
            changed |= OverworldGui::connection_edit(&mut self.overworld.connector[self.conn_selected], ui);
        });
        changed
    }

    fn connection_edit(conn: &mut Connector, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        let width = ui.push_item_width(100.0);

        ui.text("Position:");
        changed |= ui.input_int(im_str!("xpos"), &mut conn.x).build();
        ui.same_line(0.0);
        changed |= ui.input_int(im_str!("ypos"), &mut conn.y).build();

        ui.text("Connects to:");
        changed |= ui.input_int(im_str!("Map"), &mut conn.dest_map).build();
        ui.same_line(0.0);
        let width2 = ui.push_item_width(40.0);
        changed |= imgui::ComboBox::new(im_str!("W##world")).build_simple_string(
            ui,
            &mut conn.dest_world,
            &[
                &im_str!("0"),
                &im_str!("1"),
                &im_str!("2"),
                &im_str!("3"),
                &im_str!("4"),
                &im_str!("5"),
                &im_str!("6"),
                &im_str!("7"),
            ],
        );
        ui.same_line(0.0);
        changed |= imgui::ComboBox::new(im_str!("OV##overworld")).build_simple_string(
            ui,
            &mut conn.dest_overworld,
            &[&im_str!("0"), &im_str!("1"), &im_str!("2"), &im_str!("3")],
        );

        width2.pop(ui);

        ui.text("Properties:");
        changed |= imgui::ComboBox::new(im_str!("Entry")).build_simple_string(
            ui,
            &mut conn.entry,
            &[
                &im_str!("Screen 1"),
                &im_str!("Screen 2"),
                &im_str!("Screen 3"),
                &im_str!("Screen 4"),
            ],
        );

        if let Some(hidden) = conn.hidden.as_mut() {
            ui.same_line(0.0);
            changed |= ui.checkbox(im_str!("Hidden"), &mut hidden.hidden);
        }

        changed |= ui.checkbox(im_str!("Extern  "), &mut conn.external);
        ui.same_line(0.0);
        changed |= ui.checkbox(im_str!("Second  "), &mut conn.second);
        ui.same_line(0.0);
        changed |= ui.checkbox(im_str!("2 lower "), &mut conn.exit_2_lower);

        changed |= ui.checkbox(im_str!("Right   "), &mut conn.entry_right);
        ui.same_line(0.0);
        changed |= ui.checkbox(im_str!("Passthru"), &mut conn.passthru);
        ui.same_line(0.0);
        changed |= ui.checkbox(im_str!("Fall    "), &mut conn.fall);

        width.pop(ui);
        changed
    }

    fn encounter_edit(e: &mut Encounter, ui: &imgui::Ui) -> bool {
        let mut changed = false;

        let width = ui.push_item_width(100.0);
        changed |= ui.input_int(im_str!("##dest"), &mut e.dest_map).build();
        ui.same_line(0.0);
        changed |= imgui::ComboBox::new(im_str!("##entry")).build_simple_string(
            ui,
            &mut e.entry,
            &[
                &im_str!("Screen 1"),
                &im_str!("Screen 2"),
                &im_str!("Screen 3"),
                &im_str!("Screen 4"),
            ],
        );
        width.pop(ui);
        changed
    }

    fn draw_encounters_dialog(&mut self, ui: &imgui::Ui) -> bool {
        let config = Config::get(&self.edit.meta.borrow().config).unwrap();
        let mut changed = false;
        ui.popup(im_str!("encounters"), || {
            changed |= ui
                .input_int(
                    im_str!("North-South Dividing Line"),
                    &mut self.overworld.mason_dixon_line,
                )
                .build();
            ui.separator();
            ui.text(im_str!(
                "Terrain    North Side                        South Side"
            ));
            ui.separator();
            for (i, terrain) in config.overworld.terrain_name.iter().enumerate() {
                ui.text(im_str!("{:10}", terrain));
                ui.same_line(0.0);
                changed |=
                    OverworldGui::encounter_edit(&mut self.overworld.encounter[2 * i + 0], ui);
                ui.same_line(0.0);
                ui.text(im_str!("  "));
                ui.same_line(0.0);
                changed |=
                    OverworldGui::encounter_edit(&mut self.overworld.encounter[2 * i + 1], ui);
            }
        });
        changed
    }
}

impl Gui for OverworldGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let config = Config::get(&self.edit.meta.borrow().config).unwrap();
        imgui::Window::new(&im_str!("Overworld##{}", self.win_id))
            .opened(&mut visible)
            .build(ui, || {
                let names = self
                    .names
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&ImStr>>();
                let width = ui.push_item_width(200.0);
                if self.commit_index == -1 {
                    imgui::ComboBox::new(im_str!("Overworld")).build_simple_string(
                        ui,
                        self.selector.as_mut(),
                        &names,
                    );
                } else {
                    ui.label_text(im_str!("Overworld"), &names[self.selector.value()]);
                }
                width.pop(ui);
                let mut changed = false;
                ui.same_line(0.0);
                ui.checkbox(im_str!("Show Connections"), &mut self.conn_show);
                ui.same_line(0.0);
                if ui.button(im_str!("Connections"), [0.0, 0.0]) {
                    ui.open_popup(im_str!("connections"));
                }
                changed |= self.draw_connection_dialog(ui);
                ui.same_line(0.0);
                if ui.button(im_str!("Encounters"), [0.0, 0.0]) {
                    ui.open_popup(im_str!("encounters"));
                }
                changed |= self.draw_encounters_dialog(ui);

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
                        Err(e) => error!("OverworldGui: commit error {}", e),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();

                self.draw_tile_selection(ui);
                ui.same_line(0.0);
                imgui::ChildWindow::new(1)
                    .movable(false)
                    .always_vertical_scrollbar(true)
                    .always_horizontal_scrollbar(true)
                    .build(ui, || {
                        changed |= self.draw_map(ui);
                    });
                if changed {
                    self.undo.push(self.overworld.clone());
                }
                self.changed |= changed;
            });
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Overworld Changed"),
            "There are unsaved changes in the Overworld Editor.\nDo you want to discard them?",
            ui,
        );
        if self.selector.draw(
            self.changed,
            im_str!("Overworld Changed"),
            "There are unsaved changes in the Overworld Editor.\nDo you want to discard them?",
            ui,
        ) {
            let which = &config.overworld.map[self.selector.value()];
            let id = IdPath(vec![which.id.clone()]);
            self.overworld = Overworld::from_rom(&self.edit, id).unwrap();
            self.undo.reset(self.overworld.clone());
            self.cache.reset(Schema::Overworld(
                self.edit.meta.borrow().config.clone(),
                self.overworld.id.clone(),
            ));
        }
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.win_id
    }
}
