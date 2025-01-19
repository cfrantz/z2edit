use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass]
pub struct FileDialog {
    dialog: Option<rfd::FileDialog>,
}

#[pymethods]
impl FileDialog {
    #[new]
    pub fn new() -> Self {
        FileDialog {
            dialog: Some(rfd::FileDialog::new()),
        }
    }

    pub fn add_filter(&mut self, name: &str, extensions: Vec<String>) {
        self.dialog = self.dialog.take().map(|d| d.add_filter(name, &extensions));
    }

    pub fn set_directory(&mut self, name: &str) {
        self.dialog = self.dialog.take().map(|d| d.set_directory(name));
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.dialog = self.dialog.take().map(|d| d.set_file_name(name));
    }

    pub fn set_title(&mut self, name: &str) {
        self.dialog = self.dialog.take().map(|d| d.set_title(name));
    }

    pub fn set_can_create_directories(&mut self, can: bool) {
        self.dialog = self
            .dialog
            .take()
            .map(|d| d.set_can_create_directories(can));
    }

    pub fn pick_file(&mut self) -> Result<Option<PathBuf>> {
        let dialog = self
            .dialog
            .take()
            .ok_or_else(|| anyhow!("FileDialog already consumed"))?;
        Ok(dialog.pick_file())
    }

    pub fn save_file(&mut self) -> Result<Option<PathBuf>> {
        let dialog = self
            .dialog
            .take()
            .ok_or_else(|| anyhow!("FileDialog already consumed"))?;
        Ok(dialog.save_file())
    }

    pub fn pick_folder(&mut self) -> Result<Option<PathBuf>> {
        let dialog = self
            .dialog
            .take()
            .ok_or_else(|| anyhow!("FileDialog already consumed"))?;
        Ok(dialog.pick_folder())
    }
}
