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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeAction {
    None,
    Edit(String),
    Metadata(String),
}

impl TreeAction {
    pub fn set(&mut self, action: TreeAction) {
        if action != TreeAction::None {
            *self = action;
        }
    }

    pub fn menu(&mut self, ui: &imgui::Ui, path: &str) {
        if ui.menu_item("Edit") {
            *self = TreeAction::Edit(path.into());
        }
        if ui.menu_item("Metadata") {
            *self = TreeAction::Metadata(path.into());
        }
    }
}

pub trait GuiTree: Send + Sync + 'static {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction;
}

pub trait Gui: Send + Sync + 'static {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()>;
    fn wants_dispose(&self) -> bool;
    fn spawned(&mut self) -> Option<Box<dyn Gui>> {
        None
    }
}
