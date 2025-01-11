use pyo3::ffi;
use pyo3::prelude::*;

mod framework;
mod image;
pub use framework::{Framework, UiContext};
pub use image::{Color, Image};

extern "C" {
    fn PyInit_gui() -> *mut ffi::PyObject;
}

pub fn as_submodule_of(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Python::with_gil(|py| unsafe {
        let gui = Bound::from_owned_ptr_or_err(py, PyInit_gui())?;
        let gui = gui.downcast::<PyModule>()?;
        gui.add_class::<Framework>()?;
        gui.add_class::<Color>()?;
        gui.add_class::<Image>()?;
        m.add_submodule(gui)?;
        Ok(())
    })
}
