use anyhow::Result;
use pyo3::prelude::*;
use python_gui::UiContext;
use rfd::FileDialog;
use std::sync::Mutex;

use crate::gui::{ErrorDialog, Gui, GuiTree};
use crate::zelda2::project::Project;

#[pyclass]
pub struct ProjectGui {
    #[pyo3(get)]
    project: Py<Project>,
    #[pyo3(get, set)]
    filename: String,
    windows: Mutex<Vec<Box<dyn Gui>>>,
    error: ErrorDialog,
}

impl ProjectGui {
    fn menu(&mut self, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            self.menu_project(ui);
        });
    }

    fn menu_project(&mut self, ui: &imgui::Ui) {
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
            if ui.menu_item("Export ROM") {
                if let Err(e) = self.export_rom() {
                    self.error.show("Error Exporting ROM", "Error:", e);
                }
            }
            if ui.menu_item("Export Patch") {}
            ui.separator();
            if ui.menu_item("Close") {}
        });
    }

    fn draw<'p>(&mut self, py: Python<'p>, ui: &imgui::Ui) {
        ui.window(format!("{}", self.project.borrow(py).name))
            .menu_bar(true)
            .size([1000.0, 800.0], imgui::Condition::FirstUseEver)
            .build(|| {
                self.menu(ui);
                let mut project = self.project.borrow_mut(py);
                if let Some(node) = project.config.tree_node(ui, "") {
                    if let Some(edit) = project.edits.get(&node) {
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
                    match windows[i].draw(ui, &mut *project) {
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
        self.error.draw(ui);
    }
}

#[pymethods]
impl ProjectGui {
    #[new]
    fn new(project: Py<Project>) -> Result<Self> {
        Ok(Self {
            project,
            filename: String::default(),
            windows: Default::default(),
            error: ErrorDialog::default(),
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

    fn save_as(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            if let Some(filename) = FileDialog::new()
                .set_title(format!("Save As: {}", self.project.borrow(py).name))
                .add_filter("Z2 Project", &["z2prj"])
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
            let project = self.project.borrow(py);
            project.save(&self.filename, filter)
        })
    }

    fn export_rom(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            let project = self.project.borrow(py);
            if let Some(filename) = FileDialog::new()
                .set_title(format!("Export ROM: {}", project.name))
                .add_filter("NES ROM", &["nes"])
                .add_filter("All", &["*"])
                .save_file()
            {
                project.export_rom(&filename)
            } else {
                Ok(())
            }
        })
    }
}
