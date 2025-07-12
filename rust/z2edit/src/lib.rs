use nes;
use pyo3::prelude::*;

pub mod app_preferences;
pub mod error;
pub mod gui;
pub mod util;
pub mod zelda2;

pub use app_preferences::AppPreferences;

#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    python_gui::as_submodule_of(m)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<app_preferences::AppPreferencesProxy>()?;
    m.add_class::<app_preferences::MultiMapColor>()?;

    /*
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    m.add_class::<nes::AddressRange>()?;
    m.add_class::<nes::freespace::Alloc>()?;
    */

    m.add_class::<gui::project::ProjectGui>()?;
    m.add_class::<gui::file_dialog::FileDialog>()?;
    m.add_class::<gui::preferences::AppPreferencesGui>()?;
    m.add_class::<gui::wizard::ProjectWizardGui>()?;
    m.add_class::<zelda2::config::Config>()?;
    m.add_class::<zelda2::project::Project>()?;
    m.add_class::<zelda2::edit::Edit>()?;
    m.add_class::<zelda2::edit::EditProxy>()?;
    m.add_class::<zelda2::edit::Metadata>()?;
    m.add_class::<zelda2::text_encoding::Text>()?;

    nes::as_submodule_of("_z2edit", m)?;
    Ok(())
}
