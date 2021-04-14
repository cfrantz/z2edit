use std::cell::RefCell;

use imgui;
use imgui::im_str;
use rand::Rng;

use crate::errors::*;

#[derive(Debug, Default)]
pub struct ErrorDialog {
    internal: RefCell<ErrorDialogInternal>,
}

#[derive(Debug, Default)]
pub struct ErrorDialogInternal {
    title: String,
    message: String,
    error: Option<Error>,
    open: bool,
    id: i32,
}

impl ErrorDialog {
    pub fn show(&self, title: &str, message: &str, error: Option<Error>) {
        if let Some(e) = &error {
            error!("{}: {}: {:?}", title, message, e);
        } else {
            error!("{}: {}", title, message);
        }
        let mut rng = rand::thread_rng();
        let mut info = self.internal.borrow_mut();
        info.id = rng.gen();
        info.title = title.to_string();
        info.message = message.to_string();
        info.error = error;
    }

    pub fn draw(&self, ui: &imgui::Ui) {
        let mut info = self.internal.borrow_mut();
        if !info.title.is_empty() {
            if !info.open {
                ui.open_popup(&im_str!("Error: {}##{}", info.title, info.id));
                info.open = true;
            }

            ui.popup_modal(&im_str!("Error: {}##{}", info.title, info.id))
                .title_bar(true)
                .build(ui, || {
                    ui.text(&info.message);
                    ui.text("\n");
                    if let Some(error) = &info.error {
                        ui.text(im_str!("{}", error));
                    }
                    ui.separator();
                    if ui.button(im_str!("Dismiss")) {
                        info.open = false;
                        info.title.clear();
                        info.message.clear();
                        ui.close_current_popup();
                    }
                });
        }
    }
}
