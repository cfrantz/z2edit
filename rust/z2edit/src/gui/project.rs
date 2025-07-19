use anyhow::Result;
use pyo3::prelude::*;
use python_gui::docking::{Dock, DockNodeFlags};
use python_gui::UiContext;
use rfd::FileDialog;
use std::sync::Mutex;

use crate::gui::zelda2::metadata::MetadataEditor;
use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction};
use crate::zelda2::project::Project;

#[pyclass]
pub struct ProjectGui {
    #[pyo3(get)]
    project: Py<Project>,
    #[pyo3(get, set)]
    filename: String,
    windows: Mutex<Vec<Box<dyn Gui>>>,
    error: ErrorDialog,
    window_id: u32,
    dock_id: imgui::Id,
    edit_list: imgui::Id,
    edit_list_title: String,
    editor_pane: imgui::Id,
    #[pyo3(get)]
    wants_dispose: bool,
}

impl ProjectGui {
    fn menu<'p>(&mut self, py: Python<'p>, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            self.menu_project(py, ui);
        });
    }

    fn menu_project<'p>(&mut self, py: Python<'p>, ui: &imgui::Ui) {
        ui.menu("Project", || {
            if ui.menu_item("Save") {
                let result = if self.filename.is_empty() {
                    self.save_as()
                } else {
                    self.save(true)
                };
                if let Err(e) = result {
                    self.error.show(
                        "Error Saving File",
                        &format!("Error saving {:?}", self.filename),
                        e,
                    );
                }
            }
            if ui.menu_item("Save As") {
                if let Err(e) = self.save_as() {
                    self.error.show(
                        "Error Saving File",
                        &format!("Error saving {:?}", self.filename),
                        e,
                    );
                }
            }

            ui.separator();
            if ui.menu_item("Emulate") {
                if let Err(e) = self.project.borrow(py).emulate(py, None) {
                    self.error
                        .show("Error Emulating", "Error starting emulator", e);
                }
            }
            ui.separator();
            if ui.menu_item("Export ROM") {
                if let Err(e) = self.export_rom(py) {
                    self.error.show("Error Exporting ROM", "Error:", e);
                }
            }
            if ui.menu_item("Export Patch") {}
            ui.separator();
            if ui.menu_item("Close") {
                self.wants_dispose = true;
            }
        });
    }

    fn edit_tree<'p>(&self, py: Python<'p>, ui: &imgui::Ui) {
        let project = self.project.borrow(py);
        match project.config.tree_node(ui, "", &*project) {
            TreeAction::None => {}
            TreeAction::Edit(node) => self.edit(&node),
            TreeAction::Metadata(node) => self.edit_metadata(&node),
        }
    }

    fn draw<'p>(&mut self, py: Python<'p>, ui: &imgui::Ui) {
        let size = [1000.0, 900.0];
        ui.window(format!(
            "{}##{}",
            self.project.borrow(py).name,
            self.window_id
        ))
        .menu_bar(true)
        .size(size, imgui::Condition::FirstUseEver)
        .build(|| {
            if !Dock.dock_builder_has_node(self.dock_id) {
                Dock.dock_builder_remove_node(self.dock_id);
                Dock.dock_builder_add_node(self.dock_id, DockNodeFlags::DOCK_SPACE);
                Dock.dock_builder_set_node_size(self.dock_id, size);
                let (lhs, rhs) =
                    Dock.dock_builder_split_node(self.dock_id, imgui::Direction::Left, 0.35);
                self.edit_list = lhs;
                self.editor_pane = rhs;
                Dock.dock_builder_dock_window(&self.edit_list_title, self.edit_list);
                Dock.dock_builder_finish(self.dock_id);
            }

            Dock.dock_space(self.dock_id, [0.0, 0.0]);
            self.menu(py, ui);
            ui.window(&self.edit_list_title)
                .build(|| self.edit_tree(py, ui));
        });

        let mut project = self.project.borrow_mut(py);
        let mut windows = self.windows.lock().unwrap();
        let mut i = 0;
        while i < windows.len() {
            Dock.set_next_window_dock_id(self.editor_pane, imgui::Condition::Once);
            match windows[i].draw(ui, &mut *project) {
                Ok(()) => {}
                Err(e) => log::error!("Error editing: {e}"),
            }
            if let Some(window) = windows[i].spawned() {
                windows.push(window);
            }
            if windows[i].wants_dispose() {
                windows.remove(i);
            } else {
                i += 1;
            }
        }

        self.error.draw(ui);
    }
}

#[pymethods]
impl ProjectGui {
    #[new]
    fn new(project: Py<Project>) -> Result<Self> {
        let dock_val: u32 = rand::random();
        let dock_id = unsafe { std::mem::transmute::<u32, imgui::Id>(dock_val) };
        Ok(Self {
            project,
            filename: String::default(),
            windows: Default::default(),
            error: ErrorDialog::default(),
            window_id: rand::random(),
            dock_id,
            edit_list: Default::default(),
            edit_list_title: format!("Edit List##{dock_val}"),
            editor_pane: Default::default(),
            wants_dispose: false,
        })
    }

    #[getter]
    fn name(&self) -> String {
        Python::with_gil(|py| {
            let project = self.project.borrow(py);
            project.name.clone()
        })
    }

    #[pyo3(name = "draw")]
    fn _draw<'p>(&mut self, py: Python<'p>, ctx: &UiContext) {
        self.draw(py, ctx.ui)
    }

    pub fn edit(&self, node: &str) {
        Python::with_gil(|py| {
            let project = self.project.borrow(py);
            if let Some(edit) = project.edits.get(node) {
                match edit.data.gui(node) {
                    Ok(editor) => self.windows.lock().unwrap().push(editor),
                    Err(e) => log::error!("Create editor gui: {e}"),
                };
            } else {
                log::error!("No such edit: {node}");
            }
        })
    }

    pub fn edit_metadata(&self, node: &str) {
        Python::with_gil(|py| {
            let project = self.project.borrow(py);
            if let Some(edit) = project.edits.get(node) {
                match MetadataEditor::new(&edit.meta, node) {
                    Ok(editor) => self.windows.lock().unwrap().push(editor),
                    Err(e) => log::error!("Create metadata gui: {e}"),
                };
            } else {
                log::error!("No such edit: {node}");
            }
        })
    }

    fn save_as(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            if let Some(filename) = FileDialog::new()
                .set_title(format!("Save As: {}", self.project.borrow(py).name))
                .add_filter("Z2 Project", &["z2e3"])
                .add_filter("All", &["*"])
                .save_file()
            {
                self.filename = filename.to_string_lossy().into();
                self.save(true)
            } else {
                Ok(())
            }
        })
    }

    #[pyo3(signature = (filter = true))]
    fn save(&self, filter: bool) -> Result<()> {
        Python::with_gil(|py| {
            let mut project = self.project.borrow_mut(py);
            project.save(&self.filename, filter)
        })
    }

    fn export_rom<'p>(&mut self, py: Python<'p>) -> Result<()> {
        let project = self.project.borrow(py);
        if let Some(filename) = FileDialog::new()
            .set_title(format!("Export ROM: {}", project.name))
            .add_filter("NES ROM", &["nes"])
            .add_filter("All", &["*"])
            .save_file()
        {
            project.export_rom(py, &filename)
        } else {
            Ok(())
        }
    }
}
