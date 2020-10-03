use imgui;
use imgui::im_str;
use imgui::MenuItem;
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use std::time::Instant;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use sdl2::event::Event;
use pyo3::prelude::*;

use crate::gui::app_context::AppContext;
use crate::gui::glhelper;
use crate::gui::console::Console;
use crate::gui::preferences::Preferences;
use crate::util::pyexec::PythonExecutor;

#[pyclass(unsendable)]
pub struct App {
    running: Cell<bool>,
    #[pyo3(get, set)]
    preferences: Py<Preferences>,
    console: Rc<RefCell<Console>>,
}

impl App {
    pub fn new(py: Python) -> Self {
        App {
            running: Cell::new(false),
            preferences: Py::new(py, Preferences::load().unwrap_or_default()).unwrap(),
            console: Rc::new(RefCell::new(Console::new("Debug Console"))),
        }
    }

    pub fn pythonize(&self, _py: Python, module: &PyModule) {
        module.add_class::<App>().unwrap();
        module.add_class::<Preferences>().unwrap();
    }

    fn draw(&mut self, py: Python, ui: &imgui::Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("View"), true, || {
                MenuItem::new(im_str!("Console"))
                    .build_with_ref(ui, &mut self.console.borrow_mut().visible);
                MenuItem::new(im_str!("Preferences"))
                    .build_with_ref(ui, &mut self.preferences.borrow_mut(py).visible);
            });
        });
        self.preferences.borrow_mut(py).draw(ui);
    }

    pub fn run(slf: &PyCell<Self>, py: Python, executor: &mut PythonExecutor) {
        let context = AppContext::get();
        let mut last_frame = Instant::now();
        let mut imgui = imgui::Context::create();
        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui, &context.window);
        let renderer = Renderer::new(&mut imgui, |s| context.video.gl_get_proc_address(s) as _);

        slf.borrow().running.set(true);

        'running: loop {
            if !slf.borrow().running.get() {
                break 'running;
            }
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
            slf.borrow_mut().draw(py, &ui);
            let console = Rc::clone(&slf.borrow().console);
            console.borrow_mut().draw(executor, &ui);
            glhelper::clear_screen(&slf.borrow().preferences.borrow(py).background);

            imgui_sdl2.prepare_render(&ui, &context.window);
            renderer.render(ui);
            context.window.gl_swap_window();
        }
    }
}

#[pymethods]
impl App {
    #[getter]
    fn get_running(&self) -> bool {
        self.running.get()
    }

    #[setter]
    fn set_running(&self, value: bool) {
        self.running.set(value);
    }

}
