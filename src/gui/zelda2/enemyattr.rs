use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString};
use once_cell::sync::Lazy;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::Visibility;
use crate::nes::IdPath;
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
    gui_once_init: bool,
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
        let config = Config::get(&edit.meta.borrow().config)?;
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

        let win_id = edit.meta.borrow().timestamp;
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
            gui_once_init: false,
        }))
    }

    pub fn read_enemies(config: &Config, edit: &Edit) -> Result<Vec<EnemyGroup>> {
        let mut data = Vec::new();
        for group in config.enemy.0.iter() {
            let mut pg = EnemyGroup::default();
            for enemy in group.enemy.iter() {
                let mut p = Enemy {
                    id: IdPath(vec![group.id.clone(), enemy.id.clone()]),
                    ..Default::default()
                };
                p.unpack(&edit)?;
                pg.data.push(p);
            }
            data.push(pg);
        }
        Ok(data)
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let mut edit = Box::new(EnemyGroup::default());
        for (og, ng) in self.orig.iter().zip(self.group.iter()) {
            for (op, np) in og.data.iter().zip(ng.data.iter()) {
                if op != np {
                    edit.data.push(np.clone());
                }
            }
        }
        if edit.data.len() == 0 {
            info!("EnemyGui: no changes to commit.");
        } else {
            let i = project.commit(self.commit_index, edit)?;
            self.edit = project.get_commit(i)?;
            self.commit_index = i;
        }
        Ok(())
    }

    pub fn enemy_row(enemy: &mut Enemy, config: &config::Enemy, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        ui.text(im_str!("{:02x}: {}", config.offset, config.name));
        ui.next_column();

        changed |= imgui::InputInt::new(ui, im_str!("##hp"), &mut enemy.hp).build();
        ui.next_column();

        let width = ui.push_item_width(40.0);
        changed |= imgui::ComboBox::new(im_str!("##pal")).build_simple_string(
            ui,
            &mut enemy.palette,
            &HEX[0..4],
        );
        width.pop(ui);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##steal"), &mut enemy.steal_xp);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##fire"), &mut enemy.need_fire);
        ui.next_column();

        let width = ui.push_item_width(40.0);
        changed |=
            imgui::ComboBox::new(im_str!("##xp")).build_simple_string(ui, &mut enemy.xp, &HEX);
        width.pop(ui);
        ui.next_column();

        let width = ui.push_item_width(60.0);
        changed |= imgui::ComboBox::new(im_str!("##dg")).build_simple_string(
            ui,
            &mut enemy.drop_group,
            &DROP_GROUP,
        );
        width.pop(ui);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##nobeam"), &mut enemy.no_beam);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##unk1"), &mut enemy.unknown1);
        ui.next_column();

        let width = ui.push_item_width(40.0);
        changed |= imgui::ComboBox::new(im_str!("##damage")).build_simple_string(
            ui,
            &mut enemy.damage,
            &HEX,
        );
        width.pop(ui);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##nothunder"), &mut enemy.no_thunder);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##regen"), &mut enemy.regenerate);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##unk2"), &mut enemy.unknown2);
        ui.next_column();

        changed |= ui.checkbox(im_str!("##nosword"), &mut enemy.no_sword);
        ui.next_column();

        changed |= imgui::ComboBox::new(im_str!("##unk3")).build_simple_string(
            ui,
            &mut enemy.unknown3,
            &HEX,
        );
        ui.next_column();

        changed
    }
}

impl Gui for EnemyGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(&im_str!("Enemy Editor##{}", self.win_id))
            .opened(&mut visible)
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

                ui.same_line(0.0);
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => error!("EnemyGui: commit error {}", e),
                        _ => {}
                    };
                    self.changed = false;
                }

                let config = Config::get(&self.edit.meta.borrow().config).unwrap();
                ui.separator();
                ui.columns(COLUMNS.len() as i32, im_str!("columns"), true);
                for (n, (name, width)) in COLUMNS.iter().enumerate() {
                    ui.text(name);
                    if !self.gui_once_init {
                        ui.set_column_width(n as i32, *width);
                    }
                    ui.next_column();
                }
                self.gui_once_init = true;
                ui.separator();
                for (n, cfg) in config.enemy.0[self.selected].enemy.iter().enumerate() {
                    let mut enemy = &mut self.group[self.selected].data[n];
                    let id = ui.push_id(n as i32);
                    self.changed |= EnemyGui::enemy_row(&mut enemy, &cfg, ui);
                    id.pop(ui);
                    ui.separator();
                }
            });
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
}
