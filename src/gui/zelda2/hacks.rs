use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::{ImStr, ImString};

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::zelda2::config::Config;
use crate::zelda2::hacks::{Hack, Hacks};
use crate::zelda2::project::{Edit, Project, RomData};

pub struct HacksGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    hacks: Hacks,
    titles: Vec<ImString>,
    names: Vec<Vec<ImString>>,
    error: ErrorDialog,
}

impl HacksGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let win_id = edit.win_id(commit_index);
        let hacks = HacksGui::init(&edit)?;
        let (titles, names) = HacksGui::names(&edit)?;

        Ok(Box::new(HacksGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            hacks: hacks,
            titles: titles,
            names: names,
            error: ErrorDialog::default(),
        }))
    }

    fn names(edit: &Edit) -> Result<(Vec<ImString>, Vec<Vec<ImString>>)> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let cfg = &config.misc.hacks;
        let mut titles = Vec::new();
        let mut names = Vec::new();
        for hack in cfg.hack.iter() {
            titles.push(ImString::new(&hack.name));
            names.push(hack.item.iter().map(|i| ImString::new(&i.name)).collect());
        }
        Ok((titles, names))
    }

    fn init(edit: &Rc<Edit>) -> Result<Hacks> {
        let empty = Hacks::default();
        let mut hacks = Hacks::default();
        hacks.unpack(edit)?;

        let config = Config::get(&edit.meta.borrow().config)?;
        let cfg = &config.misc.hacks;

        // We don't try to detect the assembly-based hacks, so we clone
        // whatever was in the edit list.
        let edit = edit.edit.borrow();
        let orig = edit.as_any().downcast_ref::<Hacks>().unwrap_or(&empty);
        for hack in cfg.hack.iter() {
            if let Some(item) = orig.find_item(&hack.id) {
                hacks.item.push(item.clone());
            } else {
                hacks.item.push(Hack {
                    id: hack.id.clone(),
                    selected: 0,
                });
            }
        }
        Ok(hacks)
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let edit = Box::new(self.hacks.clone());
        let i = project.commit(self.commit_index, edit, None)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }
}

impl Gui for HacksGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(&im_str!("Miscellaneous Hacks##{}", self.win_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => self.error.show("HacksGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = self.changed;
                changed |= ui.checkbox(
                    im_str!("Walk anywhere on overworld"),
                    &mut self.hacks.walk_anywhere,
                );

                let width = ui.push_item_width(100.0);
                changed |= ui
                    .input_int(
                        im_str!("Item pickup delay"),
                        &mut self.hacks.item_pickup_delay,
                    )
                    .build();
                changed |= ui
                    .input_int(im_str!("Text delay"), &mut self.hacks.text_delay)
                    .build();
                changed |= ui
                    .input_int(im_str!("Beam sword time"), &mut self.hacks.beam_sword_time)
                    .build();
                changed |= ui
                    .input_int(
                        im_str!("Beam sword speed"),
                        &mut self.hacks.beam_sword_speed,
                    )
                    .build();
                changed |= ui
                    .input_int(im_str!("Elevator speed"), &mut self.hacks.elevator_speed)
                    .build();
                changed |= ui
                    .input_int(im_str!("Fairy speed"), &mut self.hacks.fairy_speed)
                    .build();
                width.pop(ui);

                for (i, item) in self.hacks.item.iter_mut().enumerate() {
                    let names = self.names[i]
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&ImStr>>();
                    imgui::ComboBox::new(&self.titles[i]).build_simple_string(
                        ui,
                        &mut item.selected,
                        &names,
                    );
                }

                self.changed = changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Hacks Changed"),
            "There are unsaved changes in the Hacks Editor.\nDo you want to discard them?",
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
