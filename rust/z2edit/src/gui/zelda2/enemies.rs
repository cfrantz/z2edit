use crate::gui::util::edit_tree_node;
use crate::gui::{ErrorDialog, Gui, GuiTree, TreeAction, Visibility};
use crate::zelda2::enemies::{config, Enemy, EnemyGroup};
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::{TableColumnSetup, TableFlags};

impl GuiTree for config::EnemyGroup {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        edit_tree_node(ui, &self.name, path, project);
        if let Some(_token) = ui.begin_popup_context_item() {
            result.menu(ui, &path);
        }
        result
    }
}

pub struct EnemyGroupEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    enemy: EnemyGroup,
}

const HEX: [&'static str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

const DROP_GROUP: [&'static str; 4] = ["None", "Small", "Large", "Unknown"];

impl EnemyGroupEditor {
    pub fn new(pg: &EnemyGroup, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(EnemyGroupEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            enemy: pg.clone(),
        }))
    }

    fn draw_row(enemy: &mut Enemy, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.input_scalar("##hp", &mut enemy.hp).step(1).build();
        width.end();

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.combo_simple_string("##pal", &mut enemy.palette, &HEX[0..4]);
        width.end();

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.combo_simple_string("##xp", &mut enemy.xp, &HEX);
        width.end();

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.combo_simple_string("##dg", &mut enemy.drop_group, &DROP_GROUP);
        width.end();

        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.combo_simple_string("##damage", &mut enemy.damage, &HEX);
        width.end();

        ui.table_next_column();
        changed |= ui.checkbox("##steal", &mut enemy.steal_xp);
        ui.table_next_column();
        changed |= ui.checkbox("##fire", &mut enemy.need_fire);
        ui.table_next_column();
        changed |= ui.checkbox("##regen", &mut enemy.regenerate);
        ui.table_next_column();
        changed |= ui.checkbox("##nobeam", &mut enemy.no_beam);
        ui.table_next_column();
        changed |= ui.checkbox("##nosword", &mut enemy.no_sword);
        ui.table_next_column();
        changed |= ui.checkbox("##nospell", &mut enemy.no_spell);
        ui.table_next_column();
        changed |= ui.checkbox("##nothunder", &mut enemy.no_thunder);

        ui.table_next_column();
        changed |= ui.checkbox("##unk2", &mut enemy.unknown2);
        ui.table_next_column();
        let width = ui.push_item_width(-1.0);
        changed |= ui.combo_simple_string("##unk3", &mut enemy.unknown3, &HEX);
        width.end();
        changed
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match project.commit(&self.path, Box::new(self.enemy.clone())) {
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

        let cfg = project.config.get::<config::EnemyGroup>(&self.path)?;

        #[rustfmt::skip]
        let columns = [
            TableColumnSetup { name: "Name", ..Default::default() },
            TableColumnSetup { name: "HitPoints", ..Default::default() },
            TableColumnSetup { name: "Palette", ..Default::default() },
            TableColumnSetup { name: "Points", ..Default::default() },
            TableColumnSetup { name: "DropGroup", ..Default::default() },
            TableColumnSetup { name: "DmgType", ..Default::default() },
            TableColumnSetup { name: "StealXP", ..Default::default() },
            TableColumnSetup { name: "NeedFire", ..Default::default() },
            TableColumnSetup { name: "Regen", ..Default::default() },
            TableColumnSetup { name: "BeamImm", ..Default::default() },
            TableColumnSetup { name: "SwordImm", ..Default::default() },
            TableColumnSetup { name: "SpellImm", ..Default::default() },
            TableColumnSetup { name: "ThndImm", ..Default::default() },
            TableColumnSetup { name: "Unknown2", ..Default::default() },
            TableColumnSetup { name: "Unknown3", ..Default::default() },
        ];

        if let Some(_table) = ui.begin_table_header_with_flags(
            self.path.as_str(),
            columns,
            TableFlags::ROW_BG
                | TableFlags::BORDERS
                | TableFlags::RESIZABLE
                | TableFlags::SCROLL_X
                | TableFlags::SCROLL_Y,
        ) {
            for (&n, enemy) in cfg.group.iter() {
                let _id = ui.push_id_usize(n as usize);
                ui.table_next_row();
                ui.table_next_column();
                ui.align_text_to_frame_padding();
                ui.text(format!("{:02x}: {}", n, enemy.name));
                if let Some(e) = self.enemy.group.get_mut(&n) {
                    self.changed |= Self::draw_row(e, ui);
                }
            }
        }
        Ok(())
    }
}

impl Gui for EnemyGroupEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("Enemies##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Enemies Changed",
            "There are unsaved chagnes in the Enemies Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
