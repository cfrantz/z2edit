use anyhow::{anyhow, Context, Result};
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::OnceLock;

pub static DIRS: OnceLock<Directories> = OnceLock::new();

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(get_all)]
pub struct Directories {
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
        .map_err(|_| anyhow!("Directories already initialized"))?;
        let dirs = DIRS.get().unwrap();
        std::fs::create_dir_all(&dirs.data_dir)
            .with_context(|| format!("Creating {:?}", dirs.data_dir))?;
        std::fs::create_dir_all(&dirs.config_dir)
            .with_context(|| format!("Creating {:?}", dirs.config_dir))?;
        Ok(())
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
