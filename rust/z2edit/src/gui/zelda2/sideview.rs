use anyhow::Result;
use indexmap::{IndexMap, IndexSet};
use python_gui::fa;

use crate::gui::util::edit_tree_node;
use crate::gui::util::{tooltip, DragHelper, EditAction};
use crate::gui::widgets::Combo;
use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::nes::Address;
use crate::util::tile_cache::{GfxCache, GfxKind};
use crate::zelda2::enemies::config::EnemyGroup;
use crate::zelda2::items::config::Items;
use crate::zelda2::object::{Object, RenderInfo};
use crate::zelda2::overworld::config::Overworld as OverworldConfig;
use crate::zelda2::overworld::Overworld;
use crate::zelda2::palette::config::PaletteGroup;
use crate::zelda2::project::Project;
use crate::zelda2::sideview::{config, AreaKind, Decompressor, Enemy, MapCommand, Sideview};
use crate::zelda2::text_table::{TextIds, TextTable};

use imgui::{MouseButton, TableColumnFlags, TableColumnSetup, TableFlags};

fn weight(name: &str, weight: f32) -> TableColumnSetup<&str> {
    TableColumnSetup {
        name,
        flags: TableColumnFlags::WIDTH_FIXED,
        init_width_or_weight: weight,
        ..Default::default()
    }
}

fn popup_width(popup: bool, width: f32) -> f32 {
    if popup {
        width
    } else {
        -1.0
    }
}

impl GuiTree for config::SideviewAreas {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        let name = &self.name;
        ui.tree_node_config(format!("{name}##{path}")).build(|| {
            for index in 0..self.length {
                let i = if self.is_background_layer {
                    index + 1
                } else {
                    index
                };
                let path = format!("{path}/{i}");
                edit_tree_node(ui, &format!("Area {i}"), &path, project);
                if let Some(_token) = ui.begin_popup_context_item() {
                    result.menu(ui, &path);
                }
            }
        });
        result
    }
}

macro_rules! str_id {
    ($popup:expr, $label:literal) => {
        if $popup == false {
            concat!("##", $label)
        } else {
            $label
        }
    };
}

pub struct SideviewEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    base: String,
    area: u8,
    screen: u8,
    chr: Option<Address>,
    background: String,
    sideview: Sideview,
    decompressor: Decompressor,
    drag_helper: DragHelper,
    enemy_list: usize,
    scale: f32,
    need_update: bool,
    objects: IndexMap<u8, Object>,
    area_names: IndexMap<u8, String>,
    world: [u8; 4],
    town_code: [u16; 4],
    sequence: usize,
    spawn: Option<Box<dyn Gui>>,
}

