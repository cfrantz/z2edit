use anyhow::{Context, Result};
use std::path::Path;
use std::sync::OnceLock;

use indexmap::IndexMap;
use pyo3::prelude::*;
use python_gui::JsonStyle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[pyclass(eq, eq_int)]
pub enum MultiMap {
    #[default]
    Invalid,
    Screen1,
    Screen2,
    Screen3,
    Screen4,
    Door1,
    Door2,
    Door3,
    Door4,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppPreferences {
    pub vanilla_rom: String,
    pub emulator: String,
    pub flips_patcher: String,
    pub background: [f32; 3],
    pub multimap: IndexMap<MultiMap, [f32; 4]>,
    pub imgui_style: JsonStyle,
}

impl Default for AppPreferences {
    fn default() -> Self {
        AppPreferences {
            vanilla_rom: String::default(),
            emulator: "fceux".into(),
            flips_patcher: String::default(),
            background: [0.0625, 0.0625, 0.0625],
            multimap: IndexMap::from([
                (MultiMap::Invalid, [0.4, 0.4, 0.4, 0.5]),
                (MultiMap::Screen1, [0.8, 0.9, 0.0, 0.9]),
                (MultiMap::Screen2, [0.0, 1.0, 0.0, 0.9]),
                (MultiMap::Screen3, [0.0, 0.0, 1.0, 0.9]),
                (MultiMap::Screen4, [1.0, 0.0, 0.0, 0.9]),
                (MultiMap::Door1, [0.9, 0.5, 0.0, 0.9]),
                (MultiMap::Door2, [0.9, 0.5, 0.0, 0.9]),
                (MultiMap::Door3, [0.9, 0.5, 0.0, 0.9]),
                (MultiMap::Door4, [0.9, 0.5, 0.0, 0.9]),
            ]),
            imgui_style: JsonStyle::default(),
        }
    }
}

static mut APP_PREFERENCES: OnceLock<AppPreferences> = OnceLock::new();

impl AppPreferences {
    pub fn get() -> &'static Self {
        unsafe { APP_PREFERENCES.get_or_init(AppPreferences::default) }
    }

    pub fn get_mut() -> &'static mut Self {
        unsafe {
            APP_PREFERENCES
                .get_mut()
                .expect("AppPreferences not initialized")
        }
    }

    pub fn set(value: AppPreferences) {
        let _ = Self::get();
        *Self::get_mut() = value;
    }

    fn _load<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        let data =
            std::fs::read_to_string(path).with_context(|| format!("Could not read {path:?}"))?;
        let data = serde_json::from_str(&data)
            .with_context(|| format!("Could not parse preferences ({path:?})"))?;
        Self::set(data);
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) {
        match Self::_load(path) {
            Ok(()) => {}
            Err(e) => log::warn!("Error loading preferences: {e}"),
        };
    }

    fn _save<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        let data = serde_json::to_string_pretty(Self::get())?;
        std::fs::write(path, &data).with_context(|| format!("Could not write {path:?}"))?;
        Ok(())
    }

    pub fn save<P: AsRef<Path>>(path: P) {
        match Self::_save(path) {
            Ok(()) => {}
            Err(e) => log::warn!("Error saving preferences: {e}"),
        };
    }
}

#[pyclass(name = "AppPreferences")]
pub(crate) struct AppPreferencesProxy;

#[pymethods]
impl AppPreferencesProxy {
    #[new]
    fn new() -> Self {
        AppPreferencesProxy
    }

    #[staticmethod]
    fn load(path: &str) {
        AppPreferences::load(path);
    }

    #[staticmethod]
    fn save(path: &str) {
        AppPreferences::save(path);
    }

    #[getter]
    fn get_vanilla_rom(&self) -> String {
        AppPreferences::get().vanilla_rom.clone()
    }
    #[setter]
    fn set_vanilla_rom(&self, value: String) {
        AppPreferences::get_mut().vanilla_rom = value;
    }

    #[getter]
    fn get_emulator(&self) -> String {
        AppPreferences::get().emulator.clone()
    }
    #[setter]
    fn set_emulator(&self, value: String) {
        AppPreferences::get_mut().emulator = value;
    }

    #[getter]
    fn get_flips_patcher(&self) -> String {
        AppPreferences::get().flips_patcher.clone()
    }
    #[setter]
    fn set_flips_patcher(&self, value: String) {
        AppPreferences::get_mut().flips_patcher = value;
    }

    #[getter]
    fn get_background(&self) -> [f32; 3] {
        AppPreferences::get().background.clone()
    }
    #[setter]
    fn set_background(&self, value: [f32; 3]) {
        AppPreferences::get_mut().background = value;
    }

    #[getter]
    fn get_multimap(&self) -> IndexMap<MultiMap, [f32; 4]> {
        AppPreferences::get().multimap.clone()
    }
    #[setter]
    fn set_multimap(&self, value: IndexMap<MultiMap, [f32; 4]>) {
        AppPreferences::get_mut().multimap = value;
    }

    #[getter]
    fn get_imgui_style(&self) -> JsonStyle {
        AppPreferences::get().imgui_style.clone()
    }
    #[setter]
    fn set_imgui_style(&self, value: JsonStyle) {
        AppPreferences::get_mut().imgui_style = value;
    }
}
