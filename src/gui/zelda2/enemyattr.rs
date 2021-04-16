use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString, TableColumnFlags, TableFlags};
use once_cell::sync::Lazy;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::zelda2::config::Config;
use crate::zelda2::enemyattr::{config, Enemy, EnemyGroup};
use crate::zelda2::project::{Edit, Project, RomData};

pub struct EnemyGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    orig: Vec<EnemyGroup>,
    group: Vec<EnemyGroup>,
    selected: usize,
    error: ErrorDialog,
}

static HEX: Lazy<Vec<&ImStr>> = Lazy::new(|| {
    vec![
        im_str!("0"),
        im_str!("1"),
        im_str!("2"),
        im_str!("3"),
        im_str!("4"),
        im_str!("5"),
        im_str!("6"),
        im_str!("7"),
        im_str!("8"),
        im_str!("9"),
        im_str!("A"),
        im_str!("B"),
        im_str!("C"),
        im_str!("D"),
        im_str!("E"),
        im_str!("F"),
    ]
});

static DROP_GROUP: Lazy<Vec<&ImStr>> = Lazy::new(|| {
    vec![
        im_str!("None"),
        im_str!("Small"),
        im_str!("Large"),
        im_str!("Unknown"),
    ]
});

static COLUMNS: Lazy<Vec<(&ImStr, f32)>> = Lazy::new(|| {
    vec![
        (im_str!("Name"), 256.0),
        (im_str!("HitPoints"), 128.0),
        (im_str!("Palette"), 64.0),
        (im_str!("StealXP"), 64.0),
        (im_str!("NeedFire"), 72.0),
        (im_str!("Points"), 64.0),
        (im_str!("DropGrp"), 72.0),
        (im_str!("BeamImm"), 64.0),
        (im_str!("SpellIm"), 64.0),
        (im_str!("DmgType"), 64.0),
        (im_str!("ThndImm"), 64.0),
        (im_str!("Regen"), 64.0),
        (im_str!("Unknown"), 64.0),
        (im_str!("SwordIm"), 64.0),
        (im_str!("Unknown"), 64.0),
    ]
});

impl EnemyGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let config = Config::get(&edit.config())?;
        let mut names = Vec::new();
        for group in config.enemy.0.iter() {
            names.push(ImString::new(&group.name));
        }
        let data = EnemyGui::read_enemies(&config, &edit)?;
        let orig = if commit_index > 0 {
            let prev = project.get_commit(commit_index - 1)?;
            EnemyGui::read_enemies(&config, &prev)?
        } else {
            data.clone()
        };

        let win_id = edit.win_id(commit_index);
        Ok(Box::new(EnemyGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            names: names,
            orig: orig,
            group: data,
            selected: 0,
            error: ErrorDialog::default(),
        }))
    }

    pub fn read_enemies(config: &Config, edit: &Rc<Edit>) -> Result<Vec<EnemyGroup>> {
        let mut data = Vec::new();
        for group in config.enemy.0.iter() {
            let mut pg = EnemyGroup::default();
            for enemy in group.enemy.iter() {
                let mut p = Enemy {
                    id: group.id.extend(&enemy.id),
                    ..Default::default()
                };
                p.unpack(edit)?;
                pg.data.push(p);
            }
            data.push(pg);
        }
        Ok(data)
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let mut edit = Box::new(EnemyGroup::default());
        // Diff the changes against the original data.
        for (og, ng) in self.orig.iter().zip(self.group.iter()) {
            for (op, np) in og.data.iter().zip(ng.data.iter()) {
                if op != np {
                    edit.data.push(np.clone());
                }
            }
        }
        let i = project.commit(self.commit_index, edit, None)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }

    pub fn enemy_row(enemy: &mut Enemy, config: &config::Sprite, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text(im_str!("{:02x}: {}", config.offset, config.name));

        ui.table_next_column();
        changed |= imgui::InputInt::new(ui, im_str!("##hp"), &mut enemy.hp).build();

        ui.table_next_column();
        let width = ui.push_item_width(40.0);
        changed |= imgui::ComboBox::new(im_str!("##pal")).build_simple_string(
            ui,
            &mut enemy.palette,
            &HEX[0..4],
        );
        width.pop(ui);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##steal"), &mut enemy.steal_xp);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##fire"), &mut enemy.need_fire);

        ui.table_next_column();
        let width = ui.push_item_width(40.0);
        changed |=
            imgui::ComboBox::new(im_str!("##xp")).build_simple_string(ui, &mut enemy.xp, &HEX);
        width.pop(ui);

        ui.table_next_column();
        let width = ui.push_item_width(60.0);
        changed |= imgui::ComboBox::new(im_str!("##dg")).build_simple_string(
            ui,
            &mut enemy.drop_group,
            &DROP_GROUP,
        );
        width.pop(ui);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##nobeam"), &mut enemy.no_beam);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##unk1"), &mut enemy.unknown1);

        ui.table_next_column();
        let width = ui.push_item_width(40.0);
        changed |= imgui::ComboBox::new(im_str!("##damage")).build_simple_string(
            ui,
            &mut enemy.damage,
            &HEX,
        );
        width.pop(ui);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##nothunder"), &mut enemy.no_thunder);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##regen"), &mut enemy.regenerate);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##unk2"), &mut enemy.unknown2);

        ui.table_next_column();
        changed |= ui.checkbox(im_str!("##nosword"), &mut enemy.no_sword);

        ui.table_next_column();
        changed |= imgui::ComboBox::new(im_str!("##unk3")).build_simple_string(
            ui,
            &mut enemy.unknown3,
            &HEX,
        );

        changed
    }
}

impl Gui for EnemyGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = if self.commit_index == -1 {
            im_str!("Enemy Editor##{}", self.win_id)
        } else {
            im_str!("{}##{}", self.edit.label(), self.win_id)
        };
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let names = self
                    .names
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&ImStr>>();
                let width = ui.push_item_width(400.0);
                imgui::ComboBox::new(im_str!("Group")).build_simple_string(
                    ui,
                    &mut self.selected,
                    &names,
                );
                width.pop(ui);

                ui.same_line();
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self.error.show("EnemyGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }

                let config = Config::get(&self.edit.config()).unwrap();
                ui.begin_table_with_flags(
                    im_str!("##table"),
                    COLUMNS.len() as i32,
                    TableFlags::ROW_BG
                        | TableFlags::BORDERS
                        | TableFlags::RESIZABLE
                        | TableFlags::SCROLL_X
                        | TableFlags::SCROLL_Y,
                );
                for (name, width) in COLUMNS.iter() {
                    ui.table_setup_column_with_weight(name, TableColumnFlags::WIDTH_FIXED, *width);
                }
                ui.table_headers_row();

                for (n, cfg) in config.enemy.0[self.selected].enemy.iter().enumerate() {
                    ui.table_next_row();
                    let mut enemy = &mut self.group[self.selected].data[n];
                    let id = ui.push_id(n as i32);
                    self.changed |= EnemyGui::enemy_row(&mut enemy, &cfg, ui);
                    id.pop();
                }
                ui.end_table();
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Enemys Changed"),
            "There are unsaved changes in the Enemy Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.win_id
    }
}
