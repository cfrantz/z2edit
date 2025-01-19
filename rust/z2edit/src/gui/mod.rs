use anyhow::Result;

mod error_dialog;
pub mod project;
mod visibility;
pub mod zelda2;
//pub mod rfd_adapter;

use error_dialog::ErrorDialog;
use project::ProjectGui;
use visibility::Visibility;

pub trait GuiTree: Send + Sync + 'static {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String>;
}

pub trait Gui: Send + Sync + 'static {
    fn draw(&mut self, ui: &imgui::Ui, project: &ProjectGui) -> Result<()>;
    fn wants_dispose(&self) -> bool;
    fn window_id(&self) -> u64;
}
