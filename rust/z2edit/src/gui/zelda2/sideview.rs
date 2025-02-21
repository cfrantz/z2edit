use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::project::Project;
use crate::zelda2::sideview::{config, Sideview};
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::SideviewAreas {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        let name = &self.name;
        ui.tree_node_config(format!("{name}##{path}")).build(|| {
            for index in 0..self.length {
                let path = format!("{path}/{index}");
                ui.tree_node_config(format!("Area {index}##{path}"))
                    .leaf(true)
                    .build(|| {});
                if let Some(_token) = ui.begin_popup_context_item() {
                    if ui.menu_item("Edit") {
                        result.replace(path);
                    }
                }
            }
        });
        result
    }
}

pub struct SideviewEditor {
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    sideview: Sideview,
}

impl SideviewEditor {
    pub fn new(sv: &Sideview, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(SideviewEditor {
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            sideview: sv.clone(),
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.sideview.clone())) {
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

        let cfg = project.config.get::<config::SideviewAreas>(&self.path)?;
        Ok(())
    }
}

impl Gui for SideviewEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Sideview##{}", self.path))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Sideview Changed",
            "There are unsaved chagnes in the Sideview Editor.\nDo you want to discard them?",
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
