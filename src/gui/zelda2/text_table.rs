use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString};

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::text_encoding::Text;
use crate::zelda2::text_table::{TextItem, TextTable};

pub struct TextTableGui {
    visible: Visibility,
    changed: bool,
    is_new: bool,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    orig: Vec<Vec<ImString>>,
    text: Vec<Vec<ImString>>,
    selected: usize,
    error: ErrorDialog,
}

impl TextTableGui {
    pub fn new(project: &Project, edit: Option<Rc<Edit>>) -> Result<Box<dyn Gui>> {
        let is_new = edit.is_none();
        let edit = edit.unwrap_or_else(|| project.create_edit("TextTable", None).unwrap());
        let config = Config::get(&edit.config())?;
        let mut names = Vec::new();
        let mut orig = Vec::<Vec<ImString>>::new();
        let mut text = Vec::<Vec<ImString>>::new();
        for table in config.text_table.table.iter() {
            names.push(ImString::new(&table.name));
            orig.push(Vec::new());
            text.push(Vec::new());
        }

        let mut ret = Box::new(TextTableGui {
            visible: Visibility::Visible,
            changed: false,
            is_new: is_new,
            edit: edit,
            names: names,
            orig: orig,
            text: text,
            selected: 0,
            error: ErrorDialog::default(),
        });
        ret.read_text(project)?;
        Ok(ret)
    }

    pub fn read_text(&mut self, project: &Project) -> Result<()> {
        let config = Config::get(&self.edit.config())?;

        let mut curr = TextTable::default();
        curr.unpack(&self.edit)?;
        let orig = if self.is_new {
            curr.clone()
        } else {
            let p = project.previous_commit(Some(&self.edit));
            let mut prev = TextTable::default();
            prev.unpack(&p)?;
            prev
        };

        for i in 0..self.orig.len() {
            self.orig[i].clear();
            self.text[i].clear();
        }
        for (o, c) in orig.data.iter().zip(curr.data.iter()) {
            let tcfg = config.text_table.find(&o.id)?;
            self.orig[tcfg.offset].push(ImString::new(&o.text));
            self.text[tcfg.offset].push(ImString::new(&c.text));
        }

        Ok(())
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let config = Config::get(&self.edit.config())?;
        let mut romdata = Box::new(TextTable::default());
        for i in 0..self.text.len() {
            let tcfg = &config.text_table.table[i];
            for (j, (ot, nt)) in self.orig[i].iter().zip(self.text[i].iter()).enumerate() {
                if ot != nt {
                    romdata.data.push(TextItem {
                        id: tcfg.id.extend(j),
                        text: nt.to_str().into(),
                    });
                }
            }
        }

        project.commit_one(&self.edit, romdata)
    }
}

impl Gui for TextTableGui {
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
                let names = self
                    .names
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&ImStr>>();
                imgui::ComboBox::new(im_str!("Group")).build_simple_string(
                    ui,
                    &mut self.selected,
                    &names,
                );

                ui.same_line();
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self.error.show("TextTableGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }

                let bytes = self.text.iter().fold(0, |acc, v| {
                    acc + v.iter().fold(0, |acc, x| acc + x.to_str().len() + 1)
                });
                ui.text(im_str!("Bytes used: {}", bytes));

                let group_id = ui.push_id(self.selected as i32);
                ui.separator();
                let mut changed = self.changed;
                for (i, text) in self.text[self.selected].iter_mut().enumerate() {
                    let id = ui.push_id(i as i32);
                    ui.text(im_str!("{:>3}", i));
                    ui.same_line();
                    if ui.input_text(im_str!(""), text).resize_buffer(true).build() {
                        *text = ImString::new(Text::validate(text.to_str(), None));
                        changed = true;
                    }
                    id.pop();
                }
                group_id.pop();
                self.changed = changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("TextTables Changed"),
            "There are unsaved changes in the TextTable Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.edit.random_id
    }
}
