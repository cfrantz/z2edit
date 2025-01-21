use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass]
pub enum FileResource {
    Vanilla(),
    File(String),
}

impl Default for FileResource {
    fn default() -> Self {
        FileResource::Vanilla()
    }
}
