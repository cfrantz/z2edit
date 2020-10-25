pub mod palette;
pub mod project;
pub mod edit;

use imgui;
use crate::zelda2::project::Project;

pub trait Gui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui);
    fn wants_dispose(&self) -> bool { false }
}
