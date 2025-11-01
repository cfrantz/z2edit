use anyhow::Result;
use pyo3::prelude::*;
use python_gui::{Color, Image};

use crate::error::Error;
use crate::gui::util::{edit_tree_node, TreeAction};
use crate::gui::util::{text_outlined, DragHelper, KeyAction, SelectBox};
use crate::gui::zelda2::multimap::MultiMapGui;
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::util::tile_cache::{GfxCache, GfxKind};
use crate::util::undo::UndoStack;
use crate::zelda2::metatile::MetatileGroup;
use crate::zelda2::overworld::{config, Connector, JsonMap, Map, Overworld};
use crate::zelda2::project::Project;

use imgui::{MouseButton, StyleVar};

impl GuiTree for config::Overworld {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        edit_tree_node(ui, &self.name, path, project)
    }
}

pub struct OverworldEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    scale: f32,
    undo: UndoStack<Overworld>,
    button_down: bool,
    select_drag: bool,
    selectbox: SelectBox,
    conn_drag: DragHelper,
    conn_show: bool,
    conn_selected: usize,
    tile_selected: usize,
    cursor: [isize; 2],
    compressed_size: usize,
    shaded: Image,
    overworld: Overworld,
    max_tiles: u8,
    spawn: Option<Box<dyn Gui>>,
}

