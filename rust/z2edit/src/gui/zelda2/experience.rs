use anyhow::Result;
use imgui::{TableColumnSetup, TableFlags};

use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::nes::Address;
use crate::util::tile_cache::{GfxCache, GfxKind};
use crate::zelda2::experience::{
    config, EnemyExperience, ExperienceTable, ExperienceTableGroup, ExperienceValue,
};
use crate::zelda2::project::Project;
use crate::zelda2::text_encoding::Text;

impl GuiTree for config::ExperienceTableGroup {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> TreeAction {
        let mut result = TreeAction::None;
        let name = &self.name;
        ui.tree_node_config(format!("{name}##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            if ui.menu_item("Edit") {
                result = TreeAction::Edit(path.into());
            }
        }
        result
    }
}

pub struct ExperienceTableGroupEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    experience: ExperienceTableGroup,
}

impl ExperienceTableGroupEditor {
    pub fn new(eg: &ExperienceTableGroup, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(ExperienceTableGroupEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            experience: eg.clone(),
        }))
    }

    fn draw_row(cfg: &config::ExperienceTable, exp: &mut ExperienceTable, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        if !cfg.header.is_empty() {
            ui.table_next_row();
            for header in cfg.header.iter() {
                ui.table_next_column();
                ui.table_header(header);
            }
        }

        ui.table_next_row();
        ui.table_next_column();
        if cfg.game_text.is_some() {
            let _width = ui.push_item_width(-1.0);
            let mut name = exp.game_text.clone();
            if ui.input_text("##text", &mut name).build() {
                exp.game_text = Text::validate(&name, Some(8));
                changed = true;
            }
        } else {
            ui.align_text_to_frame_padding();
            ui.text(&cfg.name);
        }

        for (i, value) in exp.data.iter_mut().enumerate() {
            ui.table_next_column();
            let _width = ui.push_item_width(-1.0);
            changed |= ui.input_scalar(format!("##{i}"), value).build();
        }

        changed
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.experience.clone())) {
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

        let config = project
            .config
            .get::<config::ExperienceTableGroup>(&self.path)?;

        if let Some(_table) = ui.begin_table_with_flags(
            self.path.as_str(),
            9,
            TableFlags::ROW_BG
                | TableFlags::BORDERS
                | TableFlags::RESIZABLE
                | TableFlags::SCROLL_X
                | TableFlags::SCROLL_Y,
        ) {
            for (i, (cfg, exp)) in config
                .group
                .iter()
                .zip(self.experience.group.iter_mut())
                .enumerate()
            {
                let _id = ui.push_id_usize(i);
                self.changed |= Self::draw_row(cfg, exp, ui);
            }
        }
        Ok(())
    }
}

impl Gui for ExperienceTableGroupEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let config = project
            .config
            .get::<config::ExperienceTableGroup>(&self.path)?;
        let result = ui
            .window(format!("{}##{}", config.name, self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Experience Changed",
            "There are unsaved chagnes in the Experience Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}

impl GuiTree for config::EnemyExperience {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> TreeAction {
        let mut result = TreeAction::None;
        ui.tree_node_config(format!("Enemy XP##{path}"))
            .leaf(true)
            .build(|| {});
        if let Some(_token) = ui.begin_popup_context_item() {
            if ui.menu_item("Edit") {
                result = TreeAction::Edit(path.into());
            }
        }
        result
    }
}

pub struct EnemyExperienceEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    experience: EnemyExperience,
}

impl EnemyExperienceEditor {
    pub fn new(ee: &EnemyExperience, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(EnemyExperienceEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            experience: ee.clone(),
        }))
    }

    fn draw_row(
        i: usize,
        exp: &mut ExperienceValue,
        ui: &imgui::Ui,
        project: &Project,
    ) -> Result<bool> {
        let mut changed = false;
        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text(format!("{i:X}"));

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.input_scalar("##value", &mut exp.value).build();
        width.end();

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui
            .input_scalar_n("##sprites", &mut exp.sprites)
            .display_format("%02X")
            .chars_hexadecimal(true)
            .build();
        width.end();

        ui.table_next_column();
        let [x, y] = ui.cursor_pos();
        let scale = 2.0;
        for (i, sprite) in exp.sprites.iter().enumerate() {
            let image = GfxCache::get(
                project,
                // FIXME: maybe don't hardcode the palette and chrbank.
                "/bank/1/palette/sprite",
                "0",
                GfxKind::RawSprite(Address::Chr(2, 0), *sprite, 2),
            )?;
            image.draw_at([x + i as f32 * 8.0 * scale, y], scale, ui);
        }
        Ok(changed)
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.experience.clone())) {
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
                TableColumnSetup::new("Points"),
                TableColumnSetup::new("Sprites"),
                TableColumnSetup::new("Image"),
            ],
            TableFlags::ROW_BG
                | TableFlags::BORDERS
                | TableFlags::RESIZABLE
                | TableFlags::SCROLL_X
                | TableFlags::SCROLL_Y,
        ) {
            for (i, exp) in self.experience.data.iter_mut().enumerate() {
                let _id = ui.push_id_usize(i);
                self.changed |= Self::draw_row(i, exp, ui, project)?;
            }
        }
        Ok(())
    }
}

impl Gui for EnemyExperienceEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Enemy XP##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Experience Changed",
            "There are unsaved chagnes in the Experience Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
