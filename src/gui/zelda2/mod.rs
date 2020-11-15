pub mod edit;
pub mod enemyattr;
pub mod hacks;
pub mod palette;
pub mod project;
pub mod python;
pub mod start;
pub mod text_table;
pub mod xp_spells;

use crate::zelda2::project::Project;
use imgui;

pub trait Gui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui);
    fn wants_dispose(&self) -> bool {
        false
    }
}
