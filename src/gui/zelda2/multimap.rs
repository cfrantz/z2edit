use std::any::Any;
use std::rc::Rc;

use imgui;
use imgui::{im_str, ImString, MouseButton};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::glhelper::Image;
use crate::gui::util::draw_arrow;
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
use crate::zelda2::project::{Edit, Project, ProjectExtraData};
use crate::zelda2::sideview::{Connection, Decompressor, Sideview};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct RoomLayout {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct MultiMapLayout {
    pub scale: f32,
    pub spread: [f32; 2],
    pub show_invalid_connections: bool,
    pub layout: IndexMap<usize, RoomLayout>,
}

#[typetag::serde]
impl ProjectExtraData for MultiMapLayout {
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

struct Room {
    pub id: IdPath,
    pub name: ImString,
    pub x: f32,
    pub y: f32,
    pub connection: Vec<Connection>,
    pub door: Vec<Connection>,
    pub elevator: Option<f32>,
    pub cpoints: [(f32, f32); 4],
    pub dpoints: [(f32, f32); 4],
    pub image: Image,
}

pub struct MultiMapGui {
    visible: Visibility,
    changed: bool,
    win_id: u64,
    edit: Rc<Edit>,
    scale: f32,
    spread: [f32; 2],
    show_invalid_connections: bool,
    conn_id: IdPath,
    rooms: IndexMap<usize, Room>,

    background: TileCache,
    enemy_cache: TileCache,
    item_cache: TileCache,
    editor: Option<Box<dyn Gui>>,
    error: ErrorDialog,
}

impl MultiMapGui {
    pub fn new(edit: &Rc<Edit>, conn_id: &IdPath) -> Result<Box<dyn Gui>> {
        let mut ret = Box::new(MultiMapGui {
            visible: Visibility::Visible,
            changed: false,
            win_id: UTime::now(),
            edit: Rc::clone(&edit),
            scale: 0.25,
            spread: [1.25, 1.25],
            show_invalid_connections: true,
            conn_id: conn_id.clone(),
            rooms: IndexMap::new(),
            background: TileCache::new(&edit, Schema::None),
            enemy_cache: TileCache::new(&edit, Schema::None),
            item_cache: TileCache::new(&edit, Schema::None),
            editor: None,
            error: ErrorDialog::default(),
        });
        ret.explore(true)?;
        Ok(ret)
    }

    fn explore(&mut self, remember_layout: bool) -> Result<()> {
        if let Some(id) = self.edit.overworld_connector(&self.conn_id) {
            // FIXME: should look up the connector to get the screen number
            // Link will enter.  For now, just enter on screen #0.
            self.explore_map(id, 0, 0.0, 0.0, false)?;
            if remember_layout {
                self.load_layout();
            }
            self.normalize();
        } else {
            return Err(
                ErrorKind::NotFound(format!("No destination map for {}", self.conn_id)).into(),
            );
        };
        Ok(())
    }

    fn load_layout(&mut self) {
        let id = self.conn_id.extend("layout");
        if let Some(layout) = self.edit.extra_data.get::<MultiMapLayout>(&id) {
            self.scale = layout.scale;
            self.spread = layout.spread;
            self.show_invalid_connections = layout.show_invalid_connections;
            for (k, v) in layout.layout.iter() {
                if let Some(room) = self.rooms.get_mut(k) {
                    room.x = v.x;
                    room.y = v.y;
                }
            }
        }
    }
    fn save_layout(&self) {
        let mut layout = Box::new(MultiMapLayout::default());
        layout.scale = self.scale;
        layout.spread = self.spread;
        layout.show_invalid_connections = self.show_invalid_connections;
        for (&k, v) in self.rooms.iter() {
            layout.layout.insert(k, RoomLayout { x: v.x, y: v.y });
        }
        let id = self.conn_id.extend("layout");
        self.edit.extra_data.insert(id, layout);
    }

    fn normalize(&mut self) {
        let mut xmin = f32::MAX;
        let mut ymin = f32::MAX;
        for room in self.rooms.values() {
            xmin = if room.x < xmin { room.x } else { xmin };
            ymin = if room.y < ymin { room.y } else { ymin };
        }
        for room in self.rooms.values_mut() {
            room.x -= xmin;
            room.y -= ymin;
        }
    }

    fn explore_map(
        &mut self,
        start: IdPath,
        entry: usize,
        x: f32,
        y: f32,
        invalid: bool,
    ) -> Result<()> {
        let n = start.usize_last()?;
        if n > 62 {
            return Ok(());
        }
        if !self.rooms.contains_key(&n) {
            let sideview = Sideview::from_rom(&self.edit, start.clone())?;
            let config = Config::get(&self.edit.config())?;
            let scfg = config.sideview.find(&sideview.id)?;
            let dpoints = sideview.map.doors(scfg);
            let image = self.render_map(&sideview)?;
            let mut room = Room {
                id: start.clone(),
                name: ImString::new(start.to_string()),
                x: x,
                y: y,
                connection: Vec::new(),
                door: Vec::new(),
                elevator: sideview.map.elevator().map(|e| (e * 16) as f32),
                cpoints: [
                    (0.0, 104.0),
                    (384.0, 208.0),
                    (640.0, 208.0),
                    (1024.0, 104.0),
                ],
                dpoints: [
                    (dpoints[0] as f32 * 16.0, 208.0),
                    (dpoints[1] as f32 * 16.0, 208.0),
                    (dpoints[2] as f32 * 16.0, 208.0),
                    (dpoints[3] as f32 * 16.0, 208.0),
                ],
                image: image,
            };
            let deltas = if let Some(xcoord) = sideview.map.elevator() {
                room.cpoints[1] = ((xcoord * 16) as f32, 208.0);
                room.cpoints[2] = ((xcoord * 16) as f32, 0.0);
                [
                    // Where to draw the next connected room.
                    (-1024.0, 0.0),
                    (0.0, 256.0),
                    (0.0, -256.0),
                    (1024.0, 0.0),
                    // Where to draw the next door-connected room.
                    (-1024.0, 200.0),
                    (-384.0, 200.0),
                    (384.0, 200.0),
                    (1024.0, 200.0),
                ]
            } else {
                [
                    // Where to draw the next connected room.
                    (-1024.0, 0.0),
                    (0.0, 256.0),
                    (0.0, 256.0),
                    (1024.0, 0.0),
                    // Where to draw the next door-connected room.
                    (-1024.0, 200.0),
                    (-384.0, 200.0),
                    (384.0, 200.0),
                    (1024.0, 200.0),
                ]
            };
            self.rooms.insert(n, room);
            if !invalid {
                // Screen start/end for connection exploration.
                let width = sideview.map.width as usize - 1;
                let (ss, se) = if width == 3 {
                    (0, 3)
                } else if entry & 1 == 0 {
                    (entry, entry + width)
                } else {
                    (entry - width, entry)
                };

                let start = start.pop();
                for i in 0..4 {
                    let conn = sideview.connection.get(i);
                    if conn.is_some() && i >= ss && i <= se {
                        let c = conn.unwrap();
                        self.rooms.get_mut(&n).unwrap().connection.push(c.clone());
                        self.explore_map(
                            start.extend(c.dest_map),
                            c.entry,
                            x + deltas[i].0,
                            y + deltas[i].1,
                            c.dest_map == 0,
                        )?;
                    } else {
                        self.rooms
                            .get_mut(&n)
                            .unwrap()
                            .connection
                            .push(Connection::outside());
                    }

                    let conn = sideview.door.get(i);
                    if conn.is_some() && i >= ss && i <= se && dpoints[i] >= 0 {
                        let c = conn.unwrap();
                        self.rooms.get_mut(&n).unwrap().door.push(c.clone());
                        self.explore_map(
                            start.extend(c.dest_map),
                            c.entry,
                            x + deltas[4 + i].0,
                            y + deltas[4 + i].1,
                            c.dest_map == 0,
                        )?;
                    } else {
                        self.rooms
                            .get_mut(&n)
                            .unwrap()
                            .door
                            .push(Connection::outside());
                    }
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
        // Since we're just looking for palette info, we just get the overworld
        // connector for screen 0 of this room.
        if let Some(id) = self.edit.overworld_connector(&sideview.id.extend(0)) {
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
                if c.dest_map == 0 && !self.show_invalid_connections {
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
                    // Invalid connections get colors[8], which should be gray.
                    let color = if c.dest_map == 0 {
                        colors[8]
                    } else {
                        colors[i]
                    };
                    let width = if c.dest_map == 0 { 1.0 } else { widths[i] };
                    draw_arrow([x0, y0], [x1, y1], color, width, 0.1, 10.0, ui);
                }
            }
            for (i, c) in room.door.iter().enumerate() {
                if c.dest_map == 0 {
                    continue;
                }
                if let Some(dest) = self.rooms.get(&c.dest_map) {
                    let x0 = scr[0] + spread[0] * scale * room.x + room.dpoints[i].0 * scale;
                    let y0 = scr[1] + spread[1] * scale * room.y + room.dpoints[i].1 * scale;
                    let x1 = scr[0] + spread[0] * scale * dest.x + dest.cpoints[c.entry].0 * scale;
                    let y1 = scr[1] + spread[1] * scale * dest.y + dest.cpoints[c.entry].1 * scale;
                    draw_arrow([x0, y0], [x1, y1], colors[4 + i], 2.0, 0.1, 10.0, ui);
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
        let mut changed = false;
        let config = Config::get(&self.edit.config()).expect("MultiMapGui::draw_multimap");
        self.draw_connections(origin, scr_origin, ui);
        let scale = self.scale;
        let spread = self.spread;
        for (&n, room) in self.rooms.iter_mut() {
            let id = ui.push_id(n as i32);
            let x = origin[0] + spread[0] * scale * room.x;
            let y = origin[1] + spread[1] * scale * room.y;
            ui.set_cursor_pos([x, y - 16.0]);
            ui.text(
                config
                    .sideview
                    .name(&room.id)
                    .unwrap_or_else(|_err| room.id.to_string()),
            );
            room.image.draw_at([x, y], scale, ui);

            ui.set_cursor_pos([x, y]);
            ui.invisible_button(&im_str!("##room"), [1024.0 * scale, 208.0 * scale]);
            if ui.is_item_active() {
                if ui.is_mouse_dragging(MouseButton::Left) {
                    let delta = ui.io().mouse_delta;
                    room.x += delta[0] / (scale * spread[0]);
                    room.y += delta[1] / (scale * spread[1]);
                    changed |= true;
                }
            }
            if ui.popup_context_item(im_str!("menu")) {
                if imgui::MenuItem::new(im_str!("Edit")).build(ui) {
                    self.editor = match SideviewGui::new(
                        &project,
                        Some(Rc::clone(&self.edit)),
                        Some(room.id.clone()),
                    ) {
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
            id.pop();
        }
        self.normalize();
        changed
    }
}

impl Gui for MultiMapGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = im_str!("Multimap: {}##{}", self.conn_id, self.win_id);
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let mut changed = false;
                let width = ui.push_item_width(100.0);
                if imgui::InputFloat::new(ui, im_str!("Scale"), &mut self.scale)
                    .step(0.125)
                    .build()
                {
                    self.scale = clamp(self.scale, 0.125, 4.0);
                    changed |= true;
                }
                width.pop(ui);

                ui.same_line();
                let width = ui.push_item_width(200.0);
                changed |= imgui::Slider::new(im_str!("Spread"))
                    .range(1.0..=4.0)
                    .build_array(ui, &mut self.spread);
                width.pop(ui);

                ui.same_line();
                changed |= ui.checkbox(
                    im_str!("Show Invalid Connections"),
                    &mut self.show_invalid_connections,
                );

                ui.separator();

                let size = ui.content_region_avail();
                imgui::ChildWindow::new(1)
                    .movable(false)
                    .size(size)
                    .always_vertical_scrollbar(true)
                    .always_horizontal_scrollbar(true)
                    .build(ui, || {
                        let mut origin = ui.cursor_pos();
                        let mut scr_origin = ui.cursor_screen_pos();
                        origin[0] += 16.0;
                        origin[1] += 16.0;
                        scr_origin[0] += 16.0;
                        scr_origin[1] += 16.0;
                        changed |= self.draw_multimap(project, origin, scr_origin, ui);
                    });
                if changed {
                    self.save_layout();
                    changed = false;
                }
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