impl SideviewEditor {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn new(sv: &Sideview, path: &str) -> Result<Box<dyn Gui>> {
        let (base, area) = path.rsplit_once('/').expect("sideview path");
        Ok(Box::new(SideviewEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            base: base.into(),
            area: area.parse()?,
            screen: 0,
            chr: None,
            background: "background".into(),
            sideview: sv.clone(),
            decompressor: Decompressor::new(),
            drag_helper: DragHelper::default(),
            enemy_list: 0,
            scale: 2.0,
            need_update: true,
            objects: IndexMap::default(),
            area_names: IndexMap::default(),
            world: [0; 4],
            town_code: [0; 4],
            sequence: 0,
            spawn: None,
        }))
    }

    fn refresh_objects(&mut self, project: &Project) -> Result<()> {
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        let render = project.config.get::<RenderInfo>(&config.render_info)?;
        self.objects.clear();
        self.objects.extend(render.small.clone());
        if self.sideview.map.objset == 0 {
            self.objects.extend(render.objset0.clone());
        } else {
            self.objects.extend(render.objset1.clone());
        }
        Ok(())
    }

    fn draw_enemy_dialog(
        &mut self,
        el: usize,
        index: usize,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        let Some(text_table) = &config.text_table else {
            return Ok(EditAction::None);
        };
        let screen = (self.sideview.enemy.data[el][index].x / 16) as usize;
        let mut world = self.world[screen];
        if world == 0 {
            // If we don't know the town world, assume world 1.
            world = 1;
        }
        let town_code = self.town_code[screen] as u8;
        let text_ids = project.data_ref::<TextIds>(&format!("{text_table}/{world}:ids"))?;
        let dialog = text_ids.get_text_ids(self.sideview.enemy.data[el][index].kind, town_code);
        if dialog == (None, None, None) {
            return Ok(EditAction::None);
        }
        let (dialog1, dialog2, conditions) = dialog;
        let text_table = project.data_ref::<TextTable>(&format!("{text_table}/{world}"))?;

        let mut action = EditAction::None;
        if let Some(_table) = ui.begin_table_header_with_flags(
            "dialogs",
            [weight("Text ID", 150.0), weight("Dialog", 650.0)],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            if let Some(mut dialog) = self.sideview.enemy.data[el][index].dialog.or(dialog1) {
                ui.table_next_row();
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                if ui.input_scalar("##dialog1", &mut dialog).step(1).build() {
                    self.sideview.enemy.data[el][index].dialog = Some(dialog);
                    action.set(EditAction::Update);
                }
                width.end();
                ui.table_next_column();
                if let Some(text) = text_table.data.get(&dialog) {
                    ui.text(text);
                    tooltip(text, ui);
                } else {
                    ui.text(format!("No dialog {dialog}"));
                }
            }
            if let Some(mut dialog) = self.sideview.enemy.data[el][index].dialog2.or(dialog2) {
                ui.table_next_row();
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                if ui.input_scalar("##dialog2", &mut dialog).step(1).build() {
                    self.sideview.enemy.data[el][index].dialog2 = Some(dialog);
                    action.set(EditAction::Update);
                }
                width.end();
                ui.table_next_column();
                if let Some(text) = text_table.data.get(&dialog) {
                    ui.text(text);
                    tooltip(text, ui);
                } else {
                    ui.text(format!("No dialog {dialog}"));
                }
            }
        }
        if let Some(mut condition) = self.sideview.enemy.data[el][index].condition.or(conditions) {
            if let Some(_table) = ui.begin_table_header_with_flags(
                "conditions",
                [
                    weight("Conditions", 150.0),
                    weight("b7", 32.0),
                    weight("b6", 32.0),
                    weight("b5", 32.0),
                    weight("b4", 32.0),
                    weight("b3", 32.0),
                    weight("b2", 32.0),
                    weight("b1", 32.0),
                    weight("b0", 32.0),
                ],
                TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
            ) {
                ui.table_next_row();
                ui.table_next_column();
                ui.text("Bit mask");
                for i in 0..8 {
                    ui.table_next_column();
                    let mask = 1u8 << (7 - i);
                    let mut bit = (condition & mask) != 0;
                    if ui.checkbox(&format!("##b{i}"), &mut bit) {
                        condition &= !mask;
                        condition |= if bit { mask } else { 0 };
                        self.sideview.enemy.data[el][index].condition = Some(condition);
                        action.set(EditAction::Update);
                    }
                }
            }
        }
        Ok(action)
    }

    fn draw_enemy_item(
        &mut self,
        el: usize,
        index: usize,
        popup: bool,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let _id = ui.push_id_usize(index | 0xEE00);

        if !popup {
            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_COPY)) {
                action = EditAction::NewAt(index);
            }
            tooltip("Insert a new Enemy", ui);

            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_ARROW_UP)) {
                if index > 0 {
                    action = EditAction::Swap(index, index - 1);
                }
            }
            tooltip("Move Up", ui);

            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_ARROW_DOWN)) {
                if index < self.sideview.enemy.data[el].len() - 1 {
                    action = EditAction::Swap(index, index + 1);
                }
            }
            tooltip("Move Down", ui);
        }
        if !popup {
            ui.table_next_column();
        }
        {
            let _width = ui.push_item_width(popup_width(popup, 120.0));
            let y = &mut self.sideview.enemy.data[el][index].y;
            if ui
                .input_scalar(str_id!(popup, "Y Position"), y)
                .step(1)
                .build()
            {
                *y = (*y).clamp(0, 15);
                action.set(EditAction::Update);
            }
        }

        if !popup {
            ui.table_next_column();
        }
        {
            let _width = ui.push_item_width(popup_width(popup, 120.0));
            let x = &mut self.sideview.enemy.data[el][index].x;
            if ui
                .input_scalar(str_id!(popup, "X Position"), x)
                .step(1)
                .build()
            {
                *x = (*x).clamp(0, 63);
                action.set(EditAction::Update);
            }
        }

        if !popup {
            ui.table_next_column();
        }
        {
            let config = project.config.get::<config::SideviewAreas>(&self.path)?;
            let enemies = project
                .config
                .get::<EnemyGroup>(&config.enemy_group.as_ref().unwrap())?;
            let _width = ui.push_item_width(popup_width(popup, 400.0));
            let kind = &mut self.sideview.enemy.data[el][index].kind;
            if enemies
                .group
                .combo(ui, str_id!(popup, "Enemy"), kind, |k, v| {
                    format!("{:02x}: {}", k, v.name).into()
                })
            {
                action.set(EditAction::Update);
            }
            action.set(self.draw_enemy_dialog(el, index, ui, project)?);
        }
        if !popup {
            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_TRASH)) {
                action.set(EditAction::Delete(index));
            }
            tooltip("Delete", ui);
        }
        Ok(action)
    }

    fn draw_enemy_entity(
        &mut self,
        el: usize,
        index: usize,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let draw_list = ui.get_window_draw_list();
        let scale = self.scale * 16.0;
        let _id = ui.push_id_usize(0xEE00 | index);

        let ox = self.sideview.enemy.data[el][index].x;
        let oy = self.sideview.enemy.data[el][index].y;
        let kind = self.sideview.enemy.data[el][index].kind as u16;
        let x = ox as f32 * scale;
        let y = oy as f32 * scale;

        {
            let config = project.config.get::<config::SideviewAreas>(&self.path)?;
            let screen = (ox >> 4) as usize;
            let kind = kind | (self.town_code[screen] << 8);
            let image = GfxCache::get(
                project,
                &format!("{}/sprite", config.palette), // idpath of a palette group.
                &format!("{}", self.sideview.map.sprite_palette),
                GfxKind::Enemy(config.enemy_group.as_ref().cloned().unwrap(), kind),
            )?;
            image.draw_at([x + origin[0], y + origin[1]], self.scale, ui);
        }
        draw_list
            .add_rect(
                [x + scr_origin[0], y + scr_origin[1]],
                [x + scr_origin[0] + scale, y + scr_origin[1] + scale],
                Self::RED,
            )
            .thickness(2.0)
            .build();

        ui.set_cursor_pos([x + origin[0], y + origin[1]]);
        ui.invisible_button("edit", [scale, scale]);
        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                let mp = ui.io().mouse_pos;
                let mp = [mp[0] - scr_origin[0], mp[1] - scr_origin[1]];
                // Use a constant to make enemy drag and map drags unique.
                self.drag_helper.start(0xEE00 | index);
                self.drag_helper.position(0xEE00 | index, mp);
                let x = ((mp[0] / scale) as u8).clamp(0, 64);
                let y = ((mp[1] / scale) as u8).clamp(0, 12);

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
        if let Some(_token) = ui.begin_popup_context_item() {
            action.set(self.draw_enemy_item(el, index, true, ui, project)?);
            if ui.button("Copy") {
                action.set(EditAction::CopyAt(index));
            }
            ui.same_line();
            if ui.button("Delete") {
                action.set(EditAction::Delete(index));
            }
        }
        Ok(action)
    }

    fn draw_map_commands_header(
        &mut self,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        if let Some(_table) = ui.begin_table_header_with_flags(
            "map_properties",
            [
                weight("Category", 200.0),
                weight("Parameters", 350.0),
                weight("", 350.0),
                weight("", 350.0),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            ui.table_next_row();
            ui.table_next_column();
            ui.text("Map Properties");

            ui.table_next_column();
            if ui
                .input_scalar("Width", &mut self.sideview.map.width)
                .step(1)
                .build()
            {
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui
                .input_scalar("Object Set", &mut self.sideview.map.objset)
                .step(1)
                .build()
            {
                self.sideview.map.objset = self.sideview.map.objset.clamp(0, 1);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.checkbox(
                "Cursor Moves Left",
                &mut self.sideview.map.cursor_moves_left,
            ) {
                action = EditAction::Update;
            }

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Flags");

            ui.table_next_column();
            if ui.checkbox("Ceiling", &mut self.sideview.map.ceiling) {
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.checkbox("Grass", &mut self.sideview.map.grass) {
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.checkbox("Bushes", &mut self.sideview.map.bushes) {
                action = EditAction::Update;
            }

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Features");

            ui.table_next_column();
            if ui
                .input_scalar("Floor Pos", &mut self.sideview.map.floor)
                .step(1)
                .build()
            {
                self.sideview.map.floor = self.sideview.map.floor.clamp(0, 15);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui
                .input_scalar("Tile Set", &mut self.sideview.map.tileset)
                .step(1)
                .build()
            {
                self.sideview.map.tileset = self.sideview.map.tileset.clamp(0, 7);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui
                .input_scalar("BG Map", &mut self.sideview.map.background_map)
                .step(1)
                .build()
            {
                self.sideview.map.background_map = self.sideview.map.background_map.clamp(0, 7);
                action = EditAction::Update;
            }

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Palettes");

            ui.table_next_column();
            let bgpal = project
                .config
                .get::<PaletteGroup>(&format!("{}/{}", config.palette, self.background))?;
            if bgpal.group.index_combo(
                ui,
                "Background",
                &mut self.sideview.map.background_palette,
                |_, v| v.name.as_str().into(),
            ) {
                action = EditAction::Update;
            }
            ui.table_next_column();
            let sprpal = project
                .config
                .get::<PaletteGroup>(&format!("{}/sprite", config.palette))?;
            if sprpal.group.index_combo(
                ui,
                "Sprites",
                &mut self.sideview.map.sprite_palette,
                |_, v| v.name.as_str().into(),
            ) {
                action = EditAction::Update;
            }
        }
        Ok(action)
    }

    fn draw_map_command(
        &mut self,
        index: usize,
        popup: bool,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let _id = ui.push_id_usize(index);

        if !popup {
            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_COPY)) {
                action = EditAction::NewAt(index);
            }
            tooltip("Insert a new Map Command", ui);

            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_ARROW_UP)) {
                if index > 0 {
                    action = EditAction::Swap(index, index - 1);
                }
            }
            tooltip("Move Up", ui);

            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_ARROW_DOWN)) {
                if index < self.sideview.map.data.len() - 1 {
                    action = EditAction::Swap(index, index + 1);
                }
            }
            tooltip("Move Down", ui);
        }

        if !popup {
            ui.table_next_column();
        }

        let y = self.sideview.map.data[index].y;
        let (label, tip) = match y {
            13 => (str_id!(popup, "New Floor "), "New Floor"),
            14 => (str_id!(popup, "X-Skip    "), "X-Skip"),
            15 => (str_id!(popup, "Extra Obj "), "Extra Object"),
            _ => (str_id!(popup, "Y Position"), "Y Position"),
        };
        let width = ui.push_item_width(popup_width(popup, 120.0));
        let y = &mut self.sideview.map.data[index].y;
        if ui.input_scalar(label, y).step(1).build() {
            *y = (*y).clamp(0, 15);
            action = EditAction::Update;
        }
        width.end();
        let y = self.sideview.map.data[index].y;
        if !popup {
            tooltip(tip, ui);
            ui.table_next_column();
        }
        {
            let _width = ui.push_item_width(popup_width(popup, 120.0));
            let x = &mut self.sideview.map.data[index].x;
            if ui
                .input_scalar(str_id!(popup, "X Position"), x)
                .step(1)
                .build()
            {
                *x = (*x).clamp(0, 63);
                action = EditAction::Update;
            }
            if !popup {
                tooltip("X Position", ui);
                ui.table_next_column();
            }
        }
        if y < 13 {
            let _width = ui.push_item_width(popup_width(popup, 300.0));
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(_sel) = self.objects.get(kind) {
                if self
                    .objects
                    .combo(ui, str_id!(popup, "Object"), kind, |k, v| {
                        format!("{k:02x}: {}", v.name).into()
                    })
                {
                    action = EditAction::Update;
                }
            } else {
                ui.text(format!("Unknown: Object/{:02x}", kind));
            }
        } else if y == 15 {
            let _width = ui.push_item_width(popup_width(popup, 300.0));
            let config = project.config.get::<config::SideviewAreas>(&self.path)?;
            let render = project.config.get::<RenderInfo>(&config.render_info)?;
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(_sel) = render.extra.get(kind) {
                if render
                    .extra
                    .combo(ui, str_id!(popup, "Extra"), kind, |k, v| {
                        format!("{k:02x}: {}", v.name).into()
                    })
                {
                    action = EditAction::Update;
                }
            } else {
                ui.text(format!("Unknown: Extra/{:02x}", kind));
            }
        }
        if !popup {
            ui.table_next_column();
        }
        if y == 13 {
            let _width = ui.push_item_width(popup_width(popup, 120.0));
            let mut param = self.sideview.map.data[index].param & 0xF;
            let mut ceiling = self.sideview.map.data[index].param & 0x80 == 0;
            if ui
                .input_scalar(str_id!(popup, "Param"), &mut param)
                .step(1)
                .build()
            {
                param = param.clamp(0, 15);
                action = EditAction::Update;
            }
            if popup {
                ui.same_line();
            }
            if ui.checkbox("Ceiling##ceiling", &mut ceiling) {
                action = EditAction::Update;
            }
            self.sideview.map.data[index].param = if ceiling { param } else { param | 0x80 };
        } else if y != 14 {
            if self.sideview.map.data[index].kind == 0x0F {
                let _width = ui.push_item_width(popup_width(popup, 300.0));
                let item = &mut self.sideview.map.data[index].param;
                let items = project.config.get::<Items>("/global/item")?;
                if items
                    .item
                    .index_combo(ui, str_id!(popup, "Items"), item, |_k, v| {
                        v.name.as_str().into()
                    })
                {
                    action = EditAction::Update;
                }
            } else {
                let _width = ui.push_item_width(popup_width(popup, 120.0));
                let p = &mut self.sideview.map.data[index].param;
                if ui.input_scalar(str_id!(popup, "Param"), p).step(1).build() {
                    *p = (*p).clamp(0, 15);
                    action = EditAction::Update;
                }
            }
        }
        if !popup {
            ui.table_next_column();
            if ui.button(&format!("{}", fa::ICON_TRASH)) {
                action = EditAction::Delete(index);
            }
            tooltip("Delete", ui);
        }
        Ok(action)
    }

    fn draw_map_command_tab(
        &mut self,
        ui: &imgui::Ui,
        project: &mut Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        ui.text("Map Header:");
        action.set(self.draw_map_commands_header(&ui, project)?);
        ui.text("Map Commands:");
        if let Some(_table) = ui.begin_table_header_with_flags(
            "map_commands",
            [
                weight("New", 32.0),
                weight("Up", 24.0),
                weight("Dn", 24.0),
                weight("Y Position", 130.0),
                weight("X Position", 130.0),
                weight("Object", 300.0),
                weight("Parameter", 300.0),
                weight("Del", 32.0),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            for i in 0..self.sideview.map.data.len() {
                ui.table_next_row();
                action.set(self.draw_map_command(i, false, ui, project)?);
            }
        }
        Ok(action)
    }

    fn draw_map_entity(
        &mut self,
        index: usize,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        let draw_list = ui.get_window_draw_list();
        let scale = self.scale * 16.0;
        let _id = ui.push_id_usize(index);

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
                Self::WHITE,
            )
            .thickness(2.0)
            .build();
        if oy == 13 {
            for i in 0..4 {
                let tick = i as f32;
                draw_list
                    .add_line(
                        [
                            x + scr_origin[0] + (tick + 1.0) * 4.0 * self.scale,
                            y + scr_origin[1] + 8.0 * self.scale,
                        ],
                        [
                            x + scr_origin[0] + tick * 4.0 * self.scale,
                            y + scr_origin[1] + 14.0 * self.scale,
                        ],
                        Self::WHITE,
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
        ui.invisible_button("edit", [scale, scale]);
        if ui.is_item_hovered() {
            let delta = ui.io().mouse_wheel as i8;
            if delta != 0 {
                self.sideview.map.data[index].param =
                    (self.sideview.map.data[index].param as i8 - delta).clamp(0, 15) as u8;
                action.set(EditAction::Update);
            }
        }
        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                let mp = ui.io().mouse_pos;
                let mp = [mp[0] - scr_origin[0], mp[1] - scr_origin[1]];
                self.drag_helper.start(index);
                self.drag_helper.position(index, mp);
                let x = ((mp[0] / scale) as u8).clamp(0, 64);
                let y = ((mp[1] / scale) as u8).clamp(0, 12);

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
        if let Some(_token) = ui.begin_popup_context_item() {
            action.set(self.draw_map_command(index, true, ui, project)?);
            if ui.button("Copy") {
                action.set(EditAction::CopyAt(index));
            }
            ui.same_line();
            if ui.button("Delete") {
                action.set(EditAction::Delete(index));
            }
        }
        Ok(action)
    }

    fn draw_enemies_tab(
        &mut self,
        el: usize,
        ui: &imgui::Ui,
        project: &mut Project,
    ) -> Result<EditAction> {
        let mut action = EditAction::None;
        if let Some(_table) = ui.begin_table_header_with_flags(
            "map_commands",
            [
                weight("New", 32.0),
                weight("Up", 24.0),
                weight("Dn", 24.0),
                weight("Y Position", 130.0),
                weight("X Position", 130.0),
                weight("Enemy", 400.0),
                weight("Del", 32.0),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            for i in 0..self.sideview.enemy.data[el].len() {
                ui.table_next_row();
                action.set(self.draw_enemy_item(el, i, false, ui, project)?);
            }
        }
        if self.sideview.enemy.data[el].is_empty() {
            if ui.button(&format!("{}", fa::ICON_COPY)) {
                action.set(EditAction::NewAt(0));
            }
            tooltip("Insert a new Enemy", ui);
        }
        Ok(action)
    }

    fn process_enemy_action(&mut self, el: usize, action: EditAction) -> bool {
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
            _ => {
                log::info!("map_commands: unhandled edit action {:?}", action);
                false
            }
        };
        changed
    }

    fn draw_map(
        &mut self,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<bool> {
        let scale = 16.0 * self.scale;
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        let background_palette = if config.area_kind == AreaKind::Palace {
            (self.sideview.map.background_palette != 0) as u8
        } else {
            self.sideview.map.background_palette
        };
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let image = GfxCache::get(
                    project,
                    &format!("{}/{}", config.palette, self.background), // idpath of a palette group.
                    &format!("{}", background_palette),
                    GfxKind::Metatile(
                        self.chr.unwrap_or(config.chr),
                        config.metatile.clone(),
                        self.decompressor.data[y][x],
                    ),
                )?;
                let xo = origin[0] + x as f32 * scale;
                let yo = origin[1] + y as f32 * scale;
                image.draw_at([xo, yo], self.scale, ui);
            }
        }

        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let item = self.decompressor.item[y][x];
                if item != 255 {
                    let image = GfxCache::get(
                        project,
                        &format!("{}/sprite", config.palette), // idpath of a palette group.
                        &format!("{}", self.sideview.map.sprite_palette),
                        GfxKind::Item(item),
                    )?;
                    let xo = origin[0] + x as f32 * scale;
                    let yo = origin[1] + y as f32 * scale;
                    image.draw_at([xo, yo], self.scale, ui);
                    let screen = x / 16;
                    if item != 0xEE && self.sideview.availability.get(screen) == Some(&false) {
                        let radius = std::cmp::max(image.width, image.height) as f32 * self.scale
                            / 2.0
                            - 2.0;
                        let draw_list = ui.get_window_draw_list();
                        let xc =
                            scr_origin[0] + (x as u32 * 16 + image.width / 2) as f32 * self.scale;
                        let yc =
                            scr_origin[1] + (y as u32 * 16 + image.height / 2) as f32 * self.scale;
                        draw_list
                            .add_circle([xc, yc], radius, Self::RED)
                            .thickness(2.0)
                            .build();
                        draw_list
                            .add_line(
                                [xc - radius, yc - radius],
                                [xc + radius, yc + radius],
                                Self::RED,
                            )
                            .thickness(2.0)
                            .build();
                    }
                }
            }
        }

        let mut changed = false;
        if let Some(_enemy_group) = &config.enemy_group {
            let mut action = EditAction::None;
            for index in 0..self.sideview.enemy.data[self.enemy_list].len() {
                action.set(self.draw_enemy_entity(
                    self.enemy_list,
                    index,
                    origin,
                    scr_origin,
                    ui,
                    project,
                )?);
            }
            changed |= self.process_enemy_action(self.enemy_list, action);
        }

        let mut action = EditAction::None;
        for i in 0..self.sideview.map.data.len() {
            action.set(self.draw_map_entity(i, origin, scr_origin, ui, project)?);
        }
        changed |= self.process_map_action(action);
        Ok(changed)
    }

    fn process_map_action(&mut self, action: EditAction) -> bool {
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
            _ => {
                log::info!("map_commands: unhandled edit action {:?}", action);
                false
            }
        };
        if changed || action == EditAction::Drag {
            self.need_update = true;
        }
        changed
    }

    fn draw_connections_tab(&mut self, ui: &imgui::Ui, project: &Project) -> Result<bool> {
        let mut changed = false;
        let labels = [
            "Screen 1 (Left)",
            "Screen 2 (Down)",
            "Screen 3 (Up)",
            "Screen 4 (Right)",
        ];
        if !self.sideview.connection.is_empty() {
            ui.text("Connection Table:");
            if let Some(_table) = ui.begin_table_header_with_flags(
                "connections",
                [
                    weight("Exit", 200.0),
                    weight("Destination", 350.0),
                    weight("Screen", 200.0),
                    weight("Adjust Target", 150.0),
                    weight("Edit Target", 200.0),
                ],
                TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
            ) {
                for (i, c) in self.sideview.connection.iter_mut().enumerate() {
                    let _id = ui.push_id_usize(i as usize | 0xCC00);
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(labels[i]);

                    ui.table_next_column();
                    let width = ui.push_item_width(-1.0);
                    changed |= self
                        .area_names
                        .combo(ui, "##destination", &mut c.area, |_, v| v.as_str().into());
                    width.end();

                    ui.table_next_column();
                    let width = ui.push_item_width(-1.0);
                    let mut screen = c.screen as usize;
                    if ui.combo_simple_string(
                        "##screen",
                        &mut screen,
                        &["Screen 1", "Screen 2", "Screen 3", "Screen 4"],
                    ) {
                        c.screen = screen as u8;
                        changed |= true;
                    }
                    width.end();

                    ui.table_next_column();
                    if c.area != 63 && (i == 0 || i == 3) {
                        let mut target = c.point_target_back.is_some();
                        if ui.checkbox("##adjust", &mut target) {
                            changed |= true;
                            if target {
                                c.point_target_back = Some((3 - i) as u8);
                            } else {
                                c.point_target_back = None;
                            }
                        }
                    }
                    // FIXME: check elevator
                    if c.area != 63 && (i == 1 || i == 2) {
                        let mut target = c.point_target_back.is_some();
                        if ui.checkbox("##adjust", &mut target) {
                            changed |= true;
                            if target {
                                c.point_target_back = Some((2 - i) as u8);
                            } else {
                                c.point_target_back = None;
                            }
                        }
                    }
                    ui.table_next_column();
                    if c.area != 63 && ui.button("Edit") {
                        let node = format!("{}/{}", self.base, c.area);
                        if let Some(edit) = project.edits.get(&node) {
                            self.spawn = Some(edit.data.gui(&node)?);
                        }
                    }
                }
            }
        } else {
            ui.text(format!("No connections for {}", self.path));
        }

        if !self.sideview.door.is_empty() {
            ui.text("\n\nDoor Table:");
            if let Some(_table) = ui.begin_table_header_with_flags(
                "connections",
                [
                    weight("Door", 200.0),
                    weight("Destination", 350.0),
                    weight("Screen", 200.0),
                    weight("Adjust Target", 150.0),
                    weight("Edit Target", 200.0),
                ],
                TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
            ) {
                for (i, c) in self.sideview.door.iter_mut().enumerate() {
                    let _id = ui.push_id_usize(i as usize | 0xCC00);
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(format!("Screen {i}"));

                    ui.table_next_column();
                    let width = ui.push_item_width(-1.0);
                    changed |= self
                        .area_names
                        .combo(ui, "##destination", &mut c.area, |_, v| v.as_str().into());
                    width.end();

                    ui.table_next_column();
                    let width = ui.push_item_width(-1.0);
                    let mut screen = c.screen as usize;
                    if ui.combo_simple_string(
                        "##screen",
                        &mut screen,
                        &["Screen 1", "Screen 2", "Screen 3", "Screen 4"],
                    ) {
                        c.screen = screen as u8;
                        changed |= true;
                    }
                    width.end();

                    ui.table_next_column();
                    if c.area != 63 {
                        let mut target = c.point_target_back.is_some();
                        if ui.checkbox("##adjust", &mut target) {
                            changed |= true;
                            if target {
                                c.point_target_back = Some(i as u8);
                            } else {
                                c.point_target_back = None;
                            }
                        }
                    }
                    ui.table_next_column();
                    if c.area != 63 && ui.button("Edit") {
                        let node = format!("{}/{}", self.base, c.area);
                        if let Some(edit) = project.edits.get(&node) {
                            self.spawn = Some(edit.data.gui(&node)?);
                        }
                    }
                }
            }
        }
        Ok(changed)
    }

    fn draw_availability_tab(&mut self, ui: &imgui::Ui) -> Result<bool> {
        let mut changed = false;
        for (i, a) in self.sideview.availability.iter_mut().enumerate() {
            changed |= ui.checkbox(&format!("Screen {}", i + 1), a);
        }
        Ok(changed)
    }

    fn commit(&mut self, project: &mut Project) -> Result<()> {
        project.commit(&self.path, Box::new(self.sideview.clone()))?;
        let (max_connectable_index, max_door_index, text_table) = {
            let config = project.config.get::<config::SideviewAreas>(&self.path)?;
            (
                config.max_connectable_index as u8,
                config.max_door_index as u8,
                config.text_table.clone(),
            )
        };

        for (i, c) in self.sideview.connection.iter().enumerate() {
            if c.area < max_connectable_index {
                if let Some(screen) = c.point_target_back {
                    let path = format!("{}/{}", self.base, c.area);
                    project.update_timestamp(&path)?;
                    let target = project.data_mut::<Sideview>(&path)?;
                    let screen = screen as usize;
                    target.connection[screen].area = self.area;
                    target.connection[screen].screen = i as u8;
                }
            }
        }
        for (i, c) in self.sideview.door.iter().enumerate() {
            if c.area < max_door_index {
                if let Some(screen) = c.point_target_back {
                    let path = format!("{}/{}", self.base, c.area);
                    project.update_timestamp(&path)?;
                    let target = project.data_mut::<Sideview>(&path)?;
                    let screen = screen as usize;
                    target.door[screen].area = self.area;
                    target.door[screen].screen = i as u8;
                }
            }
        }

        if let Some(text_table) = text_table.as_ref() {
            let mut worlds = IndexSet::new();
            for enemy in self.sideview.enemy.data[0].iter() {
                let screen = (enemy.x / 16) as usize;
                let mut world = self.world[screen];
                if world == 0 {
                    // If we don't know the town world, assume world 1.
                    world = 1;
                }
                let town_code = self.town_code[screen] as u8;
                let text_ids = project.data_mut::<TextIds>(&format!("{text_table}/{world}:ids"))?;
                if text_ids.set_text_ids(
                    enemy.kind,
                    town_code,
                    enemy.dialog,
                    enemy.dialog2,
                    enemy.condition,
                ) {
                    worlds.insert(world);
                }
            }
            for world in worlds.iter() {
                project.update_timestamp(&format!("{text_table}/{world}:ids"))?;
            }
        }

        project.connectivity.scan(project)?;
        self.sequence = project.connectivity.sequence();
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
        let width = ui.push_item_width(150.0);
        if ui.input_scalar("Scale", &mut self.scale).step(0.25).build() {
            self.scale = self.scale.clamp(0.25, 8.0);
        }
        width.end();

        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        if self.area_names.is_empty() {
            for index in 0..63 {
                self.area_names.insert(
                    index,
                    config
                        .area_names
                        .get(&index)
                        .as_ref()
                        .map(|name| format!("Area {index:02}: {name}"))
                        .unwrap_or_else(|| format!("Area {index:02}")),
                );
            }
            self.area_names.insert(63, "Outside".into());
        }
        if self.need_update {
            if self.sequence == 0 {
                self.sequence = project.connectivity.sequence();
            }
            if config.area_kind == AreaKind::Palace {
                if let Some(connector) = project
                    .connectivity
                    .get(&format!("{}/{}", self.path, self.screen))
                {
                    let (overworld, conn) = connector.rsplit_once('/').expect("connectivity path");
                    let conn = conn.parse::<u8>()?;
                    let overworld = project.data_ref::<Overworld>(overworld)?;
                    if let Some(palace) = overworld
                        .connection
                        .get(&conn)
                        .map(|c| c.palace.as_ref())
                        .flatten()
                    {
                        self.background = palace.palette.to_string();
                        self.chr = Some(Address::Chr(palace.chr_bank as i16 + 1, 0));
                    } else {
                        self.background = "background".to_string();
                    }
                }
            }
            for i in 0..4 {
                let screen = format!("{}/{i}", self.path);
                if let Some(connector) = project.connectivity.get(&screen) {
                    let (overworld, conn) = connector.rsplit_once('/').expect("connectivity path");
                    let conn = conn.parse::<u8>()?;
                    let ovcfg = project.config.get::<OverworldConfig>(overworld)?;
                    self.town_code[i] = ovcfg.town_code(conn).unwrap_or(0) as u16;
                    let overworld = project.data_ref::<Overworld>(overworld)?;
                    self.world[i] = overworld.connection[conn as usize].dest_world;
                } else {
                    self.town_code[i] = 0;
                    self.world[i] = 0;
                }
            }
            log::info!(
                "Detected world={:?} town_code={:?}",
                self.world,
                self.town_code
            );

            self.refresh_objects(project)?;
            self.decompressor
                .decompress(&self.path, &self.sideview, project)?;
            self.need_update = false;
        }
        if self.sequence != project.connectivity.sequence() {
            if self.changed {
                if let Some(choice) = self.error.show_choice(
                    "Connectivity Changed",
                    "\
The game map connectivity has changed which might have affected the
entrances, exits and doors of this room.

Do you want to reload this room?",
                    &["Reload", "Dismiss"],
                ) {
                    log::info!("choice = {choice}");
                    if choice == 0 {
                        self.need_update = true;
                        self.sideview = project.data_ref::<Sideview>(&self.path)?.clone();
                    }
                    self.sequence = project.connectivity.sequence();
                }
            } else {
                self.need_update = true;
                self.sideview = project.data_ref::<Sideview>(&self.path)?.clone();
                self.sequence = project.connectivity.sequence();
            }
        }

        let size = ui.content_region_avail();
        if let Some(change) = ui
            .child_window("1")
            .movable(false)
            .size([size[0], 16.0 * 16.0 * self.scale])
            .always_vertical_scrollbar(true)
            .always_horizontal_scrollbar(true)
            .build(|| {
                let origin = ui.cursor_pos();
                let scr_origin = ui.cursor_screen_pos();
                self.draw_map(origin, scr_origin, ui, project)
            })
            .transpose()?
        {
            self.changed |= change;
        }
        if let Some(_tab_bar) = ui.tab_bar("sideview_tabs") {
            if let Some(_item) = ui.tab_item("Map Commands") {
                let action = self.draw_map_command_tab(ui, project)?;
                self.changed |= self.process_map_action(action);
            }
            if let Some(_item) = ui.tab_item("Enemies") {
                let config = project.config.get::<config::SideviewAreas>(&self.path)?;
                if config.is_encounter(self.area, &project.edits)? {
                    self.sideview.enemy.data.resize_with(2, Default::default);
                    ui.radio_button("Small Encounter", &mut self.enemy_list, 0);
                    let action = self.draw_enemies_tab(0, ui, project)?;
                    self.changed |= self.process_enemy_action(0, action);
                    ui.radio_button("Large Encounter", &mut self.enemy_list, 1);
                    let action = self.draw_enemies_tab(1, ui, project)?;
                    self.changed |= self.process_enemy_action(1, action);
                } else {
                    self.sideview.enemy.data.resize_with(1, Default::default);
                    self.enemy_list = 0;
                    let action = self.draw_enemies_tab(0, ui, project)?;
                    self.changed |= self.process_enemy_action(0, action);
                }
            }
            if let Some(_item) = ui.tab_item("Connections") {
                self.changed |= self.draw_connections_tab(ui, project)?;
            }
            if let Some(_item) = ui.tab_item("Availability") {
                self.changed |= self.draw_availability_tab(ui)?;
            }
        }

        Ok(())
    }
}

impl Gui for SideviewEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Sideview##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Sideview Changed",
            "There are unsaved chagnes in the Sideview Editor.\nDo you want to discard them?",
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
