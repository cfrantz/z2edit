use crate::gui::GuiTree;
use crate::zelda2::config::Game;
use imgui::TreeNodeFlags;

impl GuiTree for Game {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        for (k, v) in self.bank.iter() {
            if ui.collapsing_header(format!("Bank {k}"), TreeNodeFlags::empty()) {
                result = result.or(v.tree_node(ui, &format!("{path}/bank/{k}")));
            }
        }
        if ui.collapsing_header(format!("Global"), TreeNodeFlags::empty()) {
            result = result.or(self.global.tree_node(ui, "{path}/global"));
        }
        result
    }
}
