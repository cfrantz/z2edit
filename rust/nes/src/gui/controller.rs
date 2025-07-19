use crate::controller::Controller;
use crate::system::Nes;

#[derive(Clone, Debug, Default)]
pub struct ControllerDebug {
    pub visible: bool,
}

const ON: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const OFF: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn color(active: u8) -> [f32; 4] {
    if active == 0 {
        OFF
    } else {
        ON
    }
}

impl ControllerDebug {
    fn draw_controller(&self, num: usize, c: &Controller, ui: &imgui::Ui) {
        ui.group(|| {
            ui.text(format!("Controller {}", num));
            ui.text_colored(color(c.buttons & Controller::BUTTON_UP), " U");
            ui.text_colored(color(c.buttons & Controller::BUTTON_LEFT), "L");
            ui.same_line();
            ui.text_colored(color(c.buttons & Controller::BUTTON_RIGHT), "R");
            ui.same_line();
            ui.text_colored(color(c.buttons & Controller::BUTTON_SELECT), "Sel");
            ui.same_line();
            ui.text_colored(color(c.buttons & Controller::BUTTON_START), "Sta");
            ui.same_line();
            ui.text_colored(color(c.buttons & Controller::BUTTON_B), " B");
            ui.same_line();
            ui.text_colored(color(c.buttons & Controller::BUTTON_A), "A");
            ui.text_colored(color(c.buttons & Controller::BUTTON_DOWN), " D");
        });
    }

    pub fn draw(&mut self, nes: &Nes, ui: &imgui::Ui) {
        if !self.visible {
            return;
        }
        let mut visible = self.visible;
        ui.window("Controller").opened(&mut visible).build(|| {
            let ctrl = nes.controllers.lock().expect("debug controllers");
            for (i, c) in ctrl.controller.iter().enumerate() {
                if i > 0 {
                    ui.same_line();
                    ui.text("    ");
                    ui.same_line();
                }
                self.draw_controller(i, c, ui);
            }
        });
        self.visible = visible;
    }
}
