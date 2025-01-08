use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::error::Error;
use crate::gui::Gui;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edit {
    pub meta: Metadata,
    pub data: Box<dyn GameData>,
}

pub type EditList = IndexMap<String, Edit>;

impl Edit {
    pub fn new<T>(data: Box<T>) -> Self
    where
        T: GameData,
    {
        Self {
            meta: Metadata::default(),
            data: data as Box<dyn GameData>,
        }
    }

    pub fn data_ref<T>(&self) -> Result<&T>
    where
        T: GameData,
    {
        Ok(self.data.as_any().downcast_ref::<T>().ok_or_else(|| {
            Error::Cast(
                format!(
                    "Cannot downcast {} to {}",
                    self.data.name(),
                    std::any::type_name::<T>()
                )
                .into(),
            )
        })?)
    }
}
