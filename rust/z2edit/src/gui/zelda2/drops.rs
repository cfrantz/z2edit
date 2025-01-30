use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::drops::{config, DropInfo, Dropper};
use crate::zelda2::enemies::config::EnemyGroup;
use crate::zelda2::items::config::Items;
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::DropInfo {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        ui.tree_node_config(format!("Drops##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            if ui.menu_item("Edit") {
                result = Some(path.into());
            }
        }
        result
    }
}

pub struct DropInfoEditor {
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    drops: DropInfo,
}

impl DropInfoEditor {
    pub fn new(di: &DropInfo, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(DropInfoEditor {
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            drops: di.clone(),
        }))
    }

    fn draw_dropper(
        name: &str,
        dropper: &mut Dropper,
        items: &[&str],
        enemies: &[&str],
        ui: &imgui::Ui,
    ) -> bool {
        let mut changed = false;
        if let Some(_table) = ui.begin_table_header_with_flags(
            &format!("dropper##{name}"),
            [TableColumnSetup::new(name), TableColumnSetup::default()],
            TableFlags::BORDERS,
        ) {
            if let Some(item) = dropper.item.as_mut() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text("Item");
                let mut it = *item as usize & 0x7f;
                ui.table_next_column();
                let _width = ui.push_item_width(-1.0);
                if ui.combo_simple_string(&format!("##item{name}"), &mut it, &items) {
                    *item = it as u8;
                    changed |= true;
                }
            }
            if let Some(enemy) = dropper.enemy.as_mut() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text("Enemy");
                let mut it = *enemy as usize & 0x7f;
                ui.table_next_column();
                let _width = ui.push_item_width(-1.0);
                if ui.combo_simple_string(&format!("##enemy{name}"), &mut it, &enemies) {
                    *enemy = it as u8;
                    changed |= true;
                }
            }
            if let Some(hp) = dropper.hp.as_mut() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text("Hit Points");
                ui.table_next_column();
                let _width = ui.push_item_width(-1.0);
                changed |= ui.input_scalar("##hp", hp).step(1).build();
            }
        }
        changed
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.drops.clone())) {
                Ok(()) => self.changed = false,
                Err(e) => self.error.show(
                    "Commit Error",
                    &format!("Error comitting {:?}", self.path),
                    e,
                ),
            }
        }
        ui.same_line();
        ui.text(&self.path);

        let config = project.config.get::<config::DropInfo>(&self.path)?;
        let items = project.config.get::<Items>("/global/item")?;
        let items = items
            .item
            .values()
            .map(|i| i.name.as_str())
            .collect::<Vec<_>>();

        if let Some(table) = self.drops.table.as_mut() {
            if let Some(_table) = ui.begin_table_with_flags("drops", 9, TableFlags::BORDERS) {
                ui.table_next_row();
                ui.table_next_column();
                ui.table_header("Kill Counter");
                for j in 0..8 {
                    ui.table_next_column();
                    ui.table_header(&format!("##kc{j}"));
                }

                ui.table_next_row();
                ui.table_next_column();
                ui.text("Count");
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                self.changed |= ui
                    .input_scalar("##count", &mut table.counter)
                    .step(1)
                    .build();
                width.end();

                ui.table_next_row();
                ui.table_next_column();
                ui.table_header("Drop Group");
                for j in 0..8 {
                    ui.table_next_column();
                    ui.table_header(&format!("##dg{j}"));
                }

                ui.table_next_row();
                ui.table_next_column();
                ui.text("Small");
                for (i, item) in table.small.iter_mut().enumerate() {
                    let mut it = *item as usize & 0x7f;
                    ui.table_next_column();
                    let _width = ui.push_item_width(-1.0);
                    if ui.combo_simple_string(&format!("##small{i}"), &mut it, &items) {
                        *item = it as u8 | 0x80;
                        self.changed |= true;
                    }
                }

                ui.table_next_row();
                ui.table_next_column();
                ui.text("Large");
                for (i, item) in table.large.iter_mut().enumerate() {
                    let mut it = *item as usize & 0x7f;
                    ui.table_next_column();
                    let _width = ui.push_item_width(-1.0);
                    if ui.combo_simple_string(&format!("##large{i}"), &mut it, &items) {
                        *item = it as u8 | 0x80;
                        self.changed |= true;
                    }
                }
            }
        }

        for (key, cfg) in config.dropper.iter() {
            let eg = project.config.get::<EnemyGroup>(&cfg.enemy_group)?;
            let eg = eg
                .group
                .values()
                .map(|i| i.name.as_str())
                .collect::<Vec<_>>();
            if let Some(dropper) = self.drops.dropper.get_mut(key) {
                self.changed |= Self::draw_dropper(&cfg.name, dropper, &items, &eg, ui);
            }
        }
        Ok(())
    }
}

impl Gui for DropInfoEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Drop Info##{}", self.path))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Drop Info Changed",
            "There are unsaved chagnes in the Drop Info Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }

    fn window_id(&self) -> u64 {
        0
    }
}
