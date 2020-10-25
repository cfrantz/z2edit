use imgui;
use imgui::ImStr;
use imgui::im_str;

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Hidden,
    Visible,
    Changed,
    Pending,
    Dispose,
}

impl Visibility {
    pub fn as_bool(&self) -> bool {
        match self {
            Visibility::Hidden => false,
            Visibility::Visible => true,
            Visibility::Changed => true,
            Visibility::Pending => true,
            Visibility::Dispose => false,
        }
    }

    pub fn change(&mut self, visible: bool, changed: bool) {
        if *self == Visibility::Visible {
            if !visible {
                *self = if changed { Visibility::Changed } else { Visibility::Dispose };
            }
        }
    }

    pub fn draw(&mut self, id: &ImStr, text: &str, ui: &imgui::Ui) {
        if *self == Visibility::Changed {
            ui.open_popup(id);
            *self = Visibility::Pending;
        }
        if *self == Visibility::Pending {
            ui.popup_modal(id).build(|| {
                ui.text(text);
                ui.text("\n");
                if ui.button(im_str!("Ok"), [0.0, 0.0]) {
                    *self = Visibility::Dispose;
                    ui.close_current_popup();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                    *self = Visibility::Visible;
                    ui.close_current_popup();
                }

            });
        }
    }
}
