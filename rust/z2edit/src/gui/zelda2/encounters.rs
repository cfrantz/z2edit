use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::zelda2::encounters::{config, Encounter, Encounters};
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::Encounters {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> TreeAction {
        let mut result = TreeAction::None;
        ui.tree_node_config(format!("Encounters##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            result.menu(ui, &path);
        }
        result
    }
}

pub struct EncountersEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    encounters: Encounters,
}

impl EncountersEditor {
    pub fn new(di: &Encounters, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(EncountersEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            encounters: di.clone(),
        }))
    }

    fn edit_one(e: &mut Encounter, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        let _width = ui.push_item_width(100.0);
        ui.text("Area:");
        ui.same_line();
        changed |= ui.input_scalar("##Area", &mut e.area).step(1).build();
        ui.same_line();
        ui.text("  Screen:");
        ui.same_line();
        if ui.input_scalar("##Screen", &mut e.screen).step(1).build() {
            e.screen = e.screen.clamp(0, 3);
            changed = true;
        }
        changed
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.encounters.clone())) {
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

        let config = project.config.get::<config::Encounters>(&self.path)?;

        if let Some(_table) = ui.begin_table_header_with_flags(
            &format!("##encounters"),
            [
                TableColumnSetup::new("Terrain"),
                TableColumnSetup::new("North"),
                TableColumnSetup::new("South"),
            ],
            TableFlags::BORDERS,
        ) {
            for terrain in config.terrain.iter() {
                ui.table_next_row();
                ui.table_next_column();
                ui.text(terrain);

                ui.table_next_column();
                if let Some(e) = self.encounters.north.get_mut(terrain) {
                    let _id = ui.push_id(format!("north{terrain}"));
                    self.changed |= Self::edit_one(e, ui);
                }
                ui.table_next_column();
                if let Some(e) = self.encounters.south.get_mut(terrain) {
                    let _id = ui.push_id(format!("south{terrain}"));
                    self.changed |= Self::edit_one(e, ui);
                }
            }
        }
        Ok(())
    }
}

impl Gui for EncountersEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Encounters##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Encounters Changed",
            "There are unsaved chagnes in the Encounters Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
