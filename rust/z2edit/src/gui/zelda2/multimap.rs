use anyhow::Result;
use imgui::MouseButton;
use indexmap::IndexMap;
use python_gui::Image;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::app_preferences::{AppPreferences, MultiMapColor};
use crate::error::Error;
use crate::gui::util::draw_arrow;
use crate::gui::zelda2::sideview::SideviewEditor;
use crate::gui::{ErrorDialog, Gui, Visibility};
use crate::nes::Address;
use crate::util::tile_cache::{GfxCache, GfxKind};
use crate::zelda2::edit::GameData;
use crate::zelda2::overworld::Overworld;
use crate::zelda2::project::Project;
use crate::zelda2::sideview::config::SideviewAreas;
use crate::zelda2::sideview::{AreaKind, Connection, Decompressor, Sideview};

#[derive(Debug, Default, Serialize, Deserialize)]
struct RoomLayout {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct MultiMap {
    pub scale: f32,
    pub spread: [f32; 2],
    pub show_invalid_connections: bool,
    pub layout: IndexMap<u8, RoomLayout>,
}

// The multimap isn't really a game object, but we implement GameData so we can easily store it in
// the editlist.
#[typetag::serde]
impl GameData for MultiMap {
    fn name(&self) -> String {
        "MultiMap".into()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn from_json(&mut self, json: &str) -> Result<()> {
        *self = serde_json::from_str(json)?;
        Ok(())
    }
}

struct Room {
    pub path: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub connection: Vec<Connection>,
    pub door: Vec<Connection>,
    pub elevator: Option<f32>,
    pub cpoints: [[f32; 2]; 4],
    pub dpoints: [[f32; 2]; 4],
    pub image: Image,
}

pub struct MultiMapGui {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    scale: f32,
    spread: [f32; 2],
    show_invalid_connections: bool,
    rooms: IndexMap<u8, Room>,
    sequence: usize,
    spawn: Option<Box<dyn Gui>>,
}

impl MultiMapGui {
    pub fn new(path: &str, project: &Project) -> Result<Box<dyn Gui>> {
        let mut ret = Box::new(MultiMapGui {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            scale: 0.5,
            spread: [1.3, 1.3],
            show_invalid_connections: false,
            rooms: IndexMap::default(),
            sequence: project.connectivity.sequence(),
            spawn: None,
        });
        ret.explore(true, project)?;
        Ok(ret)
    }

    fn explore(&mut self, remember_layout: bool, project: &Project) -> Result<()> {
        if let Some(svid) = project.connectivity.get(&self.path) {
            // FIXME: should look up the connector to get the screen number Link will enter.  For
            // now, just enter on screen #0.
            self.explore_map(&svid, 0, 0.0, 0.0, false, project)?;
            if remember_layout {
                self.load_layout(project)?;
            }
            self.normalize();
            Ok(())
        } else {
            Err(Error::NotFound(format!("No destination map for {}", self.path)).into())
        }
    }

    fn normalize(&mut self) {
        let mut xmin = f32::MAX;
        let mut ymin = f32::MAX;
        for room in self.rooms.values() {
            xmin = xmin.min(room.x);
            ymin = ymin.min(room.y);
        }
        for room in self.rooms.values_mut() {
            room.x -= xmin;
            room.y -= ymin;
        }
    }

