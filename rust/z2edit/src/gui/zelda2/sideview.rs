use anyhow::Result;
use indexmap::IndexMap;

use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::project::Project;
use crate::zelda2::sideview::{config, Sideview, Decompressor};
use crate::zelda2::palette::config::PaletteGroup;
use crate::gui::widgets::Combo;
use crate::gui::util::{EditAction, tooltip};
use crate::zelda2::object::{Object, RenderInfo};
use crate::zelda2::items::config::Items;
use crate::util::tile_cache::GfxCache;

use imgui::{TableColumnSetup, TableColumnFlags, TableFlags};

fn weight(name: &str, weight: f32) -> TableColumnSetup<&str> {
    TableColumnSetup {
        name, 
        flags: TableColumnFlags::WIDTH_FIXED,
        init_width_or_weight: weight,
        ..Default::default()
    }
}

impl GuiTree for config::SideviewAreas {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        let name = &self.name;
        ui.tree_node_config(format!("{name}##{path}")).build(|| {
            for index in 0..self.length {
                let i = if self.is_background_layer {index+1} else {index};
                let path = format!("{path}/{i}");
                ui.tree_node_config(format!("Area {i}##{path}"))
                    .leaf(true)
                    .build(|| {});
                if let Some(_token) = ui.begin_popup_context_item() {
                    if ui.menu_item("Edit") {
                        result.replace(path);
                    }
                }
            }
        });
        result
    }
}

pub struct SideviewEditor {
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    sideview: Sideview,
    decompressor: Decompressor,
    scale: f32,
    need_update: bool,
    objects: IndexMap<u8, Object>,
}

