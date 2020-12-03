use imgui;
use imgui::im_str;
use imgui::MenuItem;
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use sdl2::event::Event;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::console::Console;
use crate::gui::glhelper;
use crate::gui::preferences::Preferences;
use crate::gui::zelda2::project::ProjectGui;
use crate::util::pyexec::PythonExecutor;

use crate::zelda2::project::Project;

#[pyclass(unsendable)]
pub struct App {
    running: Cell<bool>,
    #[pyo3(get, set)]
    preferences: Py<Preferences>,
    //project: Py<Project>,
    project: Vec<ProjectGui>,
    console: Rc<RefCell<Console>>,
}

impl App {
    pub fn new(py: Python) -> Result<Self> {
        Ok(App {
            running: Cell::new(false),
            preferences: Py::new(py, Preferences::load().unwrap_or_default())?,
            project: Vec::new(),
            console: Rc::new(RefCell::new(Console::new("Debug Console"))),
        })
    }

    pub fn pythonize(&self, _py: Python, module: &PyModule) -> Result<()> {
        module.add_class::<App>()?;
        module.add_class::<Preferences>()?;
        Ok(())
    }

    fn load_project(&mut self, py: Python, filename: &str) -> Result<()> {
        if filename.ends_with(".nes") {
            self.project
                .push(ProjectGui::new(py, Project::from_rom(filename)?)?);
        } else {
            self.project.push(ProjectGui::from_file(py, &filename)?);
        }
        Ok(())
    }

    fn load_dialog(&mut self, py: Python, ftype: &str) {
        loop {
            let result = nfd::open_file_dialog(Some(ftype), None).unwrap();
            match result {
                nfd::Response::Okay(path) => match self.load_project(py, &path) {
                    Err(e) => error!("Could not load {:?}: {:?}", path, e),
                    Ok(_) => break,
                },
                _ => break,
            }
        }
    }

    fn draw(&mut self, py: Python, ui: &imgui::Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"), true, || {
                if MenuItem::new(im_str!("Open Project")).build(ui) {
                    self.load_dialog(py, "z2prj");
                }
                if MenuItem::new(im_str!("Open ROM")).build(ui) {
                    self.load_dialog(py, "nes");
                }
                ui.separator();
                if MenuItem::new(im_str!("Quit")).build(ui) {
                    self.running.set(false);
                }
            });
            ui.menu(im_str!("View"), true, || {
                MenuItem::new(im_str!("Console"))
                    .build_with_ref(ui, &mut self.console.borrow_mut().visible);
                MenuItem::new(im_str!("Preferences"))
                    .build_with_ref(ui, &mut self.preferences.borrow_mut(py).visible);
            });
        });
        self.preferences.borrow_mut(py).draw(ui);
        //        self.project.borrow_mut(py).draw(ui);
        for (i, project) in self.project.iter_mut().enumerate() {
            let id = ui.push_id(i as i32);
            project.draw(py, ui);
            id.pop(ui);
        }
    }

    pub fn run(slf: &PyCell<Self>, py: Python, executor: &mut PythonExecutor) {
        let context = AppContext::get();
        let mut last_frame = Instant::now();
        let mut imgui = imgui::Context::create();

        imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

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
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
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

    fn load(&mut self, py: Python, filename: &str) -> PyResult<usize> {
        let i = self.project.len();
        self.load_project(py, filename)?;
        Ok(i)
    }
}

#[pyproto]
impl PySequenceProtocol for App {
    fn __len__(&self) -> usize {
        self.project.len()
    }

    fn __getitem__(&self, index: isize) -> PyResult<Py<Project>> {
        let len = self.project.len() as isize;
        let i = if index < 0 { len + index } else { index };

        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(self
            .project
            .get(i as usize)
            .ok_or_else(|| PyIndexError::new_err("list index out of range"))?
            .project
            .clone_ref(py))
    }
}
