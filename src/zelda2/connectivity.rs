use crate::errors::*;
use crate::idpath;
use crate::nes::IdPath;
use crate::zelda2::config::Config;
use crate::zelda2::overworld;
use crate::zelda2::project::Edit;
use crate::zelda2::sideview;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct Connectivity {
    per_screen: HashMap<IdPath, IdPath>,
}

impl Connectivity {
    pub fn from_rom(edit: &Rc<Edit>) -> Result<Self> {
        let mut result = Connectivity::default();
        let config = Config::get(&edit.config())?;
        for ocfg in config.overworld.map.iter() {
            result.explore_overworld(edit, ocfg, &config)?;
        }
        Ok(result)
    }

    pub fn overworld_connector(&self, room: &IdPath) -> Option<&IdPath> {
        self.per_screen.get(room)
    }

    pub fn print(&self) {
        for (k, v) in self.per_screen.iter() {
            println!("{} => {}", k, v);
        }
    }

    fn explore_overworld(
        &mut self,
        edit: &Rc<Edit>,
        ocfg: &overworld::config::Overworld,
        config: &Config,
    ) -> Result<()> {
        let ov = overworld::Overworld::from_rom(edit, ocfg.id.clone())?;
        for conn in ov.connector.iter() {
            let hidden = if let Some(h) = &conn.hidden {
                h.hidden
            } else {
                false
            };
            if (conn.y < 0 && !hidden) || (conn.external && conn.dest_world == 0) {
                continue;
            }
            // Towns use worlds 1 and 2, but the editor treats them all as
            // world 1.
            let world = if conn.dest_world == 2 {
                1
            } else {
                conn.dest_world
            };
            let (overworld, subworld) = if conn.dest_world == 0 && conn.dest_overworld == 0 {
                // Overworld to caves/grass/etc.
                (ocfg.overworld, ocfg.subworld)
            } else {
                // Overworld to another world (palace, town, etc)
                // The game treats both DM and MZ as "overworld 1", but the editor
                // comprends them as subworlds: 0-1 and 2-1.
                if conn.dest_overworld == 1 {
                    // Prefer Maze Island because the game is coded to only
                    // permit a palace on MZ.
                    (2, 1)
                } else {
                    (conn.dest_overworld, 0)
                }
            };
            let scfg = config.sideview.find_by_world(world, overworld, subworld)?;
            let dest = idpath!(scfg.id, conn.dest_map);
            self.explore_per_screen(edit, &conn.id, &dest, conn.entry, scfg)?;
            self.per_screen.insert(conn.id.clone(), dest);
        }
        Ok(())
    }

    fn explore_per_screen(
        &mut self,
        edit: &Rc<Edit>,
        ovconn: &IdPath,
        id: &IdPath,
        entry: usize,
        scfg: &sideview::config::SideviewGroup,
    ) -> Result<()> {
        let screen = id.extend(entry);
        if self.per_screen.get(&screen) == None {
            let sv = sideview::Sideview::from_rom(edit, id.clone())?;

            let width = sv.map.width as usize - 1;
            let (ss, se) = if width >= 2 {
                // Explore all four screens.
                (0, 3)
            } else if width == 0 {
                // Explore exactly one screen.
                (entry, entry)
            } else if entry & 1 == 0 {
                // Explore two screens, entry on an even room number.
                (entry, entry + width)
            } else {
                // Explore two screens, entry on an odd room number.
                (entry - width, entry)
            };

            for i in 0..4 {
                if i >= ss && i <= se {
                    let screen = id.extend(i);
                    self.per_screen.insert(screen, ovconn.clone());
                }
            }
            for i in 0..4 {
                let conn = sv.connection.get(i);
                if conn.is_some() && i >= ss && i <= se {
                    let c = conn.unwrap();
                    if c.dest_map < scfg.length {
                        self.explore_per_screen(
                            edit,
                            ovconn,
                            &id.pop().extend(c.dest_map),
                            c.entry,
                            scfg,
                        )?;
                    }
                }
                let conn = sv.door.get(i);
                if conn.is_some() && i >= ss && i <= se {
                    let c = conn.unwrap();
                    if c.dest_map < scfg.length {
                        self.explore_per_screen(
                            edit,
                            ovconn,
                            &id.pop().extend(c.dest_map),
                            c.entry,
                            scfg,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