impl SideviewEditor {
    pub fn new(sv: &Sideview, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(SideviewEditor {
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            sideview: sv.clone(),
            decompressor: Decompressor::new(),
            scale: 2.0,
            need_update: true,
            objects: IndexMap::default(),
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

    fn draw_map_commands_header(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<EditAction> {
        let mut action = EditAction::None;
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        if let Some(_table) = ui.begin_table_with_flags("map_properties", 4, TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE) {
            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Map Properties");

            ui.table_next_column();
            if ui.input_scalar("Width", &mut self.sideview.map.width).step(1).build() {
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.input_scalar("Object Set", &mut self.sideview.map.objset).step(1).build() {
                self.sideview.map.objset = self.sideview.map.objset.clamp(0, 1);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.checkbox("Cursor Moves Left", &mut self.sideview.map.cursor_moves_left) {
                action = EditAction::Update;
            }

            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Flags");

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
            ui.table_header("Features");

            ui.table_next_column();
            if ui.input_scalar("Floor Pos", &mut self.sideview.map.floor).step(1).build() {
                self.sideview.map.floor = self.sideview.map.floor.clamp(0, 15);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.input_scalar("Tile Set", &mut self.sideview.map.tileset).step(1).build() {
                self.sideview.map.tileset = self.sideview.map.tileset.clamp(0, 7);
                action = EditAction::Update;
            }
            ui.table_next_column();
            if ui.input_scalar("BG Map", &mut self.sideview.map.background_map).step(1).build() {
                self.sideview.map.background_map = self.sideview.map.background_map.clamp(0, 7);
                action = EditAction::Update;
            }

            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Palettes");

            ui.table_next_column();
            let bgpal = project.config.get::<PaletteGroup>(&format!("{}/background", config.palette))?;
            if bgpal.group.index_combo(ui, "Background", &mut self.sideview.map.background_palette, |_, v| {
                v.name.as_str().into()
            }) {
                action = EditAction::Update;
            }
            ui.table_next_column();
            let sprpal = project.config.get::<PaletteGroup>(&format!("{}/sprite", config.palette))?;
            if sprpal.group.index_combo(ui, "Sprites", &mut self.sideview.map.sprite_palette, |_, v| {
                v.name.as_str().into()
            }) {
                action = EditAction::Update;
            }
        }
        Ok(action)
    }

    fn draw_map_command(&mut self, index: usize, popup:bool, ui: &imgui::Ui, project: &mut Project) -> Result<EditAction> {
        let mut action = EditAction::None;
        let _id = ui.push_id_usize(index);

        if !popup {
            ui.table_next_column();
            if ui.button("Cp") {
                action = EditAction::NewAt(index);
            }
            tooltip("Insert a new Map Command", ui);
            
            ui.table_next_column();
            if ui.button("Up") {
                if index > 0 {
                    action = EditAction::Swap(index, index-1);
                }
            }
            tooltip("Move Up", ui);

            ui.table_next_column();
            if ui.button("Dn") {
                if index < self.sideview.map.data.len() - 1 {
                    action = EditAction::Swap(index, index+1);
                }
            }
            tooltip("Move Down", ui);
        }

        if !popup {
            ui.table_next_column();
        }

        let y = self.sideview.map.data[index].y;
        let (label, tip) = match (popup, y) {
            (true, 13) => ("New Floor ", "New Floor"),
            (true, 14) => ("X-Skip    ", "X-Skip"),
            (true, 15) => ("Extra Obj ", "Extra Object"),
            (true,  _) => ("Y Position", "Y Position"),
            (false, 13) => ("##New Floor ", "New Floor"),
            (false, 14) => ("##X-Skip    ", "X-Skip"),
            (false, 15) => ("##Extra Obj ", "Extra Object"),
            (false,  _) => ("##Y Position", "Y Position"),
        };
        let width = ui.push_item_width(100.0);
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
            let _width = ui.push_item_width(100.0);
            let x = &mut self.sideview.map.data[index].x;
            let (label, tip) = match popup {
                true => ("X Position", "X Position"),
                false => ("##X Position", "X Position"),
            };
            if ui.input_scalar(label, x).step(1).build() {
                *x = (*x).clamp(0, 63);
                action = EditAction::Update;
            }
            if !popup {
                tooltip(tip, ui);
                ui.table_next_column();
            }
        }
        if y < 13 {
            let _width = ui.push_item_width(200.0);
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(_sel) = self.objects.get(kind) {
                if self.objects.combo(ui, "##Object", kind, |_k, v| v.name.as_str().into()) {
                    action = EditAction::Update;
                }
            } else {
                ui.text(format!("Unknown: Object/{:02x}", kind));
            }
        } else if y == 15 {
            let _width = ui.push_item_width(200.0);
            let config = project.config.get::<config::SideviewAreas>(&self.path)?;
            let render = project.config.get::<RenderInfo>(&config.render_info)?;
            let kind = &mut self.sideview.map.data[index].kind;
            if let Some(_sel) = render.extra.get(kind) {
                if render.extra.combo(ui, "##Extra", kind, |_k, v| v.name.as_str().into()) {
                    action = EditAction::Update;
                }
            } else {
                ui.text(format!("Unknown: Extra/{:02x}", kind));
            }
        }
        if !popup {
            ui.table_next_column();
        }
        if y != 14 {
            if self.sideview.map.data[index].kind == 0x0F {
                let _width = ui.push_item_width(200.0);
                let item = &mut self.sideview.map.data[index].param;
                let items = project.config.get::<Items>("/global/items")?;
                if items.item.index_combo(ui, "##Items", item, |_k, v| v.name.as_str().into()) {
                    action = EditAction::Update;
                }
            } else {
                let _width = ui.push_item_width(100.0);
                let p = &mut self.sideview.map.data[index].param;
                if ui.input_scalar("##Param", p).step(1).build() {
                    *p = (*p).clamp(0, if y==13 {255} else {15});
                    action = EditAction::Update;
                }
            }
        }
        if !popup {
            ui.table_next_column();
            if ui.button("Del") {
                action = EditAction::Delete(index);
            }
            tooltip("Delete", ui);
        }
        Ok(action)
    }

    fn draw_map_command_tab(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<EditAction> {
        let mut action = EditAction::None;
        action.set(self.draw_map_commands_header(&ui, project)?);
        if let Some(_table) = ui.begin_table_header_with_flags("map_commands",

            [
            weight("New", 20.0),
            weight("Up", 20.0),
            weight("Dn", 20.0),
            weight("Y Position", 100.0),
            weight("X Position", 100.0),
            weight("Object", 200.0),
            weight("Parameter", 200.0),
            weight("Del", 20.0),
            ],

            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE) {
            for i in 0..self.sideview.map.data.len() {
                ui.table_next_row();
                action.set(self.draw_map_command(i, false, ui, project)?);
            }
        }
        Ok(action)
    }
    fn draw_map(&self, origin: [f32;2], ui: &imgui::Ui, project: &Project) -> Result<()> {
        let scale = 16.0 * self.scale;
        let config = project.config.get::<config::SideviewAreas>(&self.path)?;
        for y in 0..Decompressor::HEIGHT {
        for x in 0..Decompressor::WIDTH {
            let image = GfxCache::metatile(
                project,
                config.chr,
                &format!("{}/background", config.palette), // idpath of a palette group.
                &format!("{}", self.sideview.map.background_palette),
                &config.metatile,
                self.decompressor.data[y][x],
            )?;
            let xo = origin[0] + x as f32 * scale;
            let yo = origin[1] + y as f32 * scale;
            image.draw_at([xo, yo], self.scale, ui);
        }
        }
        Ok(())
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.sideview.clone())) {
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

        //let cfg = project.config.get::<config::SideviewAreas>(&self.path)?;
        if self.need_update {
            self.refresh_objects(project)?;
            self.decompressor.decompress(&self.path, &self.sideview, project)?;
            for s in self.decompressor.to_strings().iter() {
                eprintln!("{s}");
            }
            self.need_update = false;
        }

        let origin = ui.cursor_pos();
        self.draw_map(origin, ui, project)?;
        self.draw_map_command_tab(ui, project)?;

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
            .window(format!("Sideview##{}", self.path))
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

    fn window_id(&self) -> u64 {
        0
    }
}
