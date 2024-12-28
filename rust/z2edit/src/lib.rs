use pyo3::prelude::*;

pub mod nes;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_gui::as_submodule_of(m)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    Ok(())
}
