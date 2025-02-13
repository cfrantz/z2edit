use crate::gui::GuiTree;
use crate::zelda2::banks::config;

impl GuiTree for config::GameBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        ui.tree_node_config(format!("Metatile##{path}")).build(|| {
            for (k, v) in self.metatile.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/metatile/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });
        ui.tree_node_config(format!("Palette##{path}")).build(|| {
            for (k, v) in self.palette.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/palette/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });

        if let Some(drop) = &self.drops {
            let _ = drop
                .tree_node(ui, &format!("{path}/drops"))
                .map(|x| Option::replace(&mut result, x));
        }
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
        ui.tree_node_config(format!("Metatile##{path}")).build(|| {
            for (k, v) in self.metatile.iter() {
                let _ = v
                    .tree_node(ui, &format!("{path}/metatile/{k}"))
                    .map(|x| Option::replace(&mut result, x));
            }
        });
        let _ = self
            .drops
            .tree_node(ui, &format!("{path}/drops"))
            .map(|x| Option::replace(&mut result, x));

        ui.tree_node_config(format!("Experience##{path}"))
            .build(|| {
                for (k, v) in self.experience.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/experience/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        let _ = self
            .enemy_xp
            .tree_node(ui, &format!("{path}/enemy_xp"))
            .map(|x| Option::replace(&mut result, x));
        let _ = self
            .misc
            .tree_node(ui, &format!("{path}/misc"))
            .map(|x| Option::replace(&mut result, x));
        let _ = self
            .start
            .tree_node(ui, &format!("{path}/start"))
            .map(|x| Option::replace(&mut result, x));

        result
    }
}
