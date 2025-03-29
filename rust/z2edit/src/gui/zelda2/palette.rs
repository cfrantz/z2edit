use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::nes::hwpalette;
use crate::zelda2::palette::{config, PaletteGroup};
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::PaletteGroup {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        let name = &self.name;
        ui.tree_node_config(format!("{name}##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            if ui.menu_item("Edit") {
                result = Some(path.into());
            }
        }
        result
    }
}

pub struct PaletteGroupEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    palette: PaletteGroup,
}

impl PaletteGroupEditor {
    pub fn new(pg: &PaletteGroup, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(PaletteGroupEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            palette: pg.clone(),
        }))
    }

    pub fn color_selector(name: &str, ui: &imgui::Ui) -> Option<u8> {
        let mut result = None;
        ui.popup(name, || {
            if let Some(_table) = ui.begin_table("_table", 17) {
                ui.table_next_row();
                ui.table_next_column();
                for x in 0..16 {
                    ui.table_next_column();
                    ui.text(format!("{x:02x}"));
                }
                for y in 0..4 {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(format!("{y:x}0"));
                    for x in 0..16 {
                        ui.table_next_column();
                        let i = y * 16 + x;
                        let style =
                            ui.push_style_color(imgui::StyleColor::Button, hwpalette::fget(i));
                        if ui.button(format!("  ##{i}")) {
                            result = Some(i as u8);
                            ui.close_current_popup();
                        }
                        style.pop();
                    }
                }
            }
        });
        result
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.palette.clone())) {
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

        let cfg = project.config.get::<config::PaletteGroup>(&self.path)?;
        if let Some(_table) = ui.begin_table_header_with_flags(
            self.path.as_str(),
            [
                TableColumnSetup::new("Title"),
                TableColumnSetup::new("Palette 0"),
                TableColumnSetup::new("Palette 1"),
                TableColumnSetup::new("Palette 2"),
                TableColumnSetup::new("Palette 3"),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS,
        ) {
            for (group, pg) in cfg.group.iter() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text(&pg.name);
                if let Some(pal) = self.palette.group.get_mut(group) {
                    for i in 0..16 {
                        if i % 4 == 0 {
                            ui.table_next_column();
                        } else {
                            ui.same_line();
                        }
                        if let Some(color) = pal.get_mut(i) {
                            let style = ui.push_style_color(
                                imgui::StyleColor::Button,
                                hwpalette::fget(*color as usize),
                            );
                            let label = format!("{:02x}##{}g{}", color, group, i);
                            if ui.button(&label) {
                                ui.open_popup(&label);
                            }
                            style.pop();
                            if let Some(update) = Self::color_selector(&label, ui) {
                                *color = update;
                                self.changed = true;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Gui for PaletteGroupEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Palette##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Palette Changed",
            "There are unsaved chagnes in the Palette Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
