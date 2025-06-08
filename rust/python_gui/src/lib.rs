use pyo3::ffi;
use pyo3::prelude::*;

mod audio;
pub mod docking;
pub mod font_awesome_5;
mod framework;
mod image;
mod style;

pub use audio::AudioOut;
pub use font_awesome_5 as fa;
pub use framework::{Framework, UiContext};
pub use image::{Color, Image};
pub use style::{JsonDirection, JsonStyle};

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
        gui.add_class::<JsonStyle>()?;
        gui.add_class::<JsonDirection>()?;
        m.add_submodule(gui)?;
        Ok(())
    })
}
