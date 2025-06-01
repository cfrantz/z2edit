use pyo3::prelude::*;
use pyo3::py_run;

pub mod address;
mod error;
pub mod freespace;
pub mod hwpalette;
pub mod nesfile;
pub(crate) mod apu;
pub(crate) mod cpu;
pub(crate) mod ppu;
pub(crate) mod ram;
pub(crate) mod system;
pub(crate) mod stall;
pub(crate) mod controller;
pub(crate) mod mapper;
pub(crate) mod gui;

pub use address::{Address, AddressRange};
pub use error::NesError;
pub use freespace::Alloc;
pub use nesfile::NesFile;
pub use system::Nes;


#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn _nes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<Nes>()?;
    m.add_class::<NesFile>()?;
    m.add_class::<Address>()?;
    m.add_class::<AddressRange>()?;
    m.add_class::<freespace::Alloc>()?;
    m.add_class::<gui::emulator::EmulatorGui>()?;
    Ok(())
}

#[pymodule]
pub fn nes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    _nes(m)
}

pub fn as_submodule_of(parent: &str, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let name = format!("{parent}.nes");
    let module = PyModule::new(m.py(), "nes")?;
    py_run!(m.py(), module, &format!("import sys; sys.modules['{name}'] = module"));
    _nes(&module)?;
    m.add_submodule(&module)?;
    Ok(())
}
