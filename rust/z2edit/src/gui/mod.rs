use anyhow::Result;

pub mod project;
pub mod zelda2;

use project::ProjectGui;

pub trait GuiTree: Send + Sync + 'static {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String>;
}

pub trait Gui: Send + Sync + 'static {
    fn draw(&mut self, ui: &imgui::Ui, project: &ProjectGui) -> Result<()>;
}
