use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::zelda2::project::Project;
use crate::zelda2::start::{config, StartValues};
use anyhow::Result;

use imgui::TableFlags;

impl GuiTree for config::StartValues {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> TreeAction {
        let mut result = TreeAction::None;
        ui.tree_node_config(format!("Start Values##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            result.menu(ui, &path);
        }
        result
    }
}

pub struct StartValuesEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    start: StartValues,
}

impl StartValuesEditor {
    pub fn new(sv: &StartValues, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(StartValuesEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            start: sv.clone(),
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.start.clone())) {
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

        if let Some(_table) = ui.begin_table_with_flags("start", 2, TableFlags::BORDERS) {
            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Start Values");
            ui.table_next_column();
            ui.table_header("##blank1");

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Attack");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##attack", &mut self.start.level.attack)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Magic");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##magic", &mut self.start.level.magic)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Life");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##life", &mut self.start.level.life)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Heart Containers");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##hc", &mut self.start.inventory.heart)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Magic Containers");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##mc", &mut self.start.inventory.magic)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Crystals");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##crystals", &mut self.start.inventory.magic)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Lives");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##lives", &mut self.start.inventory.magic)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Sword Techniques");
            ui.table_next_column();
            ui.table_header("##blank2");

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Downstab", &mut self.start.inventory.downstab);
            ui.table_next_column();
            self.changed |= ui.checkbox("Upstab", &mut self.start.inventory.upstab);

            ui.table_next_row();
            ui.table_next_column();
            ui.table_header("Spells");
            ui.table_next_column();
            ui.table_header("Inventory");

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Shield Spell", &mut self.start.spell.shield);
            ui.table_next_column();
            self.changed |= ui.checkbox("Candle", &mut self.start.inventory.candle);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Jump Spell", &mut self.start.spell.jump);
            ui.table_next_column();
            self.changed |= ui.checkbox("Glove", &mut self.start.inventory.glove);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Life Spell", &mut self.start.spell.life);
            ui.table_next_column();
            self.changed |= ui.checkbox("Raft", &mut self.start.inventory.raft);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Fairy Spell", &mut self.start.spell.fairy);
            ui.table_next_column();
            self.changed |= ui.checkbox("Boots", &mut self.start.inventory.boots);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Fire Spell", &mut self.start.spell.fire);
            ui.table_next_column();
            self.changed |= ui.checkbox("Flute", &mut self.start.inventory.flute);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Reflect Spell", &mut self.start.spell.reflect);
            ui.table_next_column();
            self.changed |= ui.checkbox("Cross", &mut self.start.inventory.cross);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Spell Spell", &mut self.start.spell.spell);
            ui.table_next_column();
            self.changed |= ui.checkbox("Hammer", &mut self.start.inventory.hammer);

            ui.table_next_row();
            ui.table_next_column();
            self.changed |= ui.checkbox("Thunder Spell", &mut self.start.spell.thunder);
            ui.table_next_column();
            self.changed |= ui.checkbox("Magic Key", &mut self.start.inventory.magic_key);
        }
        Ok(())
    }
}

impl Gui for StartValuesEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Start Values##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Start Values Changed",
            "There are unsaved chagnes in the Start Values Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
