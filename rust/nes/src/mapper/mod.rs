use anyhow::Result;
use pyo3::prelude::*;
use crate::NesFile;
use crate::error::NesError;

mod uxrom;

pub fn new<'p>(py: Python<'p>, rom: &NesFile) -> Result<Py<PyAny>> {
    match rom.mapper() {
        0 => Ok(uxrom::UxROM::new(py, rom)?.into_any()),
        _ => Err(NesError::UnsupportedMapper(rom.mapper()).into())
    }
}
