use anyhow::Result;
use imgui::{TableColumnSetup, TableFlags};

use crate::gui::util::{edit_tree_node, TreeAction};
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::project::Project;
use crate::zelda2::text_encoding::Text;
use crate::zelda2::text_table::{config, TextTable};

impl GuiTree for config::TextTable {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        edit_tree_node(ui, &self.name, path, project)
    }
}

pub struct TextTableEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    text: TextTable,
}

impl TextTableEditor {
    pub fn new(tt: &TextTable, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(TextTableEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            text: tt.clone(),
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.text.clone())) {
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
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Text"),
            ],
            TableFlags::ROW_BG | TableFlags::BORDERS | TableFlags::RESIZABLE,
        ) {
            for (&i, text) in self.text.data.iter_mut() {
                let _id = ui.push_id_usize(i as usize);
                ui.table_next_row();
                ui.table_next_column();
                ui.text(format!("{i}"));

                ui.table_next_column();
                let _width = ui.push_item_width(-1.0);
                if ui.input_text("##text", text).build() {
                    *text = Text::validate(text, None);
                    self.changed |= true;
                }
            }
        }
        Ok(())
    }
}

impl Gui for TextTableEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Text Table##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Text Table Changed",
            "There are unsaved chagnes in the Text Table Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
