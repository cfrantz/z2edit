use anyhow::Result;

use crate::zelda2::project::Project;

mod error_dialog;
pub mod file_dialog;
pub mod preferences;
pub mod project;
pub mod util;
mod visibility;
pub mod widgets;
pub mod wizard;
pub mod zelda2;

use error_dialog::ErrorDialog;
use visibility::Visibility;

pub trait GuiTree: Send + Sync + 'static {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String>;
}

pub trait Gui: Send + Sync + 'static {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()>;
    fn wants_dispose(&self) -> bool;
    fn window_id(&self) -> u64;
    fn spawned(&mut self) -> Option<Box<dyn Gui>> {
        None
    }
}
