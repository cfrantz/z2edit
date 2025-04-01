use crate::gui::util::edit_tree_node;
use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::zelda2::misc_hacks::{config, Miscellaneous};
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::Miscellaneous {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        edit_tree_node(ui, "Miscellaneous", path, project);
        if let Some(_token) = ui.begin_popup_context_item() {
            result.menu(ui, &path);
        }
        result
    }
}

pub struct MiscellaneousEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    misc: Miscellaneous,
}

impl MiscellaneousEditor {
    pub fn new(misc: &Miscellaneous, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(MiscellaneousEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            misc: misc.clone(),
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.misc.clone())) {
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

        if let Some(_table) = ui.begin_table_header_with_flags(
            "misc",
            [
                TableColumnSetup::new("Setting"),
                TableColumnSetup::new("Value"),
            ],
            TableFlags::BORDERS,
        ) {
            ui.table_next_row();
            ui.table_next_column();
            ui.text("Walk Anywhere on Overworld");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui.checkbox("##walk_anywhere", &mut self.misc.walk_anywhere);
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Item Pickup Delay");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##item_pickup_delay", &mut self.misc.item_pickup_delay)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Text Delay");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##text_delay", &mut self.misc.text_delay)
                .step(1)
                .build();
            self.misc.text_delay = self.misc.text_delay.clamp(0, i8::MAX);
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Beam Sword Time");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##beam_sword_time", &mut self.misc.beam_sword_time)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Beam Sword Speed");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##beam_sword_speed", &mut self.misc.beam_sword_speed)
                .step(1)
                .build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Elevator Speed");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##elevator_speed", &mut self.misc.elevator_speed)
                .step(1)
                .build();
            self.misc.elevator_speed = self.misc.elevator_speed.clamp(0, i8::MAX);
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Fairy Speed");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            self.changed |= ui
                .input_scalar("##fairy_speed", &mut self.misc.fairy_speed)
                .step(1)
                .build();
            self.misc.fairy_speed = self.misc.fairy_speed.clamp(0, i8::MAX);
            width.end();

            let cfg = project.config.get::<config::Miscellaneous>(&self.path)?;
            for (key, hack) in cfg.hack.iter() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text(&hack.name);
                ui.table_next_column();
                let (default, detail) = hack.detail.get_index(0).expect("hack has no details");
                let value = self.misc.hack.get(key).unwrap_or(default).clone();
                let preview = hack.detail.get(&value).unwrap_or(detail);
                let width = ui.push_item_width(-1.0);
                if let Some(_combo) = ui.begin_combo(&format!("##{key}"), &preview.name) {
                    for (id, detail) in hack.detail.iter() {
                        if id == &value {
                            ui.set_item_default_focus();
                        }
                        if ui
                            .selectable_config(&detail.name)
                            .selected(id == &value)
                            .build()
                        {
                            self.misc.hack.insert(key.clone(), id.clone());
                            self.changed |= true;
                        }
                    }
                }
                width.end();
            }
        }
        Ok(())
    }
}

impl Gui for MiscellaneousEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Miscellaneous##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Miscellaneous Changed",
            "There are unsaved chagnes in the Miscellaneous Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
