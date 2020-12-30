use std::collections::HashMap;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImString, MouseButton};

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::glhelper::Image;
use crate::gui::util::{draw_arrow, tooltip};
use crate::gui::zelda2::sideview::SideviewGui;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::Visibility;
use crate::nes::Address;
use crate::nes::IdPath;
use crate::util::clamp;
use crate::util::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::overworld::Connector;
use crate::zelda2::project::{Edit, Project};
use crate::zelda2::sideview::{Connection, Decompressor, Sideview};

struct Room {
    pub id: IdPath,
    pub name: ImString,
    pub x: f32,
    pub y: f32,
    pub connection: Vec<Connection>,
    pub elevator: Option<f32>,
    pub cpoints: [(f32, f32); 4],
    pub image: Image,
}

pub struct MultiMapGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    edit: Rc<Edit>,
    scale: f32,
    spread: [f32; 2],
    start: IdPath,
    rooms: HashMap<usize, Room>,

    background: TileCache,
    enemy_cache: TileCache,
    item_cache: TileCache,
    editor: Option<Box<dyn Gui>>,
    error: ErrorDialog,
}

impl MultiMapGui {
    pub fn new(edit: &Rc<Edit>, conn_id: &IdPath) -> Result<Box<dyn Gui>> {
        let start = if let Some(id) = edit.overworld_connector(conn_id) {
            id
        } else {
            return Err(ErrorKind::NotFound(format!("No destination map for {}", conn_id)).into());
        };

        let mut ret = Box::new(MultiMapGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: UTime::now(),
            edit: Rc::clone(&edit),
            scale: 0.25,
            spread: [1.25, 1.25],
            start: start,
            rooms: HashMap::new(),
            background: TileCache::new(&edit, Schema::None),
            enemy_cache: TileCache::new(&edit, Schema::None),
            item_cache: TileCache::new(&edit, Schema::None),
            editor: None,
            error: ErrorDialog::default(),
        });
        ret.explore()?;
        Ok(ret)
    }

    fn explore(&mut self) -> Result<()> {
        self.explore_map(self.start.clone(), 0.0, 0.0, false)?;
        self.normalize();
        Ok(())
    }

    fn normalize(&mut self) {
        let mut xmin = 0.0;
        let mut ymin = 0.0;
        for room in self.rooms.values() {
            xmin = if room.x < xmin { room.x } else { xmin };
            ymin = if room.y < ymin { room.y } else { ymin };
        }
        for room in self.rooms.values_mut() {
            room.x -= xmin;
            room.y -= ymin;
        }
    }

    fn explore_map(&mut self, start: IdPath, x: f32, y: f32, invalid: bool) -> Result<()> {
        let n = start.usize_last()?;
        if n > 62 {
            return Ok(());
        }
        if !self.rooms.contains_key(&n) {
            let sideview = Sideview::from_rom(&self.edit, start.clone())?;
            let image = self.render_map(&sideview)?;
            let mut room = Room {
                id: start.clone(),
                name: ImString::new(start.to_string()),
                x: x,
                y: y,
                connection: if invalid {
                    Vec::new()
                } else {
                    sideview.connection.clone()
                },
                elevator: sideview.map.elevator().map(|e| (e * 16) as f32),
                cpoints: [
                    (0.0, 104.0),
                    (384.0, 208.0),
                    (896.0, 208.0),
                    (1024.0, 104.0),
                ],
                image: image,
            };
            let deltas = if let Some(xcoord) = sideview.map.elevator() {
                room.cpoints[1] = ((xcoord * 16) as f32, 208.0);
                room.cpoints[2] = ((xcoord * 16) as f32, 0.0);
                [(-1024.0, 0.0), (0.0, 256.0), (0.0, -256.0), (1024.0, 0.0)]
            } else {
                [(-1024.0, 0.0), (0.0, 256.0), (0.0, 256.0), (1024.0, 0.0)]
            };
            self.rooms.insert(n, room);
            if !invalid {
                let start = start.pop();
                for (i, c) in sideview.connection.iter().enumerate() {
                    self.explore_map(
                        start.extend(c.dest_map),
                        x + deltas[i].0,
                        y + deltas[i].1,
                        c.dest_map == 0,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn render_map(&mut self, sideview: &Sideview) -> Result<Image> {
        let config = Config::get(&self.edit.config())?;
        let mut decomp = Decompressor::new();
        decomp.decompress(
            &sideview,
            sideview.background_layer_from_rom(&self.edit).as_ref(),
            &config.sideview,
            &config.objects,
        );
        self.reset_caches(sideview);
        let mut image = Image::new(
            Decompressor::WIDTH as u32 * 16,
            Decompressor::HEIGHT as u32 * 16,
        );
        self.render_background(&mut image, &decomp);
        self.render_items(&mut image, &sideview, &decomp);
        self.render_enemies(&mut image, &sideview);
        image.update();
        Ok(image)
    }

    fn render_background(&self, image: &mut Image, decomp: &Decompressor) {
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let tile = self.background.get(decomp.data[y][x]);
                image.overlay(&tile, x as u32 * 16, y as u32 * 16);
            }
        }
    }
    fn render_items(&self, image: &mut Image, sv: &Sideview, decomp: &Decompressor) {
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let item = decomp.item[y][x];
                if item != 0xFF && sv.availability.get(x / 16) == Some(&true) {
                    let sprite = self.item_cache.get(item);
                    image.overlay(&sprite, x as u32 * 16, y as u32 * 16);
                }
            }
        }
    }
    fn render_enemies(&self, image: &mut Image, sv: &Sideview) {
        for enemy in sv.enemy.data[0].iter() {
            let sprite = self.enemy_cache.get(enemy.kind as u8);
            image.overlay(&sprite, enemy.x as u32 * 16, enemy.y as u32 * 16);
        }
    }

    fn reset_caches(&mut self, sideview: &Sideview) {
        self.background.reset(Schema::MetaTile(
            self.edit.config().clone(),
            sideview.id.clone(),
            sideview.map.background_palette,
        ));
        self.item_cache.reset(Schema::Item(
            self.edit.config().clone(),
            sideview.id.clone(),
        ));
        self.enemy_cache.reset(Schema::Enemy(
            self.edit.config().clone(),
            sideview.enemy_group_id(),
            sideview.map.sprite_palette,
        ));
        if let Some(id) = self.edit.overworld_connector(&sideview.id) {
            let conn = Connector::from_rom(&self.edit, id).expect("reset_caches");
            if let Some(palace) = conn.palace {
                let chr = Address::Chr(palace.chr_bank as isize, 0);
                // FIXME: look these addresses up from Palette.
                let pal = if sideview.map.background_palette == 0 {
                    Address::Prg(4, 0x8470) + palace.palette * 16
                } else {
                    Address::Prg(4, 0xbf00) + palace.palette * 16
                };
                self.background.set_chr_override(Some(chr.add_bank(1)));
                self.item_cache.set_chr_override(Some(chr));
                self.enemy_cache.set_chr_override(Some(chr));
                if sideview.id.at(0) != "great_palace" {
                    self.background.set_pal_override(Some(pal));
                }
            }
        }
    }

    fn draw_connections(&mut self, _origin: [f32; 2], scr: [f32; 2], ui: &imgui::Ui) {
        let colors = &AppContext::pref().multimap;
        let widths = [2.0, 2.0, 4.0, 4.0];
        let scale = self.scale;
        let spread = self.spread;
        for room in self.rooms.values() {
            for (i, c) in room.connection.iter().enumerate() {
                if c.dest_map == 0 {
                    continue;
                }
                if let Some(dest) = self.rooms.get(&c.dest_map) {
                    let x0 = scr[0] + spread[0] * scale * room.x + room.cpoints[i].0 * scale;
                    let y0 = scr[1] + spread[1] * scale * room.y + room.cpoints[i].1 * scale;
                    let x1 = if dest.elevator.is_some() && (i == 1 || i == 2) {
                        let x = dest.elevator.unwrap();
                        scr[0] + spread[0] * scale * dest.x + x * scale
                    } else {
                        scr[0] + spread[0] * scale * dest.x + dest.cpoints[c.entry].0 * scale
                    };
                    let y1 = if dest.elevator.is_some() && (i == 1 || i == 2) {
                        scr[1]
                            + spread[1] * scale * dest.y
                            + if i == 1 { 0.0 } else { 208.0 * scale }
                    } else {
                        scr[1] + spread[1] * scale * dest.y + dest.cpoints[c.entry].1 * scale
                    };
                    draw_arrow([x0, y0], [x1, y1], colors[i], widths[i], 0.1, 10.0, ui);
                }
            }
        }
    }

    fn draw_multimap(
        &mut self,
        project: &Project,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
    ) -> bool {
        let config = Config::get(&self.edit.config()).expect("MultiMapGui::draw_multimap");
        self.draw_connections(origin, scr_origin, ui);
        let scale = self.scale;
        let spread = self.spread;
        for (&n, room) in self.rooms.iter_mut() {
            let id = ui.push_id(n as i32);
            let x = origin[0] + spread[0] * scale * room.x;
            let y = origin[1] + spread[1] * scale * room.y;
            room.image.draw_at([x, y], scale, ui);

            ui.set_cursor_pos([x, y]);
            ui.invisible_button(&im_str!("##room"), [1024.0 * scale, 208.0 * scale]);
            tooltip(
                &config
                    .sideview
                    .name(&room.id)
                    .expect("MultiMapGui::draw_multimap"),
                ui,
            );
            if ui.is_item_active() {
                if ui.is_mouse_dragging(MouseButton::Left) {
                    let delta = ui.io().mouse_delta;
                    room.x += delta[0] / (scale * spread[0]);
                    room.y += delta[1] / (scale * spread[1]);
                }
            }
            if ui.popup_context_item(im_str!("menu")) {
                if imgui::MenuItem::new(im_str!("Edit")).build(ui) {
                    self.editor = match SideviewGui::new(&project, -1, Some(room.id.clone())) {
                        Ok(gui) => Some(gui),
                        Err(e) => {
                            self.error.show(
                                "Sideview",
                                "Could not create Sideview Editor",
                                Some(e),
                            );
                            None
                        }
                    }
                }
                ui.end_popup();
            }
            id.pop(ui);
        }
        self.normalize();
        false
    }
}

