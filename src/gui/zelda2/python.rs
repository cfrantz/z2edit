use std::convert::From;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::ImString;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::zelda2::project::{Edit, Project};
use crate::zelda2::python::PythonScript;

pub struct PythonScriptGui {
    visible: Visibility,
    changed: bool,
    edit: Rc<Edit>,
    code: ImString,
    filename: ImString,
    is_file: bool,
    relative_name: Option<PathBuf>,
    error: ErrorDialog,
}

impl PythonScriptGui {
    pub fn new(project: &Project, edit: Option<Rc<Edit>>) -> Result<Box<dyn Gui>> {
        if let Some(edit) = &edit {
            let e = edit.edit.borrow();
            if !e.as_any().is::<PythonScript>() {
                return Err(ErrorKind::RomDataError(format!(
                    "Expected edit '{}' to contain a PythonScript edit",
                    edit.label()
                ))
                .into());
            }
        }
        let is_new = edit.is_none();
        let edit = edit.unwrap_or_else(|| project.create_edit("PythonScript", None).unwrap());

        let script = if is_new {
            PythonScript::default()
        } else {
            let edit = edit.edit.borrow();
            edit.as_any()
                .downcast_ref::<PythonScript>()
                .unwrap()
                .clone()
        };
        let filename = if let Some(file) = &script.file {
            ImString::from(
                edit.subdir
                    .path(file.as_ref())
                    .to_string_lossy()
                    .to_string(),
            )
        } else {
            ImString::default()
        };

        Ok(Box::new(PythonScriptGui {
            visible: Visibility::Visible,
            changed: false,
            edit: edit,
            code: ImString::new(&script.code),
            filename: filename,
            is_file: script.file.is_some(),
            relative_name: script.file.map(|p| p.into()),
            error: ErrorDialog::default(),
        }))
    }

    fn file_dialog(&self, ftype: Option<&str>) -> Option<String> {
        let result = nfd::open_file_dialog(ftype, None).expect("PythonScriptGui::file_dialog");
        match result {
            nfd::Response::Okay(path) => Some(path),
            _ => None,
        }
    }

    fn load_file(&mut self) {
        let fileresult = self
            .edit
            .subdir
            .relative_path(Path::new(self.filename.to_str()));
        let filepath = match fileresult {
            Ok(Some(p)) => p,
            Ok(None) => PathBuf::from(self.filename.to_str()),
            Err(e) => {
                self.relative_name = None;
                self.error.show("Load Python Code", "Could not determine the image filepath relative to the project.\nPlease save the project first.", Some(e));
                return;
            }
        };
        info!(
            "Computing relative path: {:?} => {:?}",
            self.filename.to_str(),
            filepath
        );
        let new_path = self.edit.subdir.path(&filepath);
        self.relative_name = match new_path.canonicalize().map_err(|e| e.into()) {
            Ok(_) => Some(filepath),
            Err(e) => {
                self.error
                    .show("Load Python Code", "Could not load Python Code", Some(e));
                None
            }
        };
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let romdata = Box::new(PythonScript {
            file: self.relative_name.as_ref().map(|p| p.into()),
            code: self.code.to_string(),
        });
        project.commit(&self.edit, romdata)?;
        Ok(())
    }
}

impl Gui for PythonScriptGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = ImString::new(self.edit.title());
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self.error.show("PythonScript", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = self.changed;
                ui.text("Script with local variables:");
                ui.text("  edit: EditProxy with access to the ROM.");
                ui.text("  asm: An assembler bound to the edit.");
                ui.text("\n\n");

                if ui.radio_button_bool(im_str!("File:"), self.is_file) {
                    self.is_file = true;
                    self.code.clear();
                    self.changed |= true;
                }
                ui.same_line();
                if ui
                    .input_text(im_str!("##file"), &mut self.filename)
                    .resize_buffer(true)
                    .enter_returns_true(true)
                    .read_only(!self.is_file)
                    .build()
                {
                    self.load_file();
                    changed |= true;
                }
                ui.same_line();
                if ui.button(im_str!("Browse##file")) {
                    if let Some(filename) = self.file_dialog(None) {
                        self.filename = ImString::new(filename);
                        self.load_file();
                        changed |= true;
                    }
                }

                if ui.radio_button_bool(im_str!("Code:"), !self.is_file) {
                    self.is_file = false;
                    self.filename.clear();
                    self.relative_name = None;
                    self.changed |= true;
                }
                ui.same_line();
                changed |= imgui::InputTextMultiline::new(
                    ui,
                    im_str!("##script"),
                    &mut self.code,
                    [0.0, 480.0],
                )
                .resize_buffer(true)
                .read_only(self.is_file)
                .build();

                self.changed = changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Script Changed"),
            "There are unsaved changes in the Script Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.edit.random_id
    }
}
