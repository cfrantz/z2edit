use anyhow::Result;
use imgui::{TableColumnFlags, TableColumnSetup, TableFlags};
use indexmap::IndexMap;
use python_gui::fa;

use crate::error::Error;
use crate::gui::{ErrorDialog, Gui, Visibility};
use crate::util::time::UTime;
use crate::zelda2::edit::Metadata;
use crate::zelda2::project::Project;

pub struct MetadataEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    data: Metadata,
    extra: Vec<(String, String)>,
}

impl MetadataEditor {
    pub fn new(meta: &Metadata, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(MetadataEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            data: meta.clone(),
            extra: meta
                .extra
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }))
    }

    fn commit(&mut self, project: &mut Project) -> Result<()> {
        let edit = project
            .edits
            .get_mut(&self.path)
            .ok_or(Error::NotFound(self.path.clone()))?;
        self.data.timestamp = UTime::now();
        self.data.extra = IndexMap::from_iter(self.extra.iter().map(Clone::clone));
        edit.meta = self.data.clone();
        Ok(())
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match self.commit(project) {
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

        let mut changed = false;
        if let Some(_table) = ui.begin_table_header_with_flags(
            "meta",
            [
                TableColumnSetup {
                    name: "Item",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 200.0,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Value",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
            ],
            TableFlags::BORDERS,
        ) {
            ui.table_next_row();
            ui.table_next_column();
            ui.text("Timestamp");
            ui.table_next_column();
            ui.text(format!("{}", UTime::datetime(self.data.timestamp)));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Label");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            changed |= ui.input_text("##label", &mut self.data.label).build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("User");
            ui.table_next_column();
            let width = ui.push_item_width(-1.0);
            changed |= ui.input_text("##user", &mut self.data.user).build();
            width.end();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("Comment");
            ui.table_next_column();
            changed |= ui
                .input_text_multiline("##comment", &mut self.data.comment, [-1.0, 100.0])
                .build();
        }

        ui.text("\nExtra:");
        if let Some(_table) = ui.begin_table_header_with_flags(
            "meta",
            [
                TableColumnSetup {
                    name: "Key",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Value",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Del",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 32.0,
                    ..Default::default()
                },
            ],
            TableFlags::BORDERS,
        ) {
            let mut delindex = None;
            for (i, (key, value)) in self.extra.iter_mut().enumerate() {
                let _id = ui.push_id_usize(i);
                ui.table_next_row();
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                changed |= ui.input_text("##key", key).build();
                width.end();

                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                changed |= ui.input_text("##value", value).build();
                width.end();

                ui.table_next_column();
                if ui.button(&format!("{}", fa::ICON_TRASH)) {
                    delindex = Some(i);
                }
            }
            if let Some(i) = delindex {
                self.extra.remove(i);
            }
            if ui.button(&format!("{}", fa::ICON_COPY)) {
                self.extra.push(Default::default());
            }
        }

        self.changed |= changed;
        Ok(())
    }
}

impl Gui for MetadataEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Metadata##{}", self.window_id))
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
}
