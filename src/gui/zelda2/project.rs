use std::path::Path;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImString, MenuItem};
use nfd;
use pyo3::prelude::*;

use crate::errors::*;
use crate::gui::zelda2::edit::EditDetailsGui;
use crate::gui::zelda2::enemyattr::EnemyGui;
use crate::gui::zelda2::hacks::HacksGui;
use crate::gui::zelda2::overworld::OverworldGui;
use crate::gui::zelda2::palette::PaletteGui;
use crate::gui::zelda2::python::PythonScriptGui;
use crate::gui::zelda2::start::StartGui;
use crate::gui::zelda2::text_table::TextTableGui;
use crate::gui::zelda2::xp_spells::ExperienceTableGui;
use crate::gui::zelda2::Gui;
use crate::util::UTime;
use crate::zelda2::project::{Edit, EditAction, Project};

pub struct ProjectGui {
    pub visible: bool,
    pub filename: Option<String>,
    pub widgets: Vec<Box<dyn Gui>>,
    pub project: Py<Project>,
    pub history_pane: imgui::Id<'static>,
    pub editor_pane: imgui::Id<'static>,
}

impl ProjectGui {
    pub fn new(py: Python, p: Project) -> Result<Self> {
        Ok(ProjectGui {
            visible: true,
            filename: None,
            widgets: Vec::new(),
            project: Py::new(py, p)?,
            history_pane: imgui::Id::Int(0),
            editor_pane: imgui::Id::Int(0),
        })
    }

    pub fn from_file(py: Python, filename: &str) -> Result<Self> {
        let mut project = ProjectGui::new(py, Project::from_file(&Path::new(filename))?)?;
        project.filename = Some(filename.to_owned());
        Ok(project)
    }

    fn save(&mut self, py: Python) {
        if let Some(path) = &self.filename {
            match self.project.borrow(py).to_file(&Path::new(&path)) {
                Err(e) => error!("Could not save project as {:?}: {:?}", path, e),
                _ => {}
            }
        } else {
            self.save_dialog(py, false);
        }
    }
    fn save_dialog(&mut self, py: Python, save_as: bool) {
        let result = nfd::open_save_dialog(Some("z2prj"), None).unwrap();
        match result {
            nfd::Response::Okay(path) => match self.project.borrow(py).to_file(&Path::new(&path)) {
                Err(e) => error!("Could not save project as {:?}: {:?}", path, e),
                Ok(_) => {
                    if !save_as {
                        self.filename = Some(path);
                    }
                }
            },
            _ => {}
        }
    }

    fn export_dialog(&self, edit: &Edit) {
        match nfd::open_save_dialog(Some("nes"), None).unwrap() {
            nfd::Response::Okay(path) => match edit.export(&path) {
                Err(e) => error!("Could not export ROM as {:?}: {:?}", path, e),
                Ok(_) => {}
            },
            _ => {}
        }
    }

