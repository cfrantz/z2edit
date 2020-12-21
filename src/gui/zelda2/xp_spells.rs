use std::collections::HashMap;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString};
use once_cell::sync::Lazy;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::Visibility;
use crate::idpath;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_encoding::Text;
use crate::zelda2::xp_spells::{config, ExperienceTable, ExperienceTableGroup};

pub struct ExperienceTableGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    orig: Vec<ExperienceTableGroup>,
    group: Vec<ExperienceTableGroup>,
    selected: usize,
    gui_once_init: bool,
}

static COLUMNS: Lazy<HashMap<String, Vec<&ImStr>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let levelup = vec![
        im_str!("XP for next Level"),
        im_str!("Level 2"),
        im_str!("Level 3"),
        im_str!("Level 4"),
        im_str!("Level 5"),
        im_str!("Level 6"),
        im_str!("Level 7"),
        im_str!("Level 8"),
        im_str!("1 Up"),
    ];
    let levels = vec![
        im_str!(""),
        im_str!("Level 1"),
        im_str!("Level 2"),
        im_str!("Level 3"),
        im_str!("Level 4"),
        im_str!("Level 5"),
        im_str!("Level 6"),
        im_str!("Level 7"),
        im_str!("Level 8"),
    ];
    let spells = vec![
        im_str!("Spell Effects"),
        im_str!("Shield"),
        im_str!("Jump"),
        im_str!("Life"),
        im_str!("Fairy"),
        im_str!("Fire"),
        im_str!("Reflect"),
        im_str!("Spell"),
        im_str!("Thunder"),
    ];

    map.insert("level_up".into(), levelup);
    map.insert("sword_power".into(), levels.clone());
    map.insert("damage_group".into(), levels.clone());
    map.insert("spell".into(), levels);
    map.insert("effects".into(), spells);
    map
});

impl ExperienceTableGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let config = Config::get(&edit.meta.borrow().config)?;
        let mut names = Vec::new();
        for group in config.experience.group.iter() {
            names.push(ImString::new(&group.name));
        }
        let data = ExperienceTableGui::read_tables(&config, &edit)?;
        let orig = if commit_index > 0 {
            let prev = project.get_commit(commit_index - 1)?;
            ExperienceTableGui::read_tables(&config, &prev)?
        } else {
            data.clone()
        };

        let win_id = edit.win_id(commit_index);
        Ok(Box::new(ExperienceTableGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            names: names,
            orig: orig,
            group: data,
            selected: 0,
            gui_once_init: true,
        }))
    }

    pub fn read_tables(config: &Config, edit: &Rc<Edit>) -> Result<Vec<ExperienceTableGroup>> {
        let mut data = Vec::new();
        for group in config.experience.group.iter() {
            let mut pg = ExperienceTableGroup::default();
            for table in group.table.iter() {
                let mut p = ExperienceTable {
                    id: idpath!(group.id, table.id),
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
        let mut edit = Box::new(ExperienceTableGroup::default());
        for (og, ng) in self.orig.iter().zip(self.group.iter()) {
            for (ot, nt) in og.data.iter().zip(ng.data.iter()) {
                if ot != nt {
                    edit.data.push(nt.clone());
                }
            }
        }
        if edit.data.len() == 0 {
            info!("ExperienceTableGui: no changes to commit.");
        } else {
            let i = project.commit(self.commit_index, edit, None)?;
            self.edit = project.get_commit(i)?;
            self.commit_index = i;
        }
        Ok(())
    }

    pub fn table_row(
        xpt: &mut ExperienceTable,
        config: &config::ExperienceTable,
        ui: &imgui::Ui,
    ) -> bool {
        let mut changed = false;

        if config.id == "effects" {
            for c in COLUMNS.get("effects").unwrap().iter() {
                ui.text(c);
                ui.next_column();
            }
            ui.separator();
        }
        if config.game_name.is_some() {
            let mut name = imgui::ImString::new(&xpt.game_name);
            if ui
                .input_text(im_str!(""), &mut name)
                .resize_buffer(false)
                .build()
            {
                xpt.game_name = Text::validate(name.to_str(), Some(8));
                changed = true;
            }
        } else {
            ui.align_text_to_frame_padding();
            ui.text(im_str!("{}", config.name));
        }
        ui.next_column();

        for (n, data) in xpt.data.iter_mut().enumerate() {
            let id = ui.push_id(n as i32);
            let width = ui.push_item_width(100.0);
            changed |= ui.input_int(im_str!(""), data).build();
            width.pop(ui);
            id.pop(ui);
            ui.next_column();
        }

        changed
    }
}

impl Gui for ExperienceTableGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(&im_str!("ExperienceTable Editor##{}", self.win_id))
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
                        Err(e) => error!("ExperienceTableGui: commit error {}", e),
                        _ => {}
                    };
                    self.changed = false;
                }

                let config = Config::get(&self.edit.meta.borrow().config).unwrap();
                let columns = COLUMNS
                    .get(&config.experience.group[self.selected].id)
                    .unwrap();
                ui.columns(columns.len() as i32, im_str!("columns"), true);
                ui.separator();
                for (n, name) in columns.iter().enumerate() {
                    ui.set_column_offset(n as i32, (130 * n) as f32);
                    ui.text(name);
                    ui.next_column();
                }
                self.gui_once_init = true;
                for (n, cfg) in config.experience.group[self.selected]
                    .table
                    .iter()
                    .enumerate()
                {
                    ui.separator();
                    let mut table = &mut self.group[self.selected].data[n];
                    let id = ui.push_id(&cfg.id);
                    self.changed |= ExperienceTableGui::table_row(&mut table, &cfg, ui);
                    id.pop(ui);
                }
                ui.columns(1, im_str!(""), false);
                ui.separator();
            });
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("ExperienceTables Changed"),
            "There are unsaved changes in the ExperienceTable Editor.\nDo you want to discard them?",
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