    fn load_layout(&mut self, project: &Project) -> Result<()> {
        let path = format!("multimap/{}", self.path.trim_start_matches('/'));
        if let Ok(layout) = project.data_ref::<MultiMap>(&path) {
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
        Ok(())
    }

    fn save_layout(&mut self, project: &mut Project) {
        let path = format!("multimap/{}", self.path.trim_start_matches('/'));
        let layout = Box::new(MultiMap {
            scale: self.scale,
            spread: self.spread,
            show_invalid_connections: self.show_invalid_connections,
            layout: IndexMap::from_iter(
                self.rooms
                    .iter()
                    .map(|(&k, v)| (k, RoomLayout { x: v.x, y: v.y })),
            ),
        });
        project.insert(path, layout);
    }

    fn explore_map(
        &mut self,
        start: &str,
        screen: usize,
        x: f32,
        y: f32,
        invalid: bool,
        project: &Project,
    ) -> Result<()> {
        let (base, n) = start.rsplit_once('/').expect("sideview id");
        let n = n.parse::<u8>()?;
        if n > 62 {
            return Ok(());
        }

        if !self.rooms.contains_key(&n) {
            let sideview = project.data_ref::<Sideview>(start)?;
            let config = project.config.get::<SideviewAreas>(start)?;
            let image = Self::render_map(start, project)?;
            let name = if let Some(name) = config.area_names.get(&n) {
                format!("Area {n:02}: {name}")
            } else {
                format!("Area {n:02}")
            };
            let mut room = Room {
                path: start.into(),
                name,
                x,
                y,
                connection: Vec::new(),
                door: Vec::new(),
                elevator: sideview.map.elevator().map(|e| e as f32 * 16.0),
                cpoints: [
                    [0.0, 104.0],
                    [384.0, 208.0],
                    [640.0, 208.0],
                    [1024.0, 104.0],
                ],
                dpoints: [
                    [8.0 * 16.0, 208.0],
                    [24.0 * 16.0, 208.0],
                    [40.0 * 16.0, 208.0],
                    [56.0 * 16.0, 208.0],
                ],
                image,
            };

            let deltas = if let Some(xcoord) = sideview.map.elevator() {
                room.cpoints[1] = [xcoord as f32 * 16.0, 208.0];
                room.cpoints[2] = [xcoord as f32 * 16.0, 0.0];
                [
                    // Where to draw the next connected room (with elevators).
                    [-1024.0, 0.0],
                    [0.0, 256.0],
                    [0.0, -256.0],
                    [1024.0, 0.0],
                    // Where to draw the next door-connected room.
                    [-1024.0, 200.0],
                    [-384.0, 200.0],
                    [384.0, 200.0],
                    [1024.0, 200.0],
                ]
            } else {
                [
                    // Where to draw the next connected room.
                    [-1024.0, 0.0],
                    [0.0, 256.0],
                    [0.0, 256.0],
                    [1024.0, 0.0],
                    // Where to draw the next door-connected room.
                    [-1024.0, 200.0],
                    [-384.0, 200.0],
                    [384.0, 200.0],
                    [1024.0, 200.0],
                ]
            };
            self.rooms.insert(n, room);
            if !invalid {
                // Screen start/end for connection exploration.
                let width = sideview.map.width as usize;
                let (ss, se) = if width >= 2 {
                    (0, 3)
                } else if screen & 1 == 0 {
                    (screen, screen + width)
                } else {
                    (screen - width, screen)
                };

                for i in 0..4 {
                    let c = sideview.connection.get(i);
                    if c.is_some() && i >= ss && i <= se {
                        let c = c.unwrap();
                        self.rooms.get_mut(&n).unwrap().connection.push(c.clone());
                        self.explore_map(
                            &format!("{base}/{}", c.area),
                            c.screen as usize,
                            x + deltas[i][0],
                            y + deltas[i][1],
                            c.area == 0,
                            project,
                        )?;
                    } else {
                        self.rooms
                            .get_mut(&n)
                            .unwrap()
                            .connection
                            .push(Connection::outside());
                    }

                    let c = sideview.door.get(i);
                    if c.is_some() && i >= ss && i <= se {
                        let c = c.unwrap();
                        self.rooms.get_mut(&n).unwrap().door.push(c.clone());
                        self.explore_map(
                            &format!("{base}/{}", c.area),
                            c.screen as usize,
                            x + deltas[i + 4][0],
                            y + deltas[i + 4][1],
                            c.area == 0,
                            project,
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

    fn draw_connections(&mut self, _origin: [f32; 2], scr: [f32; 2], ui: &imgui::Ui) {
        let colors = &AppPreferences::get().multimap;
        let widths = [2.0, 2.0, 4.0, 4.0];
        let scale = self.scale;
        let spread = self.spread;
        for (&_roomnum, room) in self.rooms.iter() {
            for (i, c) in room.connection.iter().enumerate() {
                if c.area == 0 && !self.show_invalid_connections {
                    continue;
                }
                let screen = c.screen as usize;
                if let Some(dest) = self.rooms.get(&c.area) {
                    let x0 = scr[0] + spread[0] * scale * room.x + room.cpoints[i][0] * scale;
                    let y0 = scr[1] + spread[1] * scale * room.y + room.cpoints[i][1] * scale;
                    let x1 = if dest.elevator.is_some() && (i == 1 || i == 2) {
                        let x = dest.elevator.unwrap();
                        scr[0] + spread[0] * scale * dest.x + x * scale
                    } else {
                        scr[0] + spread[0] * scale * dest.x + dest.cpoints[screen][0] * scale
                    };
                    let y1 = if dest.elevator.is_some() && (i == 1 || i == 2) {
                        scr[1]
                            + spread[1] * scale * dest.y
                            + if i == 1 { 0.0 } else { 208.0 * scale }
                    } else {
                        scr[1] + spread[1] * scale * dest.y + dest.cpoints[screen][1] * scale
                    };
                    // Invalid connections get colors[8], which should be gray.
                    let color = if c.area == 0 {
                        colors[&MultiMapColor::from(8)]
                    } else {
                        colors[&MultiMapColor::from(i)]
                    };
                    let width = if c.area == 0 { 1.0 } else { widths[i] };
                    draw_arrow([x0, y0], [x1, y1], color, width, 0.1, 10.0, ui);
                }
            }
            for (i, c) in room.door.iter().enumerate() {
                if c.area == 0 {
                    continue;
                }
                let screen = c.screen as usize;
                if let Some(dest) = self.rooms.get(&c.area) {
                    let x0 = scr[0] + spread[0] * scale * room.x + room.dpoints[i][0] * scale;
                    let y0 = scr[1] + spread[1] * scale * room.y + room.dpoints[i][1] * scale;
                    let x1 = scr[0] + spread[0] * scale * dest.x + dest.cpoints[screen][0] * scale;
                    let y1 = scr[1] + spread[1] * scale * dest.y + dest.cpoints[screen][1] * scale;
                    let color = if c.area == 0 {
                        colors[&MultiMapColor::from(8)]
                    } else {
                        colors[&MultiMapColor::from(i + 4)]
                    };
                    draw_arrow([x0, y0], [x1, y1], color, 2.0, 0.1, 10.0, ui);
                }
            }
        }
    }

    fn render_map(path: &str, project: &Project) -> Result<Image> {
        let mut decompressor = Decompressor::new();
        let sideview = project.data_ref::<Sideview>(path)?;
        let config = project.config.get::<SideviewAreas>(path)?;
        decompressor.decompress(path, sideview, project)?;

        let mut background = "background".to_string();
        let mut chr = config.chr;

        if config.area_kind == AreaKind::Palace {
            if let Some(connector) = project.connectivity.get(&format!("{path}/0")) {
                let (overworld, conn) = connector.rsplit_once('/').expect("connectivity path");
                let conn = conn.parse::<u8>()?;
                let overworld = project.data_ref::<Overworld>(overworld)?;
                if let Some(palace) = overworld
                    .connection
                    .get(&conn)
                    .map(|c| c.palace.as_ref())
                    .flatten()
                {
                    background = palace.palette.to_string();
                    chr = Address::Chr(palace.chr_bank as i16 + 1, 0);
                }
            }
        }

        let background_palette = if config.area_kind == AreaKind::Palace {
            (sideview.map.background_palette != 0) as u8
        } else {
            sideview.map.background_palette
        };
        let mut image = Image::new(1024, 208);
        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let im = GfxCache::get(
                    project,
                    &format!("{}/{}", config.palette, background), // idpath of a palette group.
                    &format!("{}", background_palette),
                    GfxKind::Metatile(chr, config.metatile.clone(), decompressor.data[y][x]),
                )?;
                image.overlay(&im, x as u32 * 16, y as u32 * 16);
            }
        }

        for y in 0..Decompressor::HEIGHT {
            for x in 0..Decompressor::WIDTH {
                let item = decompressor.item[y][x];
                if item != 255 && sideview.availability.get(x / 16) == Some(&true) {
                    let im = GfxCache::get(
                        project,
                        &format!("{}/sprite", config.palette), // idpath of a palette group.
                        &format!("{}", sideview.map.sprite_palette),
                        GfxKind::Item(item),
                    )?;
                    image.overlay(&im, x as u32 * 16, y as u32 * 16);
                }
            }
        }

        for enemy in sideview.enemy.data[0].iter() {
            let im = GfxCache::get(
                project,
                &format!("{}/sprite", config.palette), // idpath of a palette group.
                &format!("{}", sideview.map.sprite_palette),
                GfxKind::Enemy(
                    config.enemy_group.as_ref().cloned().unwrap(),
                    enemy.kind as u16,
                ),
            )?;
            image.overlay(&im, enemy.x as u32 * 16, enemy.y as u32 * 16);
        }
        Ok(image)
    }

    fn draw_multimap(
        &mut self,
        origin: [f32; 2],
        scr_origin: [f32; 2],
        ui: &imgui::Ui,
        project: &mut Project,
    ) -> Result<bool> {
        self.draw_connections(origin, scr_origin, ui);
        let mut changed = false;
        let scale = self.scale;
        let spread = self.spread;
        for (&n, room) in self.rooms.iter_mut() {
            let _id = ui.push_id_usize(n as usize);
            let x = origin[0] + spread[0] * scale * room.x;
            let y = origin[1] + spread[1] * scale * room.y;
            ui.set_cursor_pos([x, y - 24.0]);
            ui.text(&room.name);
            ui.set_cursor_pos([x, y]);
            room.image.draw_at([x, y], scale, ui);
            ui.set_cursor_pos([x, y]);
            ui.invisible_button("##room", [1024.0 * scale, 208.0 * scale]);
            if ui.is_item_active() {
                if ui.is_mouse_dragging(MouseButton::Left) {
                    let delta = ui.io().mouse_delta;
                    room.x += delta[0] / (scale * spread[0]);
                    room.y += delta[1] / (scale * spread[1]);
                    changed |= true;
                }
            }
            if let Some(_token) = ui.begin_popup_context_item() {
                if ui.menu_item("Edit") {
                    self.spawn = Some(SideviewEditor::new(
                        project.data_ref(&room.path)?,
                        &room.path,
                    )?);
                }
                if let Some(_token) = ui.begin_menu("Emulate") {
                    for screen in 0..=3 {
                        if ui.menu_item(format!("Screen {}", screen + 1)) {
                            project.emulate(Some(&format!("{}/{screen}", room.path)))?;
                        }
                    }
                }
            }
        }
        self.normalize();
        Ok(changed)
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut changed = false;
        if self.sequence != project.connectivity.sequence() {
            self.rooms.clear();
            self.explore(true, project)?;
            self.sequence = project.connectivity.sequence();
        }
        let width = ui.push_item_width(120.0);
        if ui
            .input_scalar("Scale", &mut self.scale)
            .step(0.125)
            .build()
        {
            self.scale = self.scale.clamp(0.125, 4.0);
            changed |= true;
        }
        width.end();

        ui.same_line();
        let width = ui.push_item_width(200.0);
        changed |= ui
            .slider_config("Spread", 1.0, 4.0)
            .build_array(&mut self.spread);
        width.end();

        ui.same_line();
        changed |= ui.checkbox(
            "Show Invalid Connection",
            &mut self.show_invalid_connections,
        );
        ui.separator();

        let size = ui.content_region_avail();
        changed |= ui
            .child_window("multimap")
            .movable(false)
            .size(size)
            .always_vertical_scrollbar(true)
            .always_horizontal_scrollbar(true)
            .build(|| {
                let mut origin = ui.cursor_pos();
                let mut scr_origin = ui.cursor_screen_pos();
                origin[0] += 16.0;
                origin[1] += 16.0;
                scr_origin[0] += 16.0;
                scr_origin[1] += 16.0;
                self.draw_multimap(origin, scr_origin, ui, project)
            })
            .transpose()?
            .unwrap_or(false);
        if changed {
            self.save_layout(project);
            changed = false;
        }
        self.changed |= changed;
        Ok(())
    }
}

impl Gui for MultiMapGui {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("MultiMap##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "MultiMap Changed",
            "There are unsaved chagnes in the MultiMap Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }

    fn spawned(&mut self) -> Option<Box<dyn Gui>> {
        self.spawn.take()
    }
}
