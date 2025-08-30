use crate::gui::util::TreeAction;
use crate::gui::GuiTree;
use crate::zelda2::config::Config;
use crate::zelda2::project::Project;
use imgui::TreeNodeFlags;

impl GuiTree for Config {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        result.set(self.chr.tree_node(ui, &format!("{path}/chr"), project));
        for vchr in self.vchr.iter() {
            result.set(vchr.tree_node(ui, &format!("{path}/vchr"), project));
        }
        for (k, v) in self.bank.iter() {
            if ui.collapsing_header(format!("Bank {k}"), TreeNodeFlags::empty()) {
                result.set(v.tree_node(ui, &format!("{path}/bank/{k}"), project));
            }
        }
        if ui.collapsing_header(format!("Global"), TreeNodeFlags::empty()) {
            result.set(
                self.global
                    .tree_node(ui, &format!("{path}/global"), project),
            );
        }
        result
    }
}
