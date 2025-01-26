use crate::gui::GuiTree;
use crate::zelda2::banks::config;

impl GuiTree for config::GameBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        ui.tree_node_config(format!("Palette##{path}")).build(|| {
            for (k, v) in self.palette.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/palette/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });
        ui.tree_node_config(format!("Enemies##{path}")).build(|| {
            for (k, v) in self.enemy.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/enemy/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });
        result
    }
}

impl GuiTree for config::GlobalBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        ui.tree_node_config(format!("Palette##{path}")).build(|| {
            for (k, v) in self.palette.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/palette/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });
        result
    }
}
