use std::borrow::Cow;
use std::collections::HashMap;
use std::i32;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString, TableColumnFlags, TableFlags};
use once_cell::sync::Lazy;

use crate::errors::*;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::idpath;
use crate::nes::Address;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_encoding::Text;
use crate::zelda2::xp_spells::{config, ExperienceTable, ExperienceTableGroup, ExperienceValue};

pub struct ExperienceTableGui {
    visible: Visibility,
    changed: bool,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    orig: Vec<ExperienceTableGroup>,
    group: Vec<ExperienceTableGroup>,
    value: Vec<ExperienceValue>,
    selected: usize,
    gui_once_init: bool,
    tile_cache: TileCache,
    error: ErrorDialog,
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
    let spell_effects = vec![
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

    let mut damage_group = levels.clone();
    let mut spell_costs = levels.clone();
    damage_group[0] = im_str!("Damage Group");
    spell_costs[0] = im_str!("Spell");

    map.insert("level_up".into(), levelup);
    map.insert("sword_power".into(), levels);
    map.insert("damage_group".into(), damage_group);
    map.insert("spell".into(), spell_costs);
    map.insert("effects".into(), spell_effects);
    map
});

impl ExperienceTableGui {
    pub fn new(project: &Project, edit: Option<Rc<Edit>>) -> Result<Box<dyn Gui>> {
        let is_new = edit.is_none();
        let edit =
            edit.unwrap_or_else(|| project.create_edit("ExperienceTableGroup", None).unwrap());
        let config = Config::get(&edit.config())?;
        let mut names = Vec::new();
        for group in config.experience.group.iter() {
            names.push(ImString::new(&group.name));
        }
        names.push(ImString::new("Experience Values & Graphics"));

        let (data, values) = ExperienceTableGui::read_tables(&config, &edit)?;
        let (orig, values) = if is_new {
            (data.clone(), values)
        } else {
            let prev = project.previous_commit(Some(&edit));
            ExperienceTableGui::read_tables(&config, &prev)?
        };

        let tile_cache = TileCache::new(
            &edit,
            Schema::RawSprite(
                edit.config().clone(),
                Address::Chr(2, 0),
                idpath!("west_hyrule_sprites", 0),
                1,
            ),
        );
        Ok(Box::new(ExperienceTableGui {
            visible: Visibility::Visible,
            changed: false,
            edit: edit,
            names: names,
            orig: orig,
            group: data,
            value: values,
            selected: 0,
            gui_once_init: true,
            tile_cache: tile_cache,
            error: ErrorDialog::default(),
        }))
    }

    pub fn read_tables(
        config: &Config,
        edit: &Rc<Edit>,
    ) -> Result<(Vec<ExperienceTableGroup>, Vec<ExperienceValue>)> {
        let mut data = Vec::new();
        let mut values = Vec::new();
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

        for i in 0..config.experience.values.length {
            values.push(ExperienceValue::from_rom(
                edit,
                config.experience.values.id.extend(i),
            )?);
        }
        Ok((data, values))
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let mut romdata = Box::new(ExperienceTableGroup::default());
        for (og, ng) in self.orig.iter().zip(self.group.iter()) {
            for (ot, nt) in og.data.iter().zip(ng.data.iter()) {
                if ot != nt {
                    romdata.data.push(nt.clone());
                }
            }
        }

        romdata.value = self.value.clone();
        project.commit_one(&self.edit, romdata)
    }

    pub fn table_row(
        xpt: &mut ExperienceTable,
        config: &config::ExperienceTable,
        ui: &imgui::Ui,
    ) -> bool {
        let mut changed = false;

        if config.id == "effects" {
            for c in COLUMNS.get("effects").unwrap().iter() {
                ui.table_next_column();
                ui.text(c);
            }
            ui.table_next_row();
        }
        ui.table_next_column();
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

        for (n, data) in xpt.data.iter_mut().enumerate() {
            ui.table_next_column();
            let id = ui.push_id(n as i32);
            let width = ui.push_item_width(100.0);
            changed |= ui.input_int(im_str!(""), data).build();
            width.pop(ui);
            id.pop();
        }

        changed
    }

