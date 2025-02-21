use crate::gui::GuiTree;
use crate::zelda2::banks::config;

impl GuiTree for config::GameBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        if let Some(drop) = &self.drops {
            let _ = drop
                .tree_node(ui, &format!("{path}/drops"))
                .map(|x| Option::replace(&mut result, x));
        }
        if let Some(encounter) = &self.encounters {
            let _ = encounter
                .tree_node(ui, &format!("{path}/encounters"))
                .map(|x| Option::replace(&mut result, x));
        }
        if !self.enemy.is_empty() {
            ui.tree_node_config(format!("Enemies##{path}")).build(|| {
                for (k, v) in self.enemy.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/enemy/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        }
        if !self.metatile.is_empty() {
            ui.tree_node_config(format!("Metatile##{path}")).build(|| {
                for (k, v) in self.metatile.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/metatile/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        }
        if !self.overworld.is_empty() {
            ui.tree_node_config(format!("Overworld##{path}")).build(|| {
                for (k, v) in self.overworld.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/overworld/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        }
        if !self.palette.is_empty() {
            ui.tree_node_config(format!("Palette##{path}")).build(|| {
                for (k, v) in self.palette.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/palette/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        }
        if let Some(sideview) = &self.sideview {
            ui.tree_node_config(format!("Sideview##{path}")).build(|| {
                for (k, v) in sideview.group.iter() {
                    let _ = v
                        .tree_node(ui, &format!("{path}/sideview/{k}"))
                        .map(|x| Option::replace(&mut result, x));
                }
            });
        }

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
