use crate::gui::{Gui, GuiTree};
use crate::nes::NesFile;
use crate::zelda2::config::Game;
use crate::zelda2::edit::EditList;
use anyhow::Result;
use pyo3::prelude::*;
use python_gui::UiContext;
use std::sync::Mutex;

use rfd::FileDialog;

#[pyclass]
pub struct ProjectGui {
    pub config: Game,
    pub rom: NesFile,
    pub edits: EditList,
    pub windows: Mutex<Vec<Box<dyn Gui>>>,
}

impl ProjectGui {
    fn menu(&mut self, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            self.menu_project(ui);
        });
    }

    fn menu_project(&mut self, ui: &imgui::Ui) {
        ui.menu("Project", || {
            if ui.menu_item("Save") {}
            if ui.menu_item("Save As") {}
            ui.separator();
            if ui.menu_item("Export ROM") {}
            if ui.menu_item("Export Patch") {}
            ui.separator();
            if ui.menu_item("Close") {}
        });
    }

    fn draw(&mut self, ui: &imgui::Ui) {
        ui.window("Project")
            .menu_bar(true)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| {
                self.menu(ui);
                if let Some(node) = self.config.tree_node(ui, "") {
                    if let Some(edit) = self.edits.get(&node) {
                        match edit.data.gui(&node) {
                            Ok(editor) => self.windows.lock().unwrap().push(editor),
                            Err(e) => log::error!("Create editor gui: {e}"),
                        };
                    } else {
                        log::error!("No such edit: {node}");
                    }
                }

                let mut windows = self.windows.lock().unwrap();
                let mut i = 0;
                while i < windows.len() {
                    match windows[i].draw(ui, &self) {
                        Ok(()) => {}
                        Err(e) => log::error!("Error editing: {e}"),
                    }

                    if windows[i].wants_dispose() {
                        windows.remove(i);
                    } else {
                        i += 1;
                    }
                }
            });
    }
}

#[pymethods]
impl ProjectGui {
    #[new]
    fn new(config: &str, rom: &str) -> Result<Self> {
        let config = Game::load(config)?;
        let rom = NesFile::load(rom)?;
        let mut edits = EditList::default();
        config.unpack(&rom, "", &mut edits)?;
        Ok(Self {
            config,
            rom,
            edits,
            windows: Default::default(),
        })
    }

    #[pyo3(name = "draw")]
    fn _draw(&mut self, ctx: &UiContext) {
        self.draw(ctx.ui)
    }
}
