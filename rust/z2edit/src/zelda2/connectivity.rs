use anyhow::Result;
use indexmap::IndexMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::error::Error;
use crate::zelda2::overworld::Overworld;
use crate::zelda2::project::Project;
use crate::zelda2::sideview::config::SideviewAreas;
use crate::zelda2::sideview::Sideview;

/// Maintains a connectivity map between the overworld and sideview areas.
/// For any sideview area, this allows us to determine which overworld
/// connection connects to that area, thus allowing us to determine the
/// palace or town code for any room.
#[derive(Debug, Default)]
pub struct Connectivity {
    per_screen: Mutex<IndexMap<String, String>>,
    sequence: AtomicUsize,
}

impl Connectivity {
    pub fn scan(&self, project: &Project) -> Result<()> {
        for (bank, config) in project.config.bank.iter() {
            for (id, _) in config.overworld.iter() {
                self.explore_overworld(bank, id, project)?;
            }
        }
        let _ = self.sequence.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn sequence(&self) -> usize {
        self.sequence.load(Ordering::Relaxed)
    }

    pub fn get(&self, path: &str) -> Option<String> {
        self.per_screen.lock().unwrap().get(path).map(String::clone)
    }

    pub fn report(&self) {
        for (a, b) in self.per_screen.lock().unwrap().iter() {
            log::debug!("{a} => {b}");
        }
    }

    fn find_by_world(world: u8, project: &Project) -> Result<String> {
        for (bank, config) in project.config.bank.iter() {
            if let Some(svg) = &config.sideview {
                for (id, sideview) in svg.group.iter() {
                    if !sideview.is_background_layer && world == sideview.world {
                        return Ok(format!("/bank/{bank}/sideview/{id}"));
                    }
                }
            }
        }
        Err(Error::NotFound(format!("world {world}")).into())
    }

    // Iterate over all overworld connections and map out where they connect.
    fn explore_overworld(&self, bank: &str, id: &str, project: &Project) -> Result<()> {
        let overworld = format!("/bank/{bank}/overworld/{id}");
        let ov = project.data_ref::<Overworld>(&overworld)?;
        for (cid, conn) in ov.connection.iter() {
            let hidden = conn.hidden.unwrap_or(false);
            if (conn.y >= 128 && !hidden) || (conn.external && conn.dest_world == 0) {
                log::info!("{overworld}: skipping {cid}: {conn:?}");
                continue;
            }

            // Towns use worlds 1 and 2, but the editor treats them all as world 1.
            let world = if conn.dest_world == 2 {
                1
            } else {
                conn.dest_world
            };
            let (oconn, path) = if conn.dest_world == 0 && conn.dest_overworld == 0 {
                // Overworld to caves/grass/etc.
                (
                    format!("{overworld}/{cid}"),
                    format!("/bank/{bank}/sideview/{id}"),
                )
            } else {
                // Overworkd to another world (palace, town ,etc).
                // The game treats both DM and MZ as overworld 1, but the editor
                // comprehends DM/MZ as subworlds: 0-1 and 2-1.
                if conn.dest_overworld == 1 {
                    // Hack: prefer Maze Island because the game is coded to
                    // only permit a palace on MZ.
                    (
                        format!("/bank/2/overworld/1/{cid}"),
                        Self::find_by_world(world, project)?,
                    )
                } else {
                    (
                        format!("{overworld}/{cid}"),
                        Self::find_by_world(world, project)?,
                    )
                }
            };
            self.explore_per_screen(&oconn, &path, conn.area, conn.screen, project)?;
            let id = format!("{path}/{}", conn.area);
            self.per_screen
                .lock()
                .unwrap()
                .insert(oconn.clone(), id.clone());
            self.per_screen.lock().unwrap().insert(id, oconn);
        }
        Ok(())
    }

    // Explore sideview connections.
    fn explore_per_screen(
        &self,
        overworld: &str,
        path: &str,
        area: u8,
        screen: u8,
        project: &Project,
    ) -> Result<()> {
        let area = area as usize;
        let screen = screen as usize;
        let id = format!("{path}/{area}");
        let screen_id = format!("{id}/{screen}");

        if self.per_screen.lock().unwrap().get(&screen_id) == None {
            let sideview = project.data_ref::<Sideview>(&id)?;
            let config = project.config.get::<SideviewAreas>(&id)?;
            let width = sideview.map.width as usize;
            let (ss, se) = if width >= 2 {
                // Explore all four screens.
                (0, 3)
            } else if width == 0 {
                // Explore exactly one screen.
                (screen, screen)
            } else if screen & 1 == 0 {
                // Explore two screens, entry on an even screen number.
                (screen, screen + width)
            } else {
                // Explore two screens, entry on an odd screen number.
                (screen - width, screen)
            };
            for i in ss..=se {
                self.per_screen
                    .lock()
                    .unwrap()
                    .insert(format!("{id}/{i}"), overworld.into());
            }
            if area <= config.max_connectable_index {
                for i in ss..=se {
                    if let Some(c) = sideview.connection.get(i) {
                        if (c.area as usize) < config.length {
                            self.explore_per_screen(overworld, path, c.area, c.screen, project)?;
                        }
                    }
                }
            }
            if area <= config.max_door_index {
                for i in ss..=se {
                    if let Some(c) = sideview.door.get(i) {
                        if (c.area as usize) < config.length {
                            self.explore_per_screen(overworld, path, c.area, c.screen, project)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
