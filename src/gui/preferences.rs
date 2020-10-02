use imgui;
use imgui::im_str;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::fs::File;
use std::path::Path;
use ron;
use pyo3::prelude::*;

use crate::errors::*;
use crate::gui::app_context::AppContext;


#[pyclass]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Preferences {
    #[pyo3(get, set)]
    pub visible: bool,
    #[pyo3(get, set)]
    pub background: [f32; 3],
}

impl Preferences {
    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = self.visible;
        if !visible {
            return;
        }
        let mut changed = false;
        imgui::Window::new(im_str!("Preferences"))
            .opened(&mut visible)
            .build(&ui, || {
                ui.text(format!(
                    "Instantaneous FPS: {:>6.02}", 1.0 / ui.io().delta_time));

                changed |= imgui::ColorEdit::new(im_str!("Background"), &mut self.background)
                    .alpha(false)
                    .inputs(false)
                    .picker(true)
                    .build(&ui);
            });
        self.visible = visible;
        if changed {
            let file = AppContext::get().config_dir.join("preferences.ron");
            match self.save(&file) {
                Err(e) => error!("Error saving preferences: {}", e),
                _ => {}
            }
        }
    }

    pub fn from_reader(r: impl Read) -> Result<Self> {
        ron::de::from_reader(r).map_err(|e| ErrorKind::DecodeError(e).into())
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Preferences::from_reader(&file)
    }

    pub fn load() -> Result<Self> {
        let file = AppContext::get().config_dir.join("preferences.ron");
        Preferences::from_file(&file)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let pretty = ron::ser::PrettyConfig::new();
        let val = ron::ser::to_string_pretty(self, pretty).unwrap();
        std::fs::write(path, &val)?;
        Ok(())
    }
}
