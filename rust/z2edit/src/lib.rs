use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::OnceLock;

pub mod app_preferences;
pub mod dirs;
pub mod error;
pub mod gui;
pub mod nes;
pub mod zelda2;

pub use app_preferences::AppPreferences;
pub use dirs::Directories;

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    python_gui::as_submodule_of(m)?;
    m.add_class::<Directories>()?;
    m.add_class::<app_preferences::AppPreferencesProxy>()?;
    m.add_class::<app_preferences::MultiMap>()?;
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    m.add_class::<gui::project::ProjectGui>()?;
    m.add_class::<gui::file_dialog::FileDialog>()?;
    m.add_class::<gui::preferences::AppPreferencesGui>()?;
    Ok(())
}
