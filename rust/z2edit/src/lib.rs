use anyhow::Result;
use pyo3::prelude::*;

pub mod nes;
pub mod zelda2;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn load_config(cfg: &str, rom: &str) -> Result<()> {
    use zelda2::banks::config::Game;
    let game = Game::load(cfg)?;
    let rom = nes::NesFile::load(rom)?;
    let data = game.unpack(&rom)?;
    log::info!("{data:#x?}");
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    python_gui::as_submodule_of(m)?;
    m.add_function(wrap_pyfunction!(load_config, m)?)?;
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    Ok(())
}
