use anyhow::Result;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;
use crate::zelda2::project::Project;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[pyclass(get_all, set_all)]
pub struct Metadata {
    pub label: String,
    pub user: String,
    pub timestamp: u64,
    pub comment: String,
    pub extra: IndexMap<String, String>,
}

#[typetag::serde]
pub trait GameData: std::fmt::Debug + Send + Sync + 'static {
    fn name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn gui(&self, name: &str) -> Result<Box<dyn Gui>> {
        Err(
            Error::NotImplemented(format!("Gui for {}({name})", std::any::type_name::<Self>()))
                .into(),
        )
    }
    fn to_json(&self) -> Result<String>;
    fn from_json(&mut self, json: &str) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Edit {
    #[pyo3(get)]
    pub meta: Metadata,
    pub data: Box<dyn GameData>,
}

pub type EditList = IndexMap<String, Edit>;

impl Edit {
    pub fn new<T>(data: Box<T>) -> Self
    where
        T: GameData,
    {
        Self::from_data(data as Box<dyn GameData>)
    }

    pub fn from_data(data: Box<dyn GameData>) -> Self {
        Self {
            meta: Metadata::default(),
            data,
        }
    }

    pub fn data_ref<T>(&self) -> Result<&T>
    where
        T: GameData,
    {
        Ok(self.data.as_any().downcast_ref::<T>().ok_or_else(|| {
            Error::Cast(format!(
                "Cannot downcast {} to {}",
                self.data.name(),
                std::any::type_name::<T>()
            ))
        })?)
    }

    pub fn data_mut<T>(&mut self) -> Result<&mut T>
    where
        T: GameData,
    {
        let name = self.data.name().clone();
        Ok(self.data.as_any_mut().downcast_mut::<T>().ok_or_else(|| {
            Error::Cast(format!(
                "Cannot downcast {name} to {}",
                std::any::type_name::<T>()
            ))
        })?)
    }
}

#[pymethods]
impl Edit {
    #[getter]
    fn get_name(&self) -> String {
        self.data.name()
    }
    #[getter]
    fn get_data(&self) -> Result<String> {
        self.data.to_json()
    }
    #[setter]
    fn set_data(&mut self, json: &str) -> Result<()> {
        self.data.from_json(json)
    }
}

#[derive(Debug)]
#[pyclass]
pub struct EditProxy {
    project: Py<Project>,
    path: String,
}

#[pymethods]
impl EditProxy {
    #[new]
    pub fn new(project: Py<Project>, path: String) -> EditProxy {
        EditProxy { project, path }
    }
    #[getter]
    fn get_name<'p>(&self, py: Python<'p>) -> Result<String> {
        let project = self.project.borrow(py);
        let edit = project
            .edits
            .get(&self.path)
            .ok_or_else(|| Error::NotFound(self.path.clone()))?;
        Ok(edit.data.name())
    }
    #[getter]
    fn get_data<'p>(&self, py: Python<'p>) -> Result<String> {
        let project = self.project.borrow(py);
        let edit = project
            .edits
            .get(&self.path)
            .ok_or_else(|| Error::NotFound(self.path.clone()))?;
        edit.data.to_json()
    }
    #[setter]
    fn set_data<'p>(&self, py: Python<'p>, json: &str) -> Result<()> {
        let mut project = self.project.borrow_mut(py);
        let edit = project
            .edits
            .get_mut(&self.path)
            .ok_or_else(|| Error::NotFound(self.path.clone()))?;
        edit.data.from_json(json)
    }
    #[getter]
    fn get_meta<'p>(&self, py: Python<'p>) -> Result<Metadata> {
        let project = self.project.borrow(py);
        let edit = project
            .edits
            .get(&self.path)
            .ok_or_else(|| Error::NotFound(self.path.clone()))?;
        Ok(edit.meta.clone())
    }
    #[setter]
    fn set_meta<'p>(&self, py: Python<'p>, meta: Metadata) -> Result<()> {
        let mut project = self.project.borrow_mut(py);
        let edit = project
            .edits
            .get_mut(&self.path)
            .ok_or_else(|| Error::NotFound(self.path.clone()))?;
        edit.meta = meta;
        Ok(())
    }
}
