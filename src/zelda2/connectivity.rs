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
    connector: HashMap<IdPath, IdPath>,
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

    pub fn overworld_connector(&self, id: &IdPath) -> Option<&IdPath> {
        self.connector.get(id)
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
            // The game treats both DM and MZ as "overworld 1", but the editor
            // comprends them as subworlds: 0-1 and 2-1.
            let (overworld, subworld) = if conn.dest_overworld == 1 {
                // prefer maze island
                (2, 1)
            } else {
                (conn.dest_overworld, 0)
            };
            let scfg = config.sideview.find_by_world(world, overworld, subworld)?;
            let dest = idpath!(scfg.id, conn.dest_map);
            self.explore_sideview(edit, &conn.id, &dest, scfg)?;
            self.connector.insert(conn.id.clone(), dest);
        }
        Ok(())
    }

    fn explore_sideview(
        &mut self,
        edit: &Rc<Edit>,
        ovconn: &IdPath,
        id: &IdPath,
        scfg: &sideview::config::SideviewGroup,
    ) -> Result<()> {
        if self.connector.get(id) == None {
            self.connector.insert(id.clone(), ovconn.clone());
            let sv = sideview::Sideview::from_rom(edit, id.clone())?;
            for c in sv.connection.iter() {
                if c.dest_map == scfg.length {
                    continue;
                }
                self.explore_sideview(edit, ovconn, &idpath!(scfg.id, c.dest_map), scfg)?;
            }
            for c in sv.door.iter() {
                if c.dest_map == scfg.length {
                    continue;
                }
                self.explore_sideview(edit, ovconn, &idpath!(scfg.id, c.dest_map), scfg)?;
            }
        }
        Ok(())
    }
}
