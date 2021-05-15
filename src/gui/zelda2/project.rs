use std::path::Path;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImString, MenuItem, MouseButton, StyleColor};
use nfd;
use pyo3::prelude::*;
use rand::Rng;

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::util::DragHelper;
use crate::gui::zelda2::edit::EditDetailsGui;
use crate::gui::zelda2::enemyattr::EnemyGui;
use crate::gui::zelda2::hacks::HacksGui;
use crate::gui::zelda2::import_chr::ImportChrBankGui;
use crate::gui::zelda2::metatile::MetatileGroupGui;
use crate::gui::zelda2::overworld::OverworldGui;
use crate::gui::zelda2::palette::PaletteGui;
use crate::gui::zelda2::python::PythonScriptGui;
use crate::gui::zelda2::sideview::SideviewGui;
use crate::gui::zelda2::start::StartGui;
use crate::gui::zelda2::text_table::TextTableGui;
use crate::gui::zelda2::xp_spells::ExperienceTableGui;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::util::UTime;
use crate::zelda2::project::{Edit, EditAction, Project};

const ERROR_HEADER: [f32; 4] = [1.0, 0.4, 0.4, 0.3];
const ERROR_HEADER_HOVERED: [f32; 4] = [1.0, 0.4, 0.4, 0.8];
const ERROR_HEADER_ACTIVE: [f32; 4] = [1.0, 0.4, 0.4, 1.0];

const GRAY_HEADER: [f32; 4] = [0.5, 0.5, 0.5, 0.3];
const GRAY_HEADER_HOVERED: [f32; 4] = [0.5, 0.5, 0.5, 0.8];
const GRAY_HEADER_ACTIVE: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

const DRAG_HEADER: [f32; 4] = [0.5, 0.5, 0.9, 0.3];
const DRAG_HEADER_HOVERED: [f32; 4] = [0.5, 0.5, 0.9, 0.8];
const DRAG_HEADER_ACTIVE: [f32; 4] = [0.5, 0.5, 0.9, 1.0];

pub struct ProjectGui {
    pub visible: Visibility,
    pub changed: bool,
    pub want_autosave: bool,
    pub filename: Option<String>,
    pub widgets: Vec<Box<dyn Gui>>,
    pub project: Py<Project>,
    pub title_id: i32,
    pub dock_id: i32,
    pub history_pane: imgui::Id<'static>,
    pub editor_pane: imgui::Id<'static>,
    pub error: ErrorDialog,
    pub history_origin: [f32; 2],
    pub drag_helper: DragHelper,
    pub drag_ypos: Vec<i32>,
}

impl ProjectGui {
    pub fn new(py: Python, p: Project) -> Result<Self> {
        let mut rng = rand::thread_rng();
        Ok(ProjectGui {
            visible: Visibility::Visible,
            changed: false,
            want_autosave: false,
            filename: None,
            widgets: Vec::new(),
            project: Py::new(py, p)?,
            title_id: rng.gen(),
            dock_id: rng.gen(),
            history_pane: imgui::Id::Int(0),
            editor_pane: imgui::Id::Int(0),
            error: ErrorDialog::default(),
            history_origin: [0.0, 0.0],
            drag_helper: DragHelper::default(),
            drag_ypos: Vec::new(),
        })
    }

    pub fn from_file(py: Python, filename: &str) -> Result<Self> {
        let mut project = ProjectGui::new(py, Project::from_file(&Path::new(filename))?)?;
        project.filename = Some(filename.to_owned());
        Ok(project)
    }

    fn save(&mut self, py: Python) {
        if let Some(path) = &self.filename {
            match self.project.borrow(py).to_file(&Path::new(&path), false) {
                Err(e) => self.error.show(
                    "Save",
                    &format!("Could not save project as {:?}", path),
                    Some(e),
                ),
                Ok(_) => {
                    self.changed = false;
                    self.want_autosave = false;
                    self.remove_autosave(py);
                }
            }
        } else {
            self.save_dialog(py, false);
        }
    }
    fn save_dialog(&mut self, py: Python, save_as: bool) {
        let result = nfd::open_save_dialog(Some("z2prj"), None).unwrap();
        match result {
            nfd::Response::Okay(path) => {
                match self.project.borrow(py).to_file(&Path::new(&path), false) {
                    Err(e) => self.error.show(
                        "Save",
                        &format!("Could not save project as {:?}", path),
                        Some(e),
                    ),
                    Ok(_) => {
                        if !save_as {
                            self.filename = Some(path);
                        }
                        self.changed = false;
                        self.want_autosave = false;
                        self.remove_autosave(py);
                    }
                }
            }
            _ => {}
        }
    }

