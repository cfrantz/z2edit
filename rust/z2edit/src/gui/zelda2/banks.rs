use crate::gui::{GuiTree, TreeAction};
use crate::zelda2::banks::config;
use crate::zelda2::project::Project;

impl GuiTree for config::GameBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        if let Some(drop) = &self.drops {
            result.set(drop.tree_node(ui, &format!("{path}/drops"), project));
        }
        if let Some(encounter) = &self.encounters {
            result.set(encounter.tree_node(ui, &format!("{path}/encounters"), project));
        }
        if !self.enemy.is_empty() {
            ui.tree_node_config(format!("Enemies##{path}")).build(|| {
                for (k, v) in self.enemy.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/enemy/{k}"), project));
                }
            });
        }
        if !self.metatile.is_empty() {
            ui.tree_node_config(format!("Metatile##{path}")).build(|| {
                for (k, v) in self.metatile.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/metatile/{k}"), project));
                }
            });
        }
        if !self.overworld.is_empty() {
            ui.tree_node_config(format!("Overworld##{path}")).build(|| {
                for (k, v) in self.overworld.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/overworld/{k}"), project));
                }
            });
        }
        if !self.palette.is_empty() {
            ui.tree_node_config(format!("Palette##{path}")).build(|| {
                for (k, v) in self.palette.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/palette/{k}"), project));
                }
            });
        }
        if let Some(sideview) = &self.sideview {
            ui.tree_node_config(format!("Sideview##{path}")).build(|| {
                for (k, v) in sideview.group.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/sideview/{k}"), project));
                }
            });
        }
        if !self.text_table.is_empty() {
            ui.tree_node_config(format!("Text Table##{path}"))
                .build(|| {
                    for (k, v) in self.text_table.iter() {
                        result.set(v.tree_node(ui, &format!("{path}/text_table/{k}"), project));
                    }
                });
        }

        result
    }
}

impl GuiTree for config::GlobalBank {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        result.set(self.drops.tree_node(ui, &format!("{path}/drops"), project));
        ui.tree_node_config(format!("Experience##{path}"))
            .build(|| {
                for (k, v) in self.experience.iter() {
                    result.set(v.tree_node(ui, &format!("{path}/experience/{k}"), project));
                }
            });
        result.set(
            self.enemy_xp
                .tree_node(ui, &format!("{path}/enemy_xp"), project),
        );
        result.set(self.item.tree_node(ui, &format!("{path}/item"), project));
        ui.tree_node_config(format!("Metatile##{path}")).build(|| {
            for (k, v) in self.metatile.iter() {
                result.set(v.tree_node(ui, &format!("{path}/metatile/{k}"), project));
            }
        });
        result.set(self.misc.tree_node(ui, &format!("{path}/misc"), project));
        ui.tree_node_config(format!("Palette##{path}")).build(|| {
            for (k, v) in self.palette.iter() {
                result.set(v.tree_node(ui, &format!("{path}/palette/{k}"), project));
            }
        });
        result.set(self.start.tree_node(ui, &format!("{path}/start"), project));

        result
    }
}
