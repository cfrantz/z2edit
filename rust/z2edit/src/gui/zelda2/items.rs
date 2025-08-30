use anyhow::Result;
use imgui::{TableColumnSetup, TableFlags};

use crate::gui::util::{edit_tree_node, TreeAction};
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::items::{config, Items};
use crate::zelda2::project::Project;

impl GuiTree for config::Items {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        edit_tree_node(ui, "Items", path, project)
    }
}

pub struct ItemsEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    items: Items,
}

impl ItemsEditor {
    const SLOTS: [&str; 16] = [
        "Rauru",
        "Ruto",
        "Saria",
        "Mido",
        "Nabooru",
        "Darunia",
        "New Kasuto",
        "Old Kasuto",
        "Shield",
        "Jump",
        "Life",
        "Fairy",
        "Fire",
        "Reflect",
        "Spell",
        "Thunder",
    ];
    pub fn new(it: &Items, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(ItemsEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            items: it.clone(),
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.items.clone())) {
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
            self.path.as_str(),
            [
                TableColumnSetup::new("Item"),
                TableColumnSetup::new("Activates Bit"),
                TableColumnSetup::new("For Town or Spell"),
                TableColumnSetup::new("Constraint"),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            for (name, effect) in self.items.effect.iter_mut() {
                let _id = ui.push_id(name);
                ui.table_next_row();
                ui.table_next_column();
                ui.text(format!("{name}"));

                ui.table_next_column();
                if ui.input_scalar("##bit", &mut effect.bit).step(1).build() {
                    effect.bit = effect.bit.clamp(0, 7);
                    self.changed |= true;
                }

                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                let mut slot = effect.slot as usize;
                if ui.combo_simple_string("##slot", &mut slot, &Self::SLOTS) {
                    effect.slot = slot as i8;
                    self.changed |= true;
                }
                width.end();

                ui.table_next_column();
                if let Some(count) = &mut effect.count {
                    if ui
                        .input_scalar("Magic Containers##count", count)
                        .step(1)
                        .build()
                    {
                        *count = (*count).clamp(0, 8);
                        self.changed |= true;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Gui for ItemsEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Items##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Items Changed",
            "There are unsaved chagnes in the Items Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