impl OverworldEditor {
    #[rustfmt::skip]
    const CONNECTIONS: [&str; 63] = [
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
        "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
        "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
        "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "60", "61", "62",
    ];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const SELECTED: [f32; 4] = [1.0, 1.0, 1.0, 0.25];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];

    pub fn new(ov: &Overworld, path: &str) -> Result<Box<dyn Gui>> {
        let mut undo = UndoStack::new(1000);
        undo.reset(ov.clone());
        Ok(Box::new(OverworldEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            scale: 2.0,
            undo,
            button_down: false,
            select_drag: false,
            selectbox: SelectBox::default(),
            conn_drag: DragHelper::default(),
            conn_show: true,
            conn_selected: 0,
            tile_selected: 0,
            cursor: [0, 0],
            compressed_size: 0,
            shaded: Image::with_color(16, 16, Color::new(0x80808080)),
            overworld: ov.clone(),
            max_tiles: 16,
            spawn: None,
        }))
    }

    fn copy_to_clipboard(&self, ui: &imgui::Ui) {
        if self.selectbox.valid() {
            // FIXME: should use normalized selectbox coords.
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
            let text = serde_json::to_string_pretty(&map).unwrap();
            ui.set_clipboard_text(&text);
        }
    }

    fn decode_clipboard(&self, ui: &imgui::Ui) -> Result<Map> {
        if let Some(text) = ui.clipboard_text() {
            let map = serde_json::from_str::<JsonMap>(&text)?;
            Ok(Map::try_from(map)?)
        } else {
            Ok(Map::default())
        }
    }

    fn paste_from_clipboard(&mut self, ui: &imgui::Ui) {
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
                        (x0 + map.width).clamp(0, self.overworld.map.width) as isize - 1,
                        (y0 + map.height).clamp(0, self.overworld.map.height) as isize - 1,
                    );
                }
            }
            Err(e) => {
                log::warn!("OverworldGui clipboard: {:?}", e);
            }
        }
    }

    fn calculate_size(&mut self, project: &Project) -> Result<()> {
        let cfg = project.config.get::<config::Overworld>(&self.path)?;
        self.compressed_size = self.overworld.calculate_size(cfg).unwrap_or(0);
        Ok(())
    }

    fn draw_tile_selection(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let cfg = project.config.get::<config::Overworld>(&self.path)?;
        let _group = ui.begin_group();
        ui.text(format!(
            "Map:\n  Width: {}\n  Height: {}\n\n",
            self.overworld.map.width, self.overworld.map.height,
        ));
        ui.text(format!(
            "Cursor:\n  X: {}\n  Y: {}\n  Tile: {}\n",
            self.cursor[0],
            self.cursor[1],
            self.overworld.map.data[self.cursor[1] as usize][self.cursor[0] as usize]
        ));
        ui.text(format!(
            "Compressed Size:\n  {} / {} bytes\n\n",
            self.compressed_size, cfg.consts.ram.length,
        ));
        ui.text("Tiles:");
        let meta = project.data_ref::<MetatileGroup>(&cfg.metatile)?;
        let group = meta.group.get(&0).expect("background tiles");
        let _style = ui.push_style_var(StyleVar::FramePadding([4.0, 4.0]));
        for i in 0..group.tile.len() {
            if i % 4 != 0 {
                ui.same_line();
            }
            let image = GfxCache::get(
                project,
                &cfg.palette, // idpath of a palette group.
                "background", // key of a full palette within a group.
                GfxKind::Metatile(cfg.chr, cfg.metatile.clone(), i as u8),
            )?;

            let _style = if i == self.tile_selected {
                Some((
                    ui.push_style_color(imgui::StyleColor::Button, [0.9, 0.9, 0.9, 0.9]),
                    ui.push_style_color(imgui::StyleColor::ButtonHovered, [1.0, 1.0, 1.0, 1.0]),
                    ui.push_style_color(imgui::StyleColor::ButtonActive, [0.9, 0.9, 0.9, 0.9]),
                ))
            } else {
                None
            };
            if ui.image_button(format!("sel{i}"), image.imgui_id(), [32.0, 32.0]) {
                self.tile_selected = i;
            }
        }
        self.max_tiles = group.tile.len() as u8;
        Ok(())
    }

    fn draw_multimap_button(
        &mut self,
        conn: usize,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<()> {
        if ui.button("View Area") {
            let target = format!("{}/{}", self.path, conn);
            log::info!("Multimap for {target}");
            match MultiMapGui::new(&target, project) {
                Ok(m) => self.spawn = Some(m),
                Err(e) => log::error!("Error spawning multimap: {e}"),
            };
        }
        Ok(())
    }
    fn draw_emulator_button(
        &mut self,
        conn: usize,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<()> {
        if ui.button("Emulate") {
            let target = format!("{}/{}", self.path, conn);
            if let Some(svid) = project.connectivity.get(&target) {
                Python::with_gil(|py| project.emulate(py, Some(&svid)))?;
            } else {
                return Err(
                    Error::NotFound(format!("No destination map for {}", self.path)).into(),
                );
            }
        }
        Ok(())
    }

    fn draw_connection_dialog(&mut self, ui: &imgui::Ui, project: &Project) -> bool {
        let mut changed = false;
        if let Some(_popup) = ui.begin_popup("connections") {
            ui.combo_simple_string("Connection", &mut self.conn_selected, &Self::CONNECTIONS);
            ui.separator();
            changed |= Self::connection_edit(
                &mut self.overworld.connection[self.conn_selected],
                ui,
                project,
            );
        }
        changed
    }

    fn connection_edit(conn: &mut Connector, ui: &imgui::Ui, _project: &Project) -> bool {
        let mut changed = false;

        let _width = ui.push_item_width(100.0);
        ui.text("Position:");
        changed |= ui.input_scalar("xpos", &mut conn.x).step(1).build();
        ui.same_line();
        changed |= ui.input_scalar("ypos", &mut conn.y).step(1).build();

        ui.text("Connects to:");
        changed |= ui.input_scalar("Area", &mut conn.area).step(1).build();
        ui.same_line();
        if ui
            .input_scalar("W##world", &mut conn.dest_world)
            .step(1)
            .build()
        {
            conn.dest_world = conn.dest_world.clamp(0, 7);
            changed |= true;
        }
        ui.same_line();
        if ui
            .input_scalar("OV##overworld", &mut conn.dest_overworld)
            .step(1)
            .build()
        {
            conn.dest_overworld = conn.dest_overworld.clamp(0, 3);
            changed |= true;
        }

        ui.text("Properties:");
        if ui.input_scalar("Screen", &mut conn.screen).step(1).build() {
            conn.screen = conn.screen.clamp(0, 3);
            changed |= true;
        }
        if let Some(hidden) = conn.hidden.as_mut() {
            ui.same_line();
            changed |= ui.checkbox("Hidden", hidden);
        }

        changed |= ui.checkbox("Extern  ", &mut conn.external);
        ui.same_line();
        changed |= ui.checkbox("Second  ", &mut conn.second);
        ui.same_line();
        changed |= ui.checkbox("2 lower ", &mut conn.exit_2_lower);

        changed |= ui.checkbox("Right   ", &mut conn.entry_right);
        ui.same_line();
        changed |= ui.checkbox("Passthru", &mut conn.passthru);
        ui.same_line();
        changed |= ui.checkbox("Fall    ", &mut conn.fall);

        if let Some(palace) = conn.palace.as_mut() {
            ui.separator();
            ui.text("Palace Graphics & Palette:");
            changed |= ui
                .input_scalar("CHR Bank", &mut palace.chr_bank)
                .step(1)
                .build();
            changed |= ui
                .input_scalar("Palette", &mut palace.palette)
                .step(1)
                .build();
        }
        changed
    }

    fn draw_connection(&mut self, ui: &imgui::Ui, n: usize, project: &Project) -> (bool, bool) {
        let Some((_, conn)) = self.overworld.connection.get_index_mut(n) else {
            return (false, false);
        };
        let mut changed = false;
        let scale = self.scale * 16.0;
        let id = ui.push_id(&n.to_string());
        let delta = self.conn_drag.delta(n as usize);
        let pos = [
            conn.x as f32 * scale + delta[0],
            conn.y as f32 * scale + delta[1],
        ];
        ui.set_cursor_pos(pos);
        text_outlined(ui, Self::MAGENTA, &format!("{n:02}"));
        ui.set_cursor_pos(pos);
        ui.invisible_button("edit", [scale, scale]);
        let focus = ui.is_item_active();
        if focus {
            if ui.is_mouse_dragging(MouseButton::Left) {
                self.conn_drag.start(n as usize);
                self.conn_drag.drag(n as usize, ui.io().mouse_delta);
            }
        } else {
            if let Some(amount) = self.conn_drag.finalize(n as usize) {
                let x = conn.x as i8 + (amount[0] / scale) as i8;
                let y = conn.y as i8 + (amount[1] / scale) as i8;
                conn.x = x.clamp(0, 63) as u8;
                conn.y = y.clamp(0, 95) as u8;
                changed = true;
            }
        }
        let _ = conn;

        if let Some(_token) = ui.begin_popup_context_item() {
            ui.text(format!("Overworld Connector {n:02}"));
            ui.separator();
            let _ = self.draw_multimap_button(n, ui, project);
            ui.same_line();
            if let Err(e) = self.draw_emulator_button(n, ui, project) {
                self.error
                    .show("Emulation Error", "Error spawning emulator", e);
            }
            let (_, conn) = self.overworld.connection.get_index_mut(n).unwrap();
            changed |= Self::connection_edit(conn, ui, project);
        }
        id.pop();
        (focus, changed)
    }

    fn draw_map(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<bool> {
        let mut changed = false;
        let cfg = project.config.get::<config::Overworld>(&self.path)?;
        let origin = ui.cursor_pos();
        let scr_origin = ui.cursor_screen_pos();
        let mut bounds = ui.content_region_avail();
        bounds[0] += ui.scroll_x();
        bounds[1] += ui.scroll_y();
        let scale = 16.0 * self.scale;
        for (y, row) in self.overworld.map.data.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let tile = tile % self.max_tiles;
                let image = GfxCache::get(
                    project,
                    &cfg.palette, // idpath of a palette group.
                    "background", // key of a full palette within a group.
                    GfxKind::Metatile(cfg.chr, cfg.metatile.clone(), tile),
                )?;

                let xo = origin[0] + x as f32 * scale;
                let yo = origin[1] + y as f32 * scale;
                image.draw_at([xo, yo], self.scale, ui);
                if tile == 13 {
                    // FIXME: Hardcoded the walkable water tile.  Probably should be an item in
                    // config.
                    self.shaded.draw_at([xo, yo], self.scale, ui);
                }
            }
        }

        // Manage the connection list here so we can abort before all of
        // tile editing stuff.  This is important because we want to handle
        // connection mouse events first and skip processing tile-edit mouse
        // if we handled them for connections.
        if self.conn_show {
            let mut focused = false;
            for i in 0..self.overworld.connection.len() {
                let (f, c) = self.draw_connection(ui, i, project);
                focused |= f;
                changed |= c;
            }
            if focused {
                return Ok(changed);
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

        if ui.is_window_hovered()
            && tx >= 0
            && tx < self.overworld.map.width as isize
            && ty >= 0
            && ty < self.overworld.map.height as isize
            && mx < bounds[0]
            && my < bounds[1]
        {
            self.cursor = [tx, ty];
            let x = scr_origin[0] + tx as f32 * scale;
            let y = scr_origin[1] + ty as f32 * scale;

            draw_list
                .add_rect([x, y], [x + scale, y + scale], Self::WHITE)
                .thickness(2.0)
                .build();
            if ui.is_mouse_clicked(MouseButton::Left) {
                self.button_down = true;
            } else if ui.is_mouse_released(MouseButton::Right) {
                self.select_drag = false;
            } else if ui.is_mouse_released(MouseButton::Left) {
                self.button_down = false;
                self.select_drag = false;
            }
            if !modifier && !self.select_drag && self.button_down {
                if self.selectbox.contains(tx, ty) {
                    for y in self.selectbox.y0..=self.selectbox.y1 {
                        for x in self.selectbox.x0..=self.selectbox.x1 {
                            let new = self.tile_selected as u8;
                            let orig = self.overworld.map.data[ty as usize][tx as usize];
                            self.overworld.map.data[y as usize][x as usize] = new;
                            changed |= new != orig;
                        }
                    }
                } else {
                    let new = self.tile_selected as u8;
                    let orig = self.overworld.map.data[ty as usize][tx as usize];
                    self.overworld.map.data[ty as usize][tx as usize] = new;
                    self.selectbox = SelectBox::default();
                    changed |= new != orig;
                }
            }
            if (!modifier && ui.is_mouse_clicked(MouseButton::Right))
                || (io.key_shift && ui.is_mouse_clicked(MouseButton::Left))
            {
                self.selectbox.init(tx, ty);
            }
        }
        if (!modifier && ui.is_mouse_dragging(MouseButton::Right))
            || (io.key_shift && ui.is_mouse_dragging(MouseButton::Left))
        {
            self.selectbox.drag(
                tx.clamp(0, self.overworld.map.width as isize - 1),
                ty.clamp(0, self.overworld.map.height as isize - 1),
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
                .add_rect([x0, y0], [x1, y1], Self::SELECTED)
                .filled(true)
                .build();
            draw_list.add_rect([x0, y0], [x1, y1], Self::BLACK).build();
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
                    self.calculate_size(project)?;
                }
            }
            KeyAction::Redo => {
                if let Some(overworld) = self.undo.redo() {
                    self.overworld = overworld.clone();
                    self.calculate_size(project)?;
                }
            }
            KeyAction::None => {}
        }
        Ok(changed)
    }

    fn commit(&self, project: &mut Project) -> Result<()> {
        project.commit(&self.path, Box::new(self.overworld.clone()))?;
        project.connectivity.scan(project)?;
        Ok(())
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match self.commit(project) {
                Ok(()) => self.changed = false,
                Err(e) => self.error.show(
                    "Commit Error",
                    &format!("Error comitting {:?}", self.path),
                    e,
                ),
            }
        }
        ui.same_line();
        ui.text(&self.path);
        ui.separator();

        ui.checkbox("Show Connections", &mut self.conn_show);
        ui.same_line();
        if ui.button("Connections") {
            ui.open_popup("connections");
        }
        self.changed |= self.draw_connection_dialog(ui, project);
        ui.same_line();
        let width = ui.push_item_width(150.0);
        ui.input_scalar("Scale", &mut self.scale).step(0.25).build();
        width.end();

        self.draw_tile_selection(ui, project)?;
        ui.same_line();
        ui.child_window("map")
            .movable(false)
            .always_vertical_scrollbar(true)
            .always_horizontal_scrollbar(true)
            .build(|| -> Result<()> {
                let changed = self.draw_map(ui, project)?;
                if changed || self.compressed_size == 0 {
                    self.calculate_size(project)?;
                }
                if changed {
                    self.undo.push(self.overworld.clone());
                }
                Ok(())
            })
            .transpose()?;
        Ok(())
    }
}

impl Gui for OverworldEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Overworld##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Overworld Changed",
            "There are unsaved chagnes in the Overworld Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }

    fn spawned(&mut self) -> Option<Box<dyn Gui>> {
        self.spawn.take()
    }
}
