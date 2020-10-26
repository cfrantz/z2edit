pub mod edit;
pub mod enemyattr;
pub mod palette;
pub mod project;

use crate::zelda2::project::Project;
use imgui;

pub trait Gui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui);
    fn wants_dispose(&self) -> bool {
        false
    }
}
