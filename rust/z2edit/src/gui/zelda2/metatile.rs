use anyhow::Result;
use imgui::{StyleColor, TableFlags};

use crate::gui::util::{edit_tree_node, TreeAction};
use crate::gui::widgets::Combo;
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::util::tile_cache::{GfxCache, GfxKind};
use crate::zelda2::metatile::{config, MetatileGroup};
use crate::zelda2::palette;
use crate::zelda2::project::Project;
use nes::Address;

impl GuiTree for config::MetatileGroup {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        edit_tree_node(ui, &self.name, path, project)
    }
}

pub struct MetatileGroupEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    metatile: MetatileGroup,
    group: usize,
    palette: String,
    scale: f32,
    bank: Address,
    selected: usize,
}

impl MetatileGroupEditor {
    pub fn new(metatile: &MetatileGroup, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(MetatileGroupEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            metatile: metatile.clone(),
            group: 0,
            palette: String::default(),
            scale: 4.0,
            bank: Address::Chr(-1, 0),
            selected: 0,
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.metatile.clone())) {
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
        let cfg = project.config.get::<config::MetatileGroup>(&self.path)?;
        let pcfg = project
            .config
            .get::<palette::config::PaletteGroup>(&cfg.palette)?;
        if self.palette.is_empty() {
            let (k, _) = pcfg.group.get_index(0).expect("PaletteGroup empty");
            self.palette.clone_from(k);
        }
        if self.bank.bank() == Some(-1) {
            if cfg.chr.is_chr() {
                self.bank = cfg.chr;
            } else {
                self.bank = Address::Chr(3, 0);
            }
        }

        if let Some(_table) = ui.begin_table_with_flags("metatile", 2, TableFlags::BORDERS) {
            ui.table_next_row();
            ui.table_next_column();
            ui.text("Group");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.metatile
                .group
                .combo(ui, "##group", &mut self.group, |k, _| {
                    format!("Group {k}").into()
                });
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Palette");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            pcfg.group.combo(ui, "##pal", &mut self.palette, |_, v| {
                v.name.as_str().into()
            });
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Scale");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            ui.input_scalar("##scale", &mut self.scale)
                .step(1.0)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("CHR Bank");
            ui.table_next_column();
            let mut bank = self.bank.bank().expect("CHR address");
            let width = ui.push_item_width(-1.0);
            if ui.input_scalar("##bank", &mut bank).step(1).build() {
                self.bank = Address::Chr(bank, 0);
            }
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Tile IDs");
            ui.table_next_column();
            let mut tiles = self.metatile.group[self.group].tile[self.selected].to_be_bytes();
            let width = ui.push_item_width(-1.0);
            if ui
                .input_scalar_n("##tiles", &mut tiles)
                .display_format("%02X")
                .chars_hexadecimal(true)
                .build()
            {
                self.metatile.group[self.group].tile[self.selected] = u32::from_be_bytes(tiles);
                self.changed |= true;
            }
            width.end();
            if let Some(pindex) = self.metatile.group[self.group]
                .palette
                .get_mut(self.selected)
            {
                if ui.input_scalar("Palette Index", pindex).step(1).build() {
                    self.changed |= true;
                    *pindex = (*pindex).clamp(0, 3);
                }
            }
        }
        let sel_color = ui.style_color(StyleColor::TextSelectedBg);
        if let Some(_table) = ui.begin_table_with_flags("tiles", 8, TableFlags::BORDERS) {
            let group = &self.metatile.group[self.group];
            for (i, tile) in group.tile.iter().enumerate() {
                if i % 8 == 0 {
                    ui.table_next_row();
                    for j in 0..8 {
                        ui.table_next_column();
                        let _color = if self.selected == i + j {
                            Some(ui.push_style_color(StyleColor::TableHeaderBg, sel_color))
                        } else {
                            None
                        };
                        ui.table_header(&format!("{:02x}", i + j));
                    }
                    ui.table_next_row();
                }
                ui.table_next_column();

                let subpalette = group.palette.get(i).copied().unwrap_or(self.group as u8);
                /*
                let image = GfxCache::raw(
                    project,
                    self.bank,
                    &cfg.palette,  // idpath of a palette group.
                    &self.palette, // key of a full palette within a group.
                    subpalette,    // subpalette within that palette.
                    &tile.to_be_bytes(),
                )?;
                */

                let image = GfxCache::get(
                    project,
                    &cfg.palette,  // idpath of a palette group.
                    &self.palette, // key of a full palette within a group.
                    GfxKind::RawTile(self.bank, tile.to_be_bytes(), subpalette),
                )?;

                if ui.image_button(
                    &format!("##img{i}"),
                    image.imgui_id(),
                    [16.0 * self.scale, 16.0 * self.scale],
                ) {
                    self.selected = i;
                }
            }
        }

        Ok(())
    }
}

impl Gui for MetatileGroupEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("MetatileGroup##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Metatile Changed",
            "There are unsaved chagnes in the Metatile Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
