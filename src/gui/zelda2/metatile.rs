use std::borrow::Cow;
use std::rc::Rc;
use std::u8;

use imgui;
use imgui::im_str;
use imgui::ImString;

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::gui::util::tooltip;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::util::clamp;
use crate::zelda2::config::Config;
use crate::zelda2::metatile::{Metatile, MetatileGroup};
use crate::zelda2::project::{Edit, Project};

pub struct MetatileGroupGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    commit_index: isize,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    group: MetatileGroup,
    orig: MetatileGroup,
    group_selected: usize,
    subgroup_selected: i32,
    tile_selected: usize,
    tile_edit: [u8; 4],
    tile_palette: Option<i32>,
    tile_image: Image,
    palette: i32,
    cache: TileCache,
    error: ErrorDialog,
}

impl MetatileGroupGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;
        let config = Config::get(&edit.config())?;

        let (orig, group) = if commit_index == -1 {
            let group = MetatileGroup::from_rom(&edit)?;
            let orig = group.clone();
            (orig, group)
        } else {
            let orig = MetatileGroup::from_rom(&project.get_commit(commit_index - 1)?)?;
            let group = MetatileGroup::from_rom(&edit)?;
            (orig, group)
        };

        let mut names = Vec::new();
        for mt in group.data.iter() {
            names.push(MetatileGroupGui::group_name(mt, &config)?);
        }

        let cache = TileCache::new(&edit, Schema::None);
        let win_id = edit.win_id(commit_index);
        let mut ret = Box::new(MetatileGroupGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: win_id,
            commit_index: commit_index,
            edit: edit,
            names: names,
            group: group,
            orig: orig,
            cache: cache,
            group_selected: 0,
            subgroup_selected: 0,
            tile_selected: 0,
            tile_edit: [0; 4],
            tile_palette: None,
            tile_image: Image::new_with_color(16, 16, 0xFFFF00FF),
            palette: 0,
            error: ErrorDialog::default(),
        });
        ret.reset_cache();
        Ok(ret)
    }

    fn group_name(mt: &Metatile, config: &Config) -> Result<ImString> {
        let name = match mt.id.last() {
            "overworldtile" => {
                let ocfg = config.overworld.find(&mt.id)?;
                im_str!("{} (Overworld)", ocfg.name)
            }
            "metatile" => {
                let scfg = config.sideview.find(&mt.id)?;
                im_str!("{} (Sideview)", scfg.name)
            }
            _ => {
                error!("Unexpected metatile category {}", mt.id);
                ImString::new(mt.id.to_string())
            }
        };
        Ok(name)
    }

    fn group_len(&self, mt: &Metatile) -> Result<usize> {
        let config = Config::get(&self.edit.config())?;
        let len = match mt.id.last() {
            "overworldtile" => {
                let ocfg = config.overworld.find(&mt.id)?;
                ocfg.objtable_len
            }
            "metatile" => 256,
            _ => {
                error!("Unexpected metatile category {}", mt.id);
                256
            }
        };
        Ok(len)
    }

    fn reset_cache(&mut self) {
        let cfgname = self.edit.config().clone();
        let id = self.group.data[self.group_selected].id.pop();
        match self.group.data[self.group_selected].id.last() {
            "overworldtile" => {
                self.cache.reset(Schema::Overworld(cfgname, id));
            }
            "metatile" => {
                self.cache
                    .reset(Schema::MetaTile(cfgname, id, self.palette));
            }
            _ => {}
        };
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let mut commit = Box::new(self.group.clone());
        for (orig, new) in self.orig.data.iter().zip(commit.data.iter_mut()) {
            new.tile.retain(|k, &mut v| Some(&v) != orig.tile.get(k));
            new.palette
                .retain(|k, &mut v| Some(&v) != orig.palette.get(k));
        }
        commit.data.retain(|data| !data.tile.is_empty());
        let i = project.commit(self.commit_index, commit, None)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        self.refresh();
        Ok(())
    }

    fn draw_metatiles(&mut self, ui: &imgui::Ui) -> Result<bool> {
        let width = ui.push_item_width(100.0);
        let tile_len = self.group_len(&self.group.data[self.group_selected])?;
        let subgroup_max = clamp(tile_len as i32 / 64 - 1, 0, 255);
        if ui
            .input_int(im_str!("Subgroup"), &mut self.subgroup_selected)
            .build()
        {
            self.subgroup_selected = clamp(self.subgroup_selected, 0, subgroup_max);
        }
        let max = if tile_len < 64 { tile_len } else { 64 };
        let start = self.subgroup_selected as usize * 64;
        let end = start + max;

        if ui.input_int(im_str!("Palette"), &mut self.palette).build() {
            self.palette = clamp(self.palette, 0, 7);
            self.reset_cache();
        }
        width.pop(ui);
        let mut changed = false;

        for i in start..end {
            if i % 8 == 0 {
                ui.text(im_str!("{:02x}: ", i));
            }
            ui.same_line(32.0 + (i % 8) as f32 * 48.0);
            if self.group.data[self.group_selected].tile.get(&i).is_none() {
                // Assumption: the first non-existent tile means end of the group.
                break;
            }

            {
                let token = if i == self.tile_selected {
                    Some(ui.push_style_colors(&[
                        (imgui::StyleColor::Button, [0.9, 0.9, 0.9, 0.9]),
                        (imgui::StyleColor::ButtonHovered, [1.0, 1.0, 1.0, 1.0]),
                        (imgui::StyleColor::ButtonActive, [0.9, 0.9, 0.9, 0.9]),
                    ]))
                } else {
                    None
                };
                let image = self.cache.get_alternate(
                    i as u8,
                    &self.group.data[self.group_selected].tile[&i],
                    self.group.data[self.group_selected]
                        .palette
                        .get(&i)
                        .map(|v| *v),
                );
                if imgui::ImageButton::new(image.id, [32.0, 32.0])
                    .frame_padding(4)
                    .build(ui)
                {
                    self.tile_selected = i;
                    self.tile_edit = self.group.data[self.group_selected].tile[&i];
                    self.tile_palette = self.group.data[self.group_selected]
                        .palette
                        .get(&i)
                        .map(|v| *v);
                    self.tile_image = self
                        .cache
                        .get_alternate_uncached(i as u8, &self.tile_edit, self.tile_palette)
                        .into_owned();
                    info!("selected tile {:02x} with {:x?}", i, self.tile_edit);
                    ui.open_popup(&im_str!("Edit Metatile ${:02x}", i));
                }
                token.map(|t| t.pop(ui));

                let tiles = &self.group.data[self.group_selected].tile[&i];
                tooltip(
                    &format!(
                        r#"Metatile ${:02x}
   {:02x} {:02x}
   {:02x} {:02x}"#,
                        i, tiles[0], tiles[2], tiles[1], tiles[3]
                    ),
                    ui,
                );
            }
            if self.draw_tile_edit(i, ui) {
                self.group.data[self.group_selected]
                    .tile
                    .insert(i, self.tile_edit);
                if let Some(p) = self.tile_palette {
                    self.group.data[self.group_selected].palette.insert(i, p);
                }
                self.reset_cache();
                changed |= true;
            }
        }
        Ok(changed)
    }
    fn draw_tile_edit(&mut self, which: usize, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        ui.popup(&im_str!("Edit Metatile ${:02x}", which), || {
            ui.group(|| {
                self.tile_image.draw(4.0, ui);
            });
            ui.same_line_with_spacing(0.0, 32.0);
            ui.group(|| {
                let mut internal_change = false;
                let width = ui.push_item_width(48.0);
                let indices = [0, 2, 1, 3];
                for &i in indices.iter() {
                    if i & 2 != 0 {
                        ui.same_line(0.0);
                    }
                    let mut val = im_str!("{:02x}", self.tile_edit[i]);
                    if ui
                        .input_text(&im_str!("##te{}", i), &mut val)
                        .chars_hexadecimal(true)
                        .build()
                    {
                        let x = u8::from_str_radix(val.to_str(), 16).unwrap_or(0);
                        if x != self.tile_edit[i] {
                            self.tile_edit[i] = x;
                            internal_change = true;
                        }
                    }
                }
                width.pop(ui);
                if let Some(p) = self.tile_palette.as_mut() {
                    let width = ui.push_item_width(100.0);
                    if ui.input_int(im_str!("Tile Palette"), p).build() {
                        *p = clamp(*p, 0, 3);
                        internal_change = true;
                    }
                    width.pop(ui);
                }
                if internal_change {
                    self.tile_image = self
                        .cache
                        .get_alternate_uncached(which as u8, &self.tile_edit, self.tile_palette)
                        .into_owned();
                }
            });
            if ui.button(im_str!("  Ok  "), [0.0, 0.0]) {
                changed = true;
                ui.close_current_popup();
            }
            ui.same_line(0.0);
            if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                ui.close_current_popup();
            }
        });
        changed
    }
}

impl Gui for MetatileGroupGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = if self.commit_index == -1 {
            im_str!("Metatile Editor##{}", self.win_id)
        } else {
            im_str!("{}##{}", self.edit.label(), self.win_id)
        };
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let width = ui.push_item_width(200.0);

                if imgui::ComboBox::new(im_str!("Group")).build_simple(
                    ui,
                    &mut self.group_selected,
                    &self.names,
                    &|x| Cow::Borrowed(&x),
                ) {
                    self.subgroup_selected = 0;
                    self.palette = 0;
                    self.tile_selected = 0;
                    self.reset_cache();
                }

                width.pop(ui);
                let mut changed = false;

                ui.same_line(0.0);
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => self.error.show("MetatileGroupGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                changed |= self.draw_metatiles(ui).expect("MetatileGroupGui::draw");
                self.changed |= changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Metatiles Changed"),
            "There are unsaved changes in the Metatile Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn refresh(&mut self) {
        self.reset_cache();
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.win_id
    }
}
