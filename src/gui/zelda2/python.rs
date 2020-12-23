use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::ImString;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::Visibility;
use crate::zelda2::project::{Edit, Project};
use crate::zelda2::python::PythonScript;

pub struct PythonScriptGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    code: ImString,
}

impl PythonScriptGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;

        let script = if commit_index == -1 {
            "".to_owned()
        } else {
            let edit = edit.edit.borrow();
            let obj = edit.as_any().downcast_ref::<PythonScript>().unwrap();
            obj.code.clone()
        };

        let win_id = edit.win_id(commit_index);
        Ok(Box::new(PythonScriptGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            code: ImString::new(&script),
        }))
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let edit = Box::new(PythonScript {
            code: self.code.to_str().into(),
        });
        let i = project.commit(self.commit_index, edit, None)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }
}

impl Gui for PythonScriptGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(&im_str!("Python Script##{}", self.win_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => error!("PythonScriptGui: commit error {}", e),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = self.changed;
                ui.text("Script with local variables:");
                ui.text("  edit: EditProxy with access to the ROM.");
                changed |= imgui::InputTextMultiline::new(
                    ui,
                    im_str!("##script"),
                    &mut self.code,
                    [0.0, 480.0],
                )
                .resize_buffer(true)
                .build();

                self.changed = changed;
            });
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
        self.win_id
    }
}
