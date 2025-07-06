use anyhow::Result;
use imgui::Key;
use pyo3::prelude::*;
use python_gui::UiContext;

use sdl2::controller::Axis;
use sdl2::controller::Button;
use sdl2::event::Event;

use crate::controller::Controller;
use crate::system::Nes;
use crate::NesFile;

#[pyclass]
pub struct EmulatorGui {
    #[pyo3(get)]
    nes: Py<Nes>,
    window_id: u32,
    #[pyo3(get)]
    wants_dispose: bool,
}

fn xbox_to_nes(xbox: &Button) -> u8 {
    match xbox {
        Button::A => Controller::BUTTON_B,
        Button::B => Controller::BUTTON_A,
        Button::Back => Controller::BUTTON_SELECT,
        Button::Start => Controller::BUTTON_START,
        Button::DPadLeft => Controller::BUTTON_LEFT,
        Button::DPadRight => Controller::BUTTON_RIGHT,
        Button::DPadUp => Controller::BUTTON_UP,
        Button::DPadDown => Controller::BUTTON_DOWN,
        _ => 0,
    }
}

impl EmulatorGui {
    fn handle_button(ui: &imgui::Ui, controller: &mut Controller, key: Key, button: u8) {
        if ui.is_key_pressed(key) {
            controller.set(button);
        }
        if ui.is_key_released(key) {
            controller.clear(button);
        }
    }

    fn handle_event(controller: &mut Controller, event: &Event) {
        match event {
            Event::ControllerButtonDown { button: b, .. } => {
                controller.set(xbox_to_nes(b));
            }
            Event::ControllerButtonUp { button: b, .. } => {
                controller.clear(xbox_to_nes(b));
            }
            Event::ControllerAxisMotion {
                axis: Axis::LeftX,
                value: v,
                ..
            } => {
                if *v < -3000 {
                    controller.set(Controller::BUTTON_LEFT);
                    controller.clear(Controller::BUTTON_RIGHT);
                } else if *v > 3000 {
                    controller.clear(Controller::BUTTON_LEFT);
                    controller.set(Controller::BUTTON_RIGHT);
                } else {
                    controller.clear(Controller::BUTTON_LEFT);
                    controller.clear(Controller::BUTTON_RIGHT);
                }
            }
            Event::ControllerAxisMotion {
                axis: Axis::LeftY,
                value: v,
                ..
            } => {
                if *v < -3000 {
                    controller.set(Controller::BUTTON_UP);
                    controller.clear(Controller::BUTTON_DOWN);
                } else if *v > 3000 {
                    controller.clear(Controller::BUTTON_UP);
                    controller.set(Controller::BUTTON_DOWN);
                } else {
                    controller.clear(Controller::BUTTON_UP);
                    controller.clear(Controller::BUTTON_DOWN);
                }
            }
            _ => {}
        }
    }
}

#[pymethods]
impl EmulatorGui {
    #[new]
    pub fn new(nes: Py<Nes>) -> Self {
        Self {
            nes,
            window_id: rand::random(),
            wants_dispose: false,
        }
    }

    #[staticmethod]
    pub fn from_file<'p>(py: Python<'p>, filename: &str) -> Result<Self> {
        let nesfile = NesFile::load(filename)?;
        let nes = Nes::new(py, nesfile)?;
        Ok(Self::new(nes))
    }

    fn handle_input<'p>(&self, py: Python<'p>, ctx: &UiContext) {
        let ui = ctx.ui;
        let nes = self.nes.borrow(py);
        let mut ctrl = nes.controllers.lock().expect("lock controllers");
        if ui.is_window_focused() {
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::UpArrow,
                Controller::BUTTON_UP,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::DownArrow,
                Controller::BUTTON_DOWN,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::LeftArrow,
                Controller::BUTTON_LEFT,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::RightArrow,
                Controller::BUTTON_RIGHT,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::LeftCtrl,
                Controller::BUTTON_B,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::LeftAlt,
                Controller::BUTTON_A,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::LeftShift,
                Controller::BUTTON_START,
            );
            Self::handle_button(
                ui,
                &mut ctrl.controller[0],
                Key::Tab,
                Controller::BUTTON_SELECT,
            );
        }
        for event in ctx.events.iter() {
            Self::handle_event(&mut ctrl.controller[0], event);
        }
    }

    fn emulate_frame<'p>(&self, py: Python<'p>, ctx: &UiContext) {
        let nes = self.nes.borrow(py);
        nes.emulate_frame(ctx.audio.expect("no audio out"));
    }

    fn draw_image<'p>(&self, py: Python<'p>, ctx: &UiContext, scale: f32, aspect: f32) {
        let nes = self.nes.borrow(py);
        let image = nes.image.lock().expect("Failed to lock image");
        image.draw_aspect(scale, aspect, ctx.ui);
    }
}