impl Gui for MultiMapGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(&im_str!("Multimap Viewer##{}", self.win_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let width = ui.push_item_width(100.0);
                if imgui::InputFloat::new(ui, im_str!("Scale"), &mut self.scale)
                    .step(0.125)
                    .build()
                {
                    self.scale = clamp(self.scale, 0.125, 4.0);
                }
                width.pop(ui);

                let width = ui.push_item_width(200.0);
                imgui::Slider::new(im_str!("Spread"))
                    .range(1.0..=2.0)
                    .build_array(ui, &mut self.spread);
                width.pop(ui);

                ui.separator();
                let changed = false;

                let size = ui.content_region_avail();
                imgui::ChildWindow::new(1)
                    .movable(false)
                    .size(size)
                    .always_vertical_scrollbar(true)
                    .always_horizontal_scrollbar(true)
                    .build(ui, || {
                        let origin = ui.cursor_pos();
                        let scr_origin = ui.cursor_screen_pos();
                        self.draw_multimap(project, origin, scr_origin, ui);
                    });
                self.changed |= changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("MultiMap Changed"),
            "There are unsaved changes in the MultiMap Viewer.\nDo you want to discard them?",
            ui,
        );
    }

    fn refresh(&mut self) {}

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.win_id
    }

    fn spawn(&mut self) -> Option<Box<dyn Gui>> {
        self.editor.take()
    }
}
