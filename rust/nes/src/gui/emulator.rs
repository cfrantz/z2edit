use anyhow::Result;
use imgui::Key;
use pyo3::prelude::*;
use python_gui::{Directories, UiContext};

use sdl2::controller::Axis;
use sdl2::controller::Button;
use sdl2::event::Event;

use crate::controller::Controller;
use crate::gui::apu::ApuDebug;
use crate::gui::controller::ControllerDebug;
use crate::gui::key::{CommandKey, InputMap};
use crate::gui::memory::MemoryDebug;
use crate::gui::ppu::PpuDebug;
use crate::system::Nes;
use crate::NesFile;

#[pyclass]
pub struct EmulatorGui {
    #[pyo3(get)]
    nes: Py<Nes>,
    #[pyo3(get)]
    wants_dispose: bool,
    apu: ApuDebug,
    controller: ControllerDebug,
    memory: MemoryDebug,
    ppu: PpuDebug,
    input_map: InputMap,
    state_slot: u32,
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

    fn state_filename(&self, nes: &Nes) -> String {
        let datadir = Directories::get().data_dir.display();
        let name = nes.name.lock().unwrap();
        format!("{datadir}/{name}.state{}", self.state_slot)
    }

    fn save_state(&self, nes: &Nes) -> Result<()> {
        let state = nes.save_state();
        let filename = self.state_filename(nes);
        let state = serde_json::to_string_pretty(&state)?;
        std::fs::write(filename, state)?;
        Ok(())
    }

    fn restore_state(&self, nes: &Nes) -> Result<()> {
        let filename = self.state_filename(nes);
        let state = std::fs::read_to_string(filename)?;
        let state = serde_json::from_str(&state)?;
        nes.restore_state(&state);
        Ok(())
    }
}

#[pymethods]
impl EmulatorGui {
    #[new]
    pub fn new(nes: Py<Nes>) -> Self {
        Self {
            nes,
            wants_dispose: false,
            controller: ControllerDebug::default(),
            apu: ApuDebug::new(),
            memory: MemoryDebug::default(),
            ppu: PpuDebug::new(),
            input_map: InputMap::default(),
            state_slot: 0,
        }
    }

    #[staticmethod]
    pub fn from_file<'p>(py: Python<'p>, filename: &str) -> Result<Self> {
        let nesfile = NesFile::load(filename)?;
        let nes = Nes::new(py, nesfile)?;
        Ok(Self::new(nes))
    }

    #[getter]
    fn get_apu_debug(&self) -> bool {
        self.apu.visible
    }
    #[setter]
    fn set_apu_debug(&mut self, v: bool) {
        self.apu.visible = v;
    }

    #[getter]
    fn get_controller_debug(&self) -> bool {
        self.controller.visible
    }
    #[setter]
    fn set_controller_debug(&mut self, v: bool) {
        self.controller.visible = v;
    }

    #[getter]
    fn get_memory_debug(&self) -> bool {
        self.memory.visible
    }
    #[setter]
    fn set_memory_debug(&mut self, v: bool) {
        self.memory.visible = v;
    }

    #[getter]
    fn get_vram_debug(&self) -> bool {
        self.ppu.vram_visible
    }
    #[setter]
    fn set_vram_debug(&mut self, v: bool) {
        self.ppu.vram_visible = v;
    }

    #[getter]
    fn get_chr_debug(&self) -> bool {
        self.ppu.chr_visible
    }
    #[setter]
    fn set_chr_debug(&mut self, v: bool) {
        self.ppu.chr_visible = v;
    }

    fn handle_input<'p>(&mut self, py: Python<'p>, ctx: &UiContext) {
        let ui = ctx.ui;
        let nes = self.nes.borrow(py);
        let mut controllers = nes.controllers.lock().expect("lock controllers");
        for event in ctx.events.iter() {
            Self::handle_event(&mut controllers.controller[0], event);
        }
        if ui.is_window_focused() {
            for (ctrl, binds) in controllers
                .controller
                .iter_mut()
                .zip(self.input_map.controller.iter())
            {
                for (&key, &button) in binds.iter() {
                    Self::handle_button(ui, ctrl, key.into(), button.into());
                }
            }
        }
        drop(controllers);

        if ui.is_window_focused() {
            for (&key, &command) in self.input_map.command.iter() {
                if ui.is_key_pressed(key.into()) {
                    match command {
                        CommandKey::SystemReset => nes.reset(),
                        CommandKey::SystemPause => nes.set_pause(!nes.get_pause()),
                        CommandKey::SystemFrameStep => {
                            nes.set_pause(true);
                            nes.set_frame_step(true);
                        }
                        CommandKey::SystemSaveState => {
                            if let Err(e) = self.save_state(&nes) {
                                log::error!("Error saving state: {e}");
                            }
                        }
                        CommandKey::SystemRestoreState => {
                            if let Err(e) = self.restore_state(&nes) {
                                log::error!("Error restoring state: {e}");
                            }
                        }
                        CommandKey::SelectState0
                        | CommandKey::SelectState1
                        | CommandKey::SelectState2
                        | CommandKey::SelectState3
                        | CommandKey::SelectState4
                        | CommandKey::SelectState5
                        | CommandKey::SelectState6
                        | CommandKey::SelectState7
                        | CommandKey::SelectState8
                        | CommandKey::SelectState9 => {
                            let slot = command as u32 - CommandKey::SelectState0 as u32;
                            log::info!("Selected state slot {slot}");
                            self.state_slot = slot;
                        }
                        _ => {
                            log::error!("{command:?} not implemented");
                        }
                    }
                }
            }
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

    fn draw_debug_windows<'p>(&mut self, py: Python<'p>, ctx: &UiContext) {
        let nes = self.nes.borrow(py);
        self.controller.draw(&*nes, ctx.ui);
        self.apu.draw(&*nes, ctx.ui);
        self.memory.draw(&*nes, ctx.ui);
        self.ppu.draw(&*nes, ctx.ui);
    }
}
