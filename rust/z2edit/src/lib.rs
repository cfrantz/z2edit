use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::OnceLock;

pub mod error;
pub mod gui;
pub mod nes;
pub mod zelda2;

pub static DIRS: OnceLock<Directories> = OnceLock::new();

#[pyclass]
#[derive(Debug, Clone)]
struct Directories {
    #[pyo3(get)]
    pub install: PathBuf,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub home_dir: PathBuf,
}

impl Directories {
    pub fn get() -> &'static Self {
        DIRS.get().expect("Directories not initialized")
    }
}

#[pymethods]
impl Directories {
    #[staticmethod]
    pub fn init(install: PathBuf) -> Result<()> {
        let base = directories::BaseDirs::new().ok_or_else(|| anyhow!("Cannot find home_dir"))?;
        let project = directories::ProjectDirs::from("org", "CF207", "Z2Edit")
            .ok_or_else(|| anyhow!("Cannot find home_dir"))?;

        DIRS.set(Directories {
            install,
            data_dir: project.data_dir().into(),
            config_dir: project.config_dir().into(),
            home_dir: base.home_dir().into(),
        })
        .map_err(|_| anyhow!("Directories already initialized"))
    }

    #[staticmethod]
    #[pyo3(name = "get")]
    fn _get() -> Self {
        Self::get().clone()
    }

    fn __str__(&self) -> String {
        format!("{self:#?}")
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _z2edit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    python_gui::as_submodule_of(m)?;
    m.add_class::<Directories>()?;
    m.add_class::<nes::NesFile>()?;
    m.add_class::<nes::Address>()?;
    m.add_class::<gui::project::ProjectGui>()?;
    m.add_class::<gui::file_dialog::FileDialog>()?;
    Ok(())
}
