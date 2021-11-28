use std::rc::Rc;

use imgui;
use imgui::{im_str, ImStr, ImString};

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::idpath;
use crate::nes::hwpalette;
use crate::zelda2::config::Config;
use crate::zelda2::palette::{Palette, PaletteGroup};
use crate::zelda2::project::{Edit, Project, RomData};

pub struct PaletteGui {
    visible: Visibility,
    changed: bool,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    orig: Vec<PaletteGroup>,
    group: Vec<PaletteGroup>,
    selected: usize,
    error: ErrorDialog,
}

impl PaletteGui {
    pub fn new(project: &Project, edit: Option<Rc<Edit>>) -> Result<Box<dyn Gui>> {
        let is_new = edit.is_none();
        let edit = edit.unwrap_or_else(|| project.create_edit("PaletteGroup", None).unwrap());
        let config = Config::get(&edit.config())?;
        let mut names = Vec::new();
        for group in config.palette.group.iter() {
            names.push(ImString::new(&group.name));
        }
        let data = PaletteGui::read_palettes(&config, &edit)?;
        let orig = if is_new {
            data.clone()
        } else {
            let prev = project.previous_commit(Some(&edit));
            PaletteGui::read_palettes(&config, &prev)?
        };

        Ok(Box::new(PaletteGui {
            visible: Visibility::Visible,
            changed: false,
            edit: edit,
            names: names,
            orig: orig,
            group: data,
            selected: 0,
            error: ErrorDialog::default(),
        }))
    }

    pub fn read_palettes(config: &Config, edit: &Rc<Edit>) -> Result<Vec<PaletteGroup>> {
        let mut data = Vec::new();
        for group in config.palette.group.iter() {
            let mut pg = PaletteGroup::default();
            for palette in group.palette.iter() {
                let mut p = Palette {
                    id: idpath!(group.id, palette.id),
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
        let mut romdata = Box::new(PaletteGroup::default());
        // Diff the changes against the original data.
        for (og, ng) in self.orig.iter().zip(self.group.iter()) {
            for (op, np) in og.data.iter().zip(ng.data.iter()) {
                if op != np {
                    romdata.data.push(np.clone());
                }
            }
        }
        project.commit(&self.edit, romdata)
    }

    pub fn color_selector(name: &ImStr, ui: &imgui::Ui) -> Option<u8> {
        let mut result = None;
        ui.popup(name, || {
            for x in 0..16 {
                //if x != 0 { ui.same_line(); }
                let g = ui.begin_group();
                ui.text(if x == 0 {
                    im_str!("   {:02x}", x)
                } else {
                    im_str!("{:02x}", x)
                });
                for y in 0..4 {
                    if x == 0 {
                        ui.text(im_str!("{:x}0", y));
                        ui.same_line();
                    }
                    let i = y * 16 + x;
                    let style = ui.push_style_color(imgui::StyleColor::Button, hwpalette::fget(i));
                    if ui.button(&im_str!("  ##{}", i)) {
                        result = Some(i as u8);
                        ui.close_current_popup();
                    }
                    style.pop();
                }
                g.end();
                ui.same_line();
                if x == 0 {
                    let pos = ui.cursor_pos();
                    ui.set_cursor_pos([pos[0], pos[1] - 3.0]);
                }
            }
        });
        result
    }
}

impl Gui for PaletteGui {
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
                        Err(e) => self.error.show("PaletteGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }

                let config = Config::get(&self.edit.config()).unwrap();
                ui.text(im_str!(
                    "{:<20}    {:<18} {:<18} {:<18} {:<18}",
                    "Title",
                    "Palette 0",
                    "Palette 1",
                    "Palette 2",
                    "Palette 3"
                ));
                ui.separator();
                for (n, p) in config.palette.group[self.selected]
                    .palette
                    .iter()
                    .enumerate()
                {
                    ui.text(im_str!("{:<20}", p.name));
                    for (i, color) in self.group[self.selected].data[n]
                        .data
                        .iter_mut()
                        .enumerate()
                    {
                        let cindex = *color as usize;
                        ui.same_line();
                        if i % 4 == 0 {
                            ui.text(" ");
                            ui.same_line();
                        }
                        let style =
                            ui.push_style_color(imgui::StyleColor::Button, hwpalette::fget(cindex));
                        let label = im_str!("{:02x}##{}", color, n * 16 + i);
                        if ui.button(&label) {
                            ui.open_popup(&label);
                        }
                        style.pop();
                        if let Some(update) = PaletteGui::color_selector(&label, ui) {
                            *color = update;
                            self.changed = true;
                        }
                    }
                }
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Palettes Changed"),
            "There are unsaved changes in the Palette Editor.\nDo you want to discard them?",
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
