use imgui;
use imgui::im_str;
use pyo3::prelude::*;
use ron;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::errors::*;
use crate::gui::app_context::AppContext;

#[derive(Clone, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(default)]
    #[default(_code = "[0.0, 0.1, 0.25]")]
    pub background: [f32; 3],
    #[serde(default)]
    #[default = "fceux"]
    pub emulator: String,
}

impl Preferences {
    pub fn from_reader(r: impl Read) -> Result<Self> {
        ron::de::from_reader(r).map_err(|e| ErrorKind::DecodeError(e).into())
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Preferences::from_reader(&file)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let pretty = ron::ser::PrettyConfig::new();
        let val = ron::ser::to_string_pretty(self, pretty).unwrap();
        std::fs::write(path, &val)?;
        Ok(())
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct PreferencesGui {
    #[pyo3(get, set)]
    pub visible: bool,
}

impl PreferencesGui {
    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = self.visible;
        if !visible {
            return;
        }
        let mut changed = false;
        let pref = AppContext::pref_mut();
        imgui::Window::new(im_str!("Preferences"))
            .opened(&mut visible)
            .build(&ui, || {
                ui.text(format!(
                    "Instantaneous FPS: {:>6.02}",
                    1.0 / ui.io().delta_time
                ));

                let mut emulator = imgui::ImString::new(&pref.emulator);
                if ui
                    .input_text(im_str!("Emulator"), &mut emulator)
                    .resize_buffer(true)
                    .build()
                {
                    pref.emulator = emulator.to_str().to_owned();
                    changed |= true;
                }
                changed |= imgui::ColorEdit::new(im_str!("Background"), &mut pref.background)
                    .alpha(false)
                    .inputs(false)
                    .picker(true)
                    .build(&ui);
            });
        self.visible = visible;
        if changed {
            let file = AppContext::get().config_dir.join("preferences.ron");
            match pref.save(&file) {
                Err(e) => error!("Error saving preferences: {}", e),
                _ => {}
            }
        }
    }
}

#[pymethods]
impl PreferencesGui {
    #[getter]
    fn get_background(&self) -> [f32; 3] {
        let pref = AppContext::pref();
        pref.background
    }
    #[setter]
    fn set_background(&self, value: [f32; 3]) {
        AppContext::pref_mut().background = value;
    }

    #[getter]
    fn get_emulator(&self) -> String {
        let pref = AppContext::pref();
        pref.emulator.clone()
    }
    #[setter]
    fn set_emulator(&self, value: &str) {
        AppContext::pref_mut().emulator = value.to_owned();
    }
}