    pub fn xp_values(&mut self, ui: &imgui::Ui) -> bool {
        let mut changed = false;

        ui.begin_table_with_flags(
            im_str!("##table"),
            4,
            TableFlags::ROW_BG
                | TableFlags::BORDERS
                | TableFlags::RESIZABLE
                | TableFlags::SCROLL_X
                | TableFlags::SCROLL_Y,
        );
        ui.table_setup_column_with_weight(
            im_str!("Experience Category"),
            TableColumnFlags::WIDTH_FIXED,
            150.0,
        );
        ui.table_setup_column_with_weight(im_str!("Points"), TableColumnFlags::WIDTH_FIXED, 100.0);
        ui.table_setup_column_with_weight(
            im_str!("Sprite IDs"),
            TableColumnFlags::WIDTH_FIXED,
            100.0,
        );
        ui.table_setup_column_with_weight(im_str!("Image"), TableColumnFlags::WIDTH_FIXED, 50.0);
        ui.table_headers_row();

        for i in 0..self.value.len() {
            let id = ui.push_id(i as i32);
            ui.table_next_row();
            ui.table_next_column();
            ui.text(im_str!("{:X}", i));
            ui.table_next_column();
            let width = ui.push_item_width(100.0);
            let value = &mut self.value[i].value;
            changed |= ui.input_int(im_str!("##value"), value).build();
            width.pop(ui);

            ui.table_next_column();
            let width = ui.push_item_width(48.0);
            let mut sprite = im_str!("{:02x}", self.value[i].sprites[0]);
            if imgui::InputText::new(ui, im_str!("##spr0"), &mut sprite)
                .chars_hexadecimal(true)
                .build()
            {
                self.value[i].sprites[0] = i32::from_str_radix(sprite.to_str(), 16).unwrap_or(0);
                changed |= true;
            }
            ui.same_line();
            let mut sprite = im_str!("{:02x}", self.value[i].sprites[1]);
            if imgui::InputText::new(ui, im_str!("##spr1"), &mut sprite)
                .chars_hexadecimal(true)
                .build()
            {
                self.value[i].sprites[1] = i32::from_str_radix(sprite.to_str(), 16).unwrap_or(0);
                changed |= true;
            }
            width.pop(ui);

            ui.table_next_column();
            ui.same_line_with_spacing(10.0, 0.0);
            self.tile_cache
                .get(self.value[i].sprites[0] as u8)
                .draw(2.0, ui);
            ui.same_line_with_spacing(0.0, 0.0);
            self.tile_cache
                .get(self.value[i].sprites[1] as u8)
                .draw(2.0, ui);
            id.pop();
        }
        ui.end_table();
        changed
    }
}

impl Gui for ExperienceTableGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = ImString::new(self.edit.title());
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let width = ui.push_item_width(400.0);
                imgui::ComboBox::new(im_str!("Group")).build_simple(
                    ui,
                    &mut self.selected,
                    &self.names,
                    &|x| Cow::Borrowed(&x),
                );
                width.pop(ui);

                ui.same_line();
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self
                            .error
                            .show("ExperienceTableGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }

                let config = Config::get(&self.edit.config()).unwrap();
                if self.selected == self.names.len() - 1 {
                    self.changed |= self.xp_values(ui);
                } else {
                    let columns = COLUMNS
                        .get(&config.experience.group[self.selected].id)
                        .unwrap();
                    ui.begin_table_with_flags(
                        im_str!("##table"),
                        columns.len() as i32,
                        TableFlags::ROW_BG
                            | TableFlags::BORDERS
                            | TableFlags::RESIZABLE
                            | TableFlags::SCROLL_X
                            | TableFlags::SCROLL_Y,
                    );

                    for name in columns.iter() {
                        ui.table_setup_column_with_weight(
                            name,
                            TableColumnFlags::WIDTH_FIXED,
                            130.0,
                        );
                    }
                    ui.table_headers_row();
                    self.gui_once_init = true;
                    for (n, cfg) in config.experience.group[self.selected]
                        .table
                        .iter()
                        .enumerate()
                    {
                        ui.table_next_row();
                        let mut table = &mut self.group[self.selected].data[n];
                        let id = ui.push_id(&cfg.id);
                        self.changed |= ExperienceTableGui::table_row(&mut table, &cfg, ui);
                        id.pop();
                    }
                    ui.end_table();
                }
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("ExperienceTables Changed"),
            "There are unsaved changes in the ExperienceTable Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn refresh(&mut self) {
        self.tile_cache.clear();
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.edit.random_id
    }
}
