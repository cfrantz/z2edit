use std::rc::Rc;
use imgui;
use imgui::{im_str, ImString};


use crate::errors::*;
use crate::gui::Visibility;
use crate::gui::zelda2::Gui;
use crate::zelda2::project::{Edit, Project};


pub struct EditDetailsGui {
    visible: Visibility,
    changed: bool,
    edit: Rc<Edit>,
    label: ImString,
    user: ImString,
    comment: ImString,
    config: ImString,
}

impl EditDetailsGui {
    pub fn new(edit: Rc<Edit>) -> Result<Box<dyn Gui>> {
        let e = Rc::clone(&edit);
        let meta = e.meta.borrow();
        Ok(Box::new(EditDetailsGui {
            visible: Visibility::Visible,
            changed: false,
            edit: edit,
            label: ImString::new(&meta.label),
            user: ImString::new(&meta.user),
            comment: ImString::new(&meta.comment),
            config: ImString::new(&meta.config),
        }))
    }
}

impl Gui for EditDetailsGui {
    fn draw(&mut self, _project: &mut Project, ui: &imgui::Ui) {
        let title = im_str!("{} Details", self.edit.meta.borrow().label);
        imgui::Window::new(&title)
            .build(ui, || {
                self.changed |= imgui::InputText::new(
                    ui, im_str!("Label"), &mut self.label).resize_buffer(true).build();
                self.changed |= imgui::InputText::new(
                    ui, im_str!("User"), &mut self.user).resize_buffer(true).build();
                self.changed |= imgui::InputText::new(
                    ui, im_str!("Config"), &mut self.config).resize_buffer(true).build();
                self.changed |= imgui::InputTextMultiline::new(
                    ui, im_str!("Comment"), &mut self.comment, [0.0, 120.0]).resize_buffer(true).build();


                if ui.button(im_str!("Ok"), [0.0, 0.0]) {
                    let mut meta = self.edit.meta.borrow_mut();
                    meta.label = self.label.to_str().to_owned();
                    meta.user = self.user.to_str().to_owned();
                    meta.comment = self.comment.to_str().to_owned();
                    meta.config = self.config.to_str().to_owned();
                    self.visible = Visibility::Dispose;
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                    self.visible = Visibility::Dispose;
                }
            });
    }
    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
