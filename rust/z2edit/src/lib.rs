use pyo3::prelude::*;

pub mod error;
pub mod gui;
pub mod nes;
pub mod zelda2;

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    python_gui::as_submodule_of(m)?;
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    m.add_class::<gui::project::ProjectGui>()?;
    Ok(())
}