    pub fn menu(&mut self, py: Python, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            ui.menu(im_str!("File"), true, || {
                if MenuItem::new(im_str!("Save")).build(ui) {
                    self.save(py);
                }
                if MenuItem::new(im_str!("Save As")).build(ui) {
                    self.save_dialog(py, true);
                }
            });
            ui.menu(im_str!("Edit"), true, || {
                if MenuItem::new(im_str!("Enemy Attributes")).build(ui) {
                    match EnemyGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create EnemyGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Experience & Spells")).build(ui) {
                    match ExperienceTableGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create ExperienceTableGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Miscellaneous Hacks")).build(ui) {
                    match HacksGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create HacksGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Overworld Editor")).build(ui) {
                    match OverworldGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create OverworldGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Palette")).build(ui) {
                    match PaletteGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create PaletteGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Python Script")).build(ui) {
                    match PythonScriptGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create PythonScriptGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Start Values")).build(ui) {
                    match StartGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create StartGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Text Table")).build(ui) {
                    match TextTableGui::new(&self.project.borrow_mut(py), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create TextTableGui: {:?}", e),
                    };
                }
            });
        });
    }

    fn dispose_widgets(&mut self) {
        let mut i = 0;
        while i != self.widgets.len() {
            if self.widgets[i].wants_dispose() {
                self.widgets.remove(i);
            } else {
                i += 1;
            }
        }
    }
    fn process_editactions(&self, py: Python) {
        let mut project = self.project.borrow_mut(py);
        let mut i = 0;
        let mut first_action = 0;
        while i != project.edits.len() {
            let action = project.edits[i].action.replace(EditAction::None);
            match action {
                EditAction::None => {
                    i += 1;
                }
                EditAction::MoveTo(pos) => {
                    let pos = pos as usize;
                    project.edits.swap(i, pos);
                    if first_action == 0 {
                        first_action = if i < pos { i } else { pos };
                    }
                }
                EditAction::Delete => {
                    project.edits.remove(i);
                    if first_action == 0 {
                        first_action = i;
                    }
                }
                EditAction::Update => {
                    if first_action == 0 {
                        first_action = i;
                    }
                }
            }
        }
        if first_action != 0 {
            match project.replay(first_action as isize, -1) {
                Err(e) => error!("EditActions: replay errror {:?}", e),
                _ => {}
            };
        }
    }

    fn draw_edit(&mut self, py: Python, index: isize, ui: &imgui::Ui) {
        let id = ui.push_id(index as i32);
        let project = self.project.borrow_mut(py);
        let edit = project.get_commit(index).unwrap();
        let len = project.edits.len() as isize;
        let meta = edit.meta.borrow();
        let hdr = imgui::CollapsingHeader::new(&ImString::new(&meta.label)).build(ui);
        if ui.popup_context_item(im_str!("menu")) {
            if MenuItem::new(im_str!("Edit")).build(ui) {
                match edit.edit.borrow().gui(&project, index) {
                    Ok(gui) => self.widgets.push(gui),
                    Err(e) => error!("Error creating widget: {:?}", e),
                }
            }
            if MenuItem::new(im_str!("Export")).build(ui) {
                self.export_dialog(&edit);
            }
            if MenuItem::new(im_str!("Details")).build(ui) {
                let gui = EditDetailsGui::new(Rc::clone(&edit)).unwrap();
                self.widgets.push(gui);
            }
            if MenuItem::new(im_str!("Move Up"))
                .enabled(index > 1)
                .build(ui)
            {
                edit.action.replace(EditAction::MoveTo(index - 1));
            }
            if MenuItem::new(im_str!("Move Down"))
                .enabled(index > 0 && index < len - 1)
                .build(ui)
            {
                edit.action.replace(EditAction::MoveTo(index + 1));
            }
            if MenuItem::new(im_str!("Delete"))
                .enabled(index > 0)
                .build(ui)
            {
                edit.action.replace(EditAction::Delete);
            }
            ui.end_popup();
        }
        if hdr {
            ui.bullet_text(&im_str!(
                "By {} on {}",
                meta.user,
                UTime::datetime(meta.timestamp)
            ));
            if !meta.comment.is_empty() {
                ui.text_wrapped(&ImString::new(&meta.comment));
            }
        }
        id.pop(ui);
    }

    pub fn draw(&mut self, py: Python, ui: &imgui::Ui) {
        let mut visible = self.visible;
        let title = if let Some(f) = &self.filename {
            im_str!("Project: {}", f)
        } else {
            imgui::ImString::new("Project: unnamed")
        };
        let dock_id = imgui::Id::Str("project");
        let window = imgui::Window::new(&title)
            .opened(&mut visible)
            .menu_bar(true)
            .begin(ui);

        if !ui.dock_builder_has_node(dock_id) {
            ui.dock_builder_remove_node(dock_id);
            ui.dock_builder_add_node(dock_id, 0);
            let (lhs, rhs) = ui.dock_builder_split_node(dock_id, imgui::Direction::Left, 0.15);
            self.history_pane = lhs;
            self.editor_pane = rhs;
            ui.dock_builder_dock_window(im_str!("Edit List"), self.history_pane);
            ui.dock_builder_finish(dock_id);
        }

        ui.dock_space(dock_id, [0.0, 0.0]);
        if let Some(token) = window {
            self.menu(py, ui);
            imgui::Window::new(im_str!("Edit List")).build(ui, || {
                let editlist = ui.push_id("editlist");
                let edits = self.project.borrow(py).edits.len();
                for i in 0..edits {
                    self.draw_edit(py, i as isize, ui);
                }
                editlist.pop(ui);
            });
            token.end(ui);

            let mut project = self.project.borrow_mut(py);
            let widgetlist = ui.push_id("widgetlist");
            for widget in self.widgets.iter_mut() {
                ui.set_next_window_dock_id(self.editor_pane, imgui::Condition::Once);
                widget.draw(&mut project, ui);
            }
            widgetlist.pop(ui);
        }
        self.visible = visible;
        self.dispose_widgets();
        self.process_editactions(py);
    }
}
