use crate::gui::GuiTree;
use crate::zelda2::banks::config;

impl GuiTree for config::GameBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        ui.tree_node_config(format!("Palette##{path}"))
            .build(|| {
                let mut result = None;
                for (k, v) in self.palette.iter() {
                    result = result.or(v.tree_node(ui, &format!("{path}/palette/{k}")));
                }
                result
            })
            .flatten()
    }
}

impl GuiTree for config::GlobalBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        ui.tree_node_config(format!("Palette##{path}"))
            .build(|| {
                let mut result = None;
                for (k, v) in self.palette.iter() {
                    result = result.or(v.tree_node(ui, &format!("{path}/palette/{k}")));
                }
                result
            })
            .flatten()
    }
}