    fn autosave(&mut self, py: Python, export: Option<&str>) {
        let project = self.project.borrow(py);
        let filename = if let Some(ex) = export {
            format!("{}-export.z2prj", ex)
        } else {
            format!("{}-autosave.z2prj", project.name)
        };
        let path = AppContext::data_file(&filename);
        match project.to_file(&path, true) {
            Err(e) => error!("Could not save project as {:?}: {:?}", path, e),
            Ok(_) => {}
        }
        self.want_autosave = false;
    }

    fn remove_autosave(&self, py: Python) {
        let project = self.project.borrow(py);
        let filename = format!("{}-autosave.z2prj", project.name);
        let path = AppContext::data_file(&filename);
        let _ = std::fs::remove_file(&path);
    }

    fn export_dialog(&self, edit: &Edit) -> Option<String> {
        match nfd::open_save_dialog(Some("nes"), None).unwrap() {
            nfd::Response::Okay(path) => match edit.export(&Path::new(&path)) {
                Err(e) => {
                    self.error.show(
                        "Export",
                        &format!("Could not export ROM as {:?}", path),
                        Some(e),
                    );
                    None
                }
                Ok(sha256) => Some(sha256),
            },
            _ => None,
        }
    }

    pub fn menu(&mut self, py: Python, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            ui.menu(im_str!("Project"), || {
                if MenuItem::new(im_str!("Save")).build(ui) {
                    self.save(py);
                }
                if MenuItem::new(im_str!("Save As")).build(ui) {
                    self.save_dialog(py, true);
                }
                ui.separator();
                if MenuItem::new(im_str!("Export ROM")).build(ui) {
                    let edit = self.project.borrow(py).previous_commit(None);
                    if let Some(sha256) = self.export_dialog(&edit) {
                        self.autosave(py, Some(&sha256));
                    }
                }
                ui.separator();
                if MenuItem::new(im_str!("Close")).build(ui) {
                    self.visible.change(false, self.changed);
                }
            });
            ui.menu(im_str!("Edit"), || {
                if MenuItem::new(im_str!("Enemy Attributes")).build(ui) {
                    match EnemyGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self.error.show("GUI", "Could not create EnemyGui", Some(e)),
                    };
                }
                if MenuItem::new(im_str!("Experience & Spells")).build(ui) {
                    match ExperienceTableGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => {
                            self.error
                                .show("GUI", "Could not create ExperienceTableGui", Some(e))
                        }
                    };
                }
                if MenuItem::new(im_str!("Import CHR Bank")).build(ui) {
                    match ImportChrBankGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => {
                            self.error
                                .show("GUI", "Could not create ImportChrBankGui", Some(e))
                        }
                    };
                }
                if MenuItem::new(im_str!("Metatile Editor")).build(ui) {
                    match MetatileGroupGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => {
                            self.error
                                .show("GUI", "Could not create MetatileGroupGui", Some(e))
                        }
                    };
                }
                if MenuItem::new(im_str!("Miscellaneous Hacks")).build(ui) {
                    match HacksGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self.error.show("GUI", "Could not create HacksGui", Some(e)),
                    };
                }

                if MenuItem::new(im_str!("Overworld Editor")).build(ui) {
                    match OverworldGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self
                            .error
                            .show("GUI", "Could not create OverworldGui", Some(e)),
                    };
                }
                if MenuItem::new(im_str!("Palette")).build(ui) {
                    match PaletteGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self
                            .error
                            .show("GUI", "Could not create PaletteGui", Some(e)),
                    };
                }
                if MenuItem::new(im_str!("Python Script")).build(ui) {
                    match PythonScriptGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => {
                            self.error
                                .show("GUI", "Could not create PythonScriptGui", Some(e))
                        }
                    };
                }
                if MenuItem::new(im_str!("Sideview Editor")).build(ui) {
                    match SideviewGui::new(&self.project.borrow_mut(py), None, None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self
                            .error
                            .show("GUI", "Could not create SideviewGui", Some(e)),
                    };
                }
                if MenuItem::new(im_str!("Start Values")).build(ui) {
                    match StartGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self.error.show("GUI", "Could not create StartGui", Some(e)),
                    };
                }
                if MenuItem::new(im_str!("Text Table")).build(ui) {
                    match TextTableGui::new(&self.project.borrow_mut(py), None) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => self
                            .error
                            .show("GUI", "Could not create TextTableGui", Some(e)),
                    };
                }
            });
        });
    }

    fn postprocess_widgets(&mut self) {
        let mut i = 0;
        while i != self.widgets.len() {
            if let Some(w) = self.widgets[i].spawn() {
                self.widgets.push(w);
            } else if self.widgets[i].wants_dispose() {
                self.widgets.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn is_window_open(&self, id: u64) -> bool {
        for w in self.widgets.iter() {
            if id == w.window_id() {
                return true;
            }
        }
        false
    }

    fn process_editactions(&mut self, py: Python) {
        let mut project = self.project.borrow_mut(py);
        let mut i = 0;
        let mut first_action = usize::MAX;
        while i != project.edits.len() {
            let action = project.edits[i].action.replace(EditAction::None);
            match action {
                EditAction::None => {
                    i += 1;
                }
                EditAction::MoveTo(pos) => {
                    if !(pos == i || pos == i + 1) {
                        let v = project.edits.remove(i);
                        if pos < i {
                            project.edits.insert(pos, v);
                            if first_action == usize::MAX {
                                first_action = i;
                            }
                        } else {
                            project.edits.insert(pos - 1, v);
                            if first_action == usize::MAX {
                                first_action = pos - 1;
                            }
                        }
                        self.changed = true;
                        self.want_autosave = true;
                    }
                }
                EditAction::Swap(_, pos) => {
                    project.edits.swap(i, pos);
                    if first_action == usize::MAX {
                        first_action = if i < pos { i } else { pos };
                    }
                    self.changed = true;
                    self.want_autosave = true;
                }
                EditAction::Delete(_) => {
                    project.edits.remove(i);
                    if first_action == usize::MAX {
                        first_action = i;
                    }
                    self.changed = true;
                    self.want_autosave = true;
                }
                EditAction::Update => {
                    if first_action == usize::MAX {
                        first_action = i;
                    }
                    self.changed = true;
                    self.want_autosave = true;
                }
                _ => {
                    info!("Edit action not handled: {:?}", action);
                }
            }
        }
        if first_action != usize::MAX && first_action < project.edits.len() {
            info!("Replay requested at {}", first_action);
            match project.replay(first_action as isize, -1) {
                Err(e) => self.error.show("Edit Action", "Replay error", Some(e)),
                _ => {}
            };
        }
    }

    fn draw_edit(&mut self, py: Python, index: isize, error_state: bool, ui: &imgui::Ui) -> bool {
        let id = ui.push_id(index as i32);
        let project = self.project.borrow_mut(py);
        let edit = project.get_commit(index).unwrap();
        let len = project.edits.len() as isize;
        let meta = edit.meta.borrow();
        let is_open = self.is_window_open(edit.random_id);
        let mut error_state = error_state;
        let drag_pos = self.drag_helper.delta(index as usize);
        let cursor = ui.cursor_pos();

        let header = if Some(index as usize) == self.drag_helper.active() {
            vec![DRAG_HEADER, DRAG_HEADER_HOVERED, DRAG_HEADER_ACTIVE]
        } else if error_state || meta.skip_pack {
            vec![GRAY_HEADER, GRAY_HEADER_HOVERED, GRAY_HEADER_ACTIVE]
        } else if !edit.error.borrow().is_empty() {
            error_state = true;
            vec![ERROR_HEADER, ERROR_HEADER_HOVERED, ERROR_HEADER_ACTIVE]
        } else {
            vec![]
        };

        if Some(index as usize) == self.drag_helper.active() {
            ui.set_cursor_pos(drag_pos);
        } else {
            if let Some(d) = self.drag_helper.any_delta() {
                let mut i = match self.drag_ypos.binary_search(&(d[1] as i32)) {
                    Ok(v) => v as isize,
                    Err(v) => v as isize,
                };
                if i < 1 {
                    i = 1
                };
                if i == index && Some(i as usize - 1) != self.drag_helper.active() {
                    let _ = imgui::ColorButton::new(im_str!("##dragto"), DRAG_HEADER_HOVERED)
                        .size([200.0, 4.0])
                        .build(ui);
                }
            }
        }
        let styles = [
            StyleColor::Header,
            StyleColor::HeaderHovered,
            StyleColor::HeaderActive,
        ];
        let mut color_id = header
            .iter()
            .zip(styles.iter())
            .map(|(col, style)| ui.push_style_color(*style, *col))
            .collect::<Vec<_>>();
        let hdr = imgui::CollapsingHeader::new(&ImString::new(&meta.label)).build(ui);

        if ui.is_item_active() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                self.drag_helper.start(index as usize);
                let mp = ui.io().mouse_pos;
                let pos = [
                    mp[0] - self.history_origin[0],
                    mp[1] - self.history_origin[1],
                ];
                self.drag_helper.position(index as usize, pos);
                ui.set_cursor_pos(cursor);
                let id = ui.push_id(index as i32 | 0x7D000000);
                let _ = imgui::CollapsingHeader::new(&ImString::new(&meta.label)).build(ui);
                id.pop();
            }
        } else {
            if let Some(d) = self.drag_helper.finalize(index as usize) {
                let mut i = match self.drag_ypos.binary_search(&(d[1] as i32)) {
                    Ok(v) => v,
                    Err(v) => v,
                };
                if i < 1 {
                    i = 1
                };
                info!("Dragging to {} => {:?}", d[1], i);
                edit.action.replace(EditAction::MoveTo(i));
            }
        }
        let _ = color_id.drain(..).map(|id| id.pop());

        if ui.popup_context_item(im_str!("menu")) {
            if MenuItem::new(im_str!("Edit")).enabled(!is_open).build(ui) {
                match edit.edit.borrow().gui(&project, &edit) {
                    Ok(gui) => self.widgets.push(gui),
                    Err(e) => self.error.show(
                        "GUI",
                        &format!("Could not create widget for {}", edit.edit.borrow().name()),
                        Some(e),
                    ),
                }
            }
            if MenuItem::new(im_str!("Emulate")).build(ui) {
                match edit.emulate(None) {
                    Ok(()) => {}
                    Err(e) => self
                        .error
                        .show("Emulate", "Could not start emulator", Some(e)),
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
                edit.action
                    .replace(EditAction::Swap(index as usize, index as usize - 1));
            }
            if MenuItem::new(im_str!("Move Down"))
                .enabled(index > 0 && index < len - 1)
                .build(ui)
            {
                edit.action
                    .replace(EditAction::Swap(index as usize, index as usize + 1));
            }
            if MenuItem::new(im_str!("Delete"))
                .enabled(index > 0)
                .build(ui)
            {
                edit.action.replace(EditAction::Delete(index as usize));
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
        id.pop();
        error_state
    }

    pub fn draw(&mut self, py: Python, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        let title = im_str!(
            "Project: {}##{}",
            self.project.borrow(py).name,
            self.title_id
        );
        let dock_id = imgui::Id::Int(self.dock_id);
        let window = imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1000.0, 900.0], imgui::Condition::FirstUseEver)
            .menu_bar(true)
            .begin(ui);

        if !ui.dock_builder_has_node(dock_id) {
            ui.dock_builder_remove_node(dock_id);
            ui.dock_builder_add_node(dock_id, imgui::DockNodeFlags::DOCK_SPACE);
            ui.dock_builder_set_node_size(dock_id, [1000.0, 900.0]);
            let (lhs, rhs) = ui.dock_builder_split_node(dock_id, imgui::Direction::Left, 0.20);
            self.history_pane = lhs;
            self.editor_pane = rhs;
            ui.dock_builder_dock_window(&im_str!("Edit List##{}", self.dock_id), self.history_pane);
            ui.dock_builder_finish(dock_id);
        }

        ui.dock_space(dock_id, [0.0, 0.0]);
        let before = self.project.borrow(py).changed.get();
        if let Some(token) = window {
            self.menu(py, ui);
            imgui::Window::new(&im_str!("Edit List##{}", self.dock_id)).build(ui, || {
                let editlist = ui.push_id("editlist");
                let edits = self.project.borrow(py).edits.len();
                let mut error_state = false;
                let mut drag_ypos = Vec::new();
                self.history_origin = ui.cursor_screen_pos();
                for i in 0..edits {
                    let pos = ui.cursor_pos();
                    drag_ypos.push(pos[1] as i32);
                    error_state = self.draw_edit(py, i as isize, error_state, ui);
                }
                self.drag_ypos = drag_ypos;
                editlist.pop();
            });
            token.end();

            let mut project = self.project.borrow_mut(py);
            let widgetlist = ui.push_id("widgetlist");
            for widget in self.widgets.iter_mut() {
                ui.set_next_window_dock_id(self.editor_pane, imgui::Condition::Once);
                widget.draw(&mut project, ui);
            }
            widgetlist.pop();
        }
        self.process_editactions(py);
        let after = self.project.borrow(py).changed.get();
        self.changed |= after;
        self.want_autosave |= before == false && after == true;
        if self.want_autosave {
            self.autosave(py, None);
            // An autosave means the project was updated, so refresh the widgets as well.
            self.refresh();
        }
        self.postprocess_widgets();
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        if Some(true)
            == self.visible.draw(
                im_str!("Project Changed"),
                "There are unsaved changes in the Project.\nDo you want to discard them?",
                ui,
            )
        {
            self.remove_autosave(py);
        }
    }

    pub fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }

    pub fn refresh(&mut self) {
        // FIXME: this is sub-optimal: should only have to refresh widgets
        // after the replay point.
        for widget in self.widgets.iter_mut() {
            widget.refresh();
        }
    }
}
