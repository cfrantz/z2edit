use imgui;
use imgui::im_str;
use imgui::MenuItem;
use imgui::{FontConfig, FontGlyphRanges, FontSource};
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
use crate::gui::fa;
use crate::gui::glhelper;
use crate::gui::preferences::PreferencesGui;
use crate::gui::project_wizard::ProjectWizardGui;
use crate::gui::zelda2::project::ProjectGui;
use crate::util::pyexec::PythonExecutor;

use crate::zelda2::project::Project;

#[pyclass(unsendable)]
pub struct App {
    running: Cell<bool>,
    #[pyo3(get, set)]
    pub preferences: Py<PreferencesGui>,
    project: Vec<ProjectGui>,
    wizard: ProjectWizardGui,
    console: Rc<RefCell<Console>>,
}

impl App {
    pub fn new(py: Python) -> Result<Self> {
        Ok(App {
            running: Cell::new(false),
            preferences: Py::new(py, PreferencesGui::default())?,
            project: Vec::new(),
            wizard: ProjectWizardGui::default(),
            console: Rc::new(RefCell::new(Console::new("Debug Console"))),
        })
    }

    pub fn pythonize(&self, _py: Python, module: &PyModule) -> Result<()> {
        module.add_class::<App>()?;
        module.add_class::<PreferencesGui>()?;
        Ok(())
    }

    pub fn load_project(&mut self, py: Python, filename: &str) -> Result<()> {
        if filename.ends_with(".nes") {
            self.project
                .push(ProjectGui::new(py, Project::from_rom(filename)?)?);
        } else {
            self.project.push(ProjectGui::from_file(py, &filename)?);
        }
        Ok(())
    }

    fn file_dialog(&self, ftype: Option<&str>) -> Option<String> {
        let result = nfd::open_file_dialog(ftype, None).expect("ProjectWizardGui::file_dialog");
        match result {
            nfd::Response::Okay(path) => Some(path),
            _ => None,
        }
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

    fn new_project_from_wizard(&mut self, py: Python) -> Result<()> {
        self.project.push(ProjectGui::new(
            py,
            Project::new(
                self.wizard.name.to_str(),
                self.wizard.rom.clone(),
                self.wizard.config(),
                self.wizard.fix,
            )?,
        )?);
        Ok(())
    }

    fn draw(&mut self, py: Python, ui: &imgui::Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"), true, || {
                if MenuItem::new(im_str!("New Project")).build(ui) {
                    self.wizard = ProjectWizardGui::new();
                    self.wizard.show();
                }
                ui.separator();
                if MenuItem::new(im_str!("Open Project")).build(ui) {
                    self.load_dialog(py, "z2prj");
                }
                if MenuItem::new(im_str!("Open ROM")).build(ui) {
                    if let Some(filename) = self.file_dialog(Some("nes")) {
                        self.wizard = ProjectWizardGui::from_filename(&filename);
                        self.wizard.show();
                    }
                }
                ui.separator();
                if MenuItem::new(im_str!("Quit")).build(ui) {
                    self.running.set(false);
                }
            });
            ui.menu(im_str!("View"), true, || {
                MenuItem::new(im_str!("Console"))
                    .build_with_ref(ui, &mut self.console.borrow_mut().visible);
                if MenuItem::new(im_str!("Preferences")).build(ui) {
                    self.preferences.borrow_mut(py).show();
                }
            });
        });
        if self.wizard.draw(ui) {
            match self.new_project_from_wizard(py) {
                Err(e) => error!("Could not create project: {:?}", e),
                Ok(_) => {}
            }
        }
        self.preferences.borrow_mut(py).draw(ui);

        for (i, project) in self.project.iter_mut().enumerate() {
            let id = ui.push_id(i as i32);
            project.draw(py, ui);
            id.pop(ui);
        }
    }

    pub fn run(slf: &Py<Self>, py: Python, executor: &mut PythonExecutor) {
        let context = AppContext::get();
        let mut last_frame = Instant::now();
        let mut imgui = imgui::Context::create();

        imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

        let font_size = 13.0;
        imgui.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../resources/fontawesome-webfont.ttf"),
                size_pixels: 16.0,
                config: Some(FontConfig {
                    glyph_ranges: FontGlyphRanges::from_slice(&[fa::ICON_MIN, fa::ICON_MAX, 0]),
                    glyph_offset: [0.0, 3.0],
                    ..FontConfig::default()
                }),
            },
        ]);

        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui, &context.window);
        let renderer = Renderer::new(&mut imgui, |s| context.video.gl_get_proc_address(s) as _);

        slf.borrow(py).running.set(true);

        'running: loop {
            if !slf.borrow(py).running.get() {
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
            slf.borrow_mut(py).draw(py, &ui);
            let console = Rc::clone(&slf.borrow(py).console);
            console.borrow_mut().draw(executor, &ui);
            glhelper::clear_screen(&AppContext::pref().background);

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
