use crate::gui::{GuiTree, TreeAction};
use crate::zelda2::config::Config;
use imgui::TreeNodeFlags;

impl GuiTree for Config {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> TreeAction {
        let mut result = TreeAction::None;
        result.set(self.chr.tree_node(ui, &format!("{path}/chr")));
        for (k, v) in self.bank.iter() {
            if ui.collapsing_header(format!("Bank {k}"), TreeNodeFlags::empty()) {
                result.set(v.tree_node(ui, &format!("{path}/bank/{k}")));
            }
        }
        if ui.collapsing_header(format!("Global"), TreeNodeFlags::empty()) {
            result.set(self.global.tree_node(ui, &format!("{path}/global")));
        }
        result
    }
}
