use imgui;
use imgui::im_str;
use imgui::MenuItem;
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use std::time::Instant;
use sdl2::event::Event;
use pyo3::prelude::*;

use crate::gui::app_context::AppContext;
use crate::gui::glhelper;
use crate::gui::console::{Console, NullExecutor};
use crate::gui::preferences::Preferences;
use crate::util::pyexec::PythonExecutor;

pub struct App<'p> {
    running: bool,
    preferences: Preferences,
    console: Console,
    python: Python<'p>,
    executor: PythonExecutor<'p>,
}

impl<'p> App<'p> {
    pub fn new(python: Python<'p>) -> Self {
        App {
            running: false,
            preferences: Preferences::load().unwrap_or_default(),
            console: Console::new("Debug Console"),
            python: python,
            executor: PythonExecutor::new(python),
        }
    }

    fn draw(&mut self, ui: &imgui::Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("View"), true, || {
                MenuItem::new(im_str!("Console"))
                    .build_with_ref(ui, &mut self.console.visible);
                MenuItem::new(im_str!("Preferences"))
                    .build_with_ref(ui, &mut self.preferences.visible);
            });
        });
        self.preferences.draw(ui);
        self.console.draw(&mut self.executor, ui);
    }

    pub fn run(&mut self) {
        let context = AppContext::get();
        let mut last_frame = Instant::now();
        let mut imgui = imgui::Context::create();
        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui, &context.window);
        let renderer = Renderer::new(&mut imgui, |s| context.video.gl_get_proc_address(s) as _);

        self.running = true;

        'running: while self.running {
            let mut event_pump = context.event_pump.borrow_mut();
            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }
                match event {
                    Event::Quit {..} => { break 'running; },
                    _ => {},
                }
            }

            imgui_sdl2.prepare_frame(imgui.io_mut(), &context.window, &event_pump.mouse_state());

            let now = Instant::now();
            let delta = now - last_frame;
            imgui.io_mut().delta_time = delta.as_secs_f32();
            last_frame = now;

            let ui = imgui.frame();
            self.draw(&ui);

            glhelper::clear_screen(&self.preferences.background);
            imgui_sdl2.prepare_render(&ui, &context.window);
            renderer.render(ui);
            context.window.gl_swap_window();
        }
    }
}
