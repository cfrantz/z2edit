use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::{ImStr, MenuItem};

use crate::nes::{IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::overworld::Connector;
use crate::zelda2::project::{Edit, EmulateAt, Project};

use crate::errors::*;

pub fn emulate_button(
    label: &ImStr,
    ui: &imgui::Ui,
    project: &Project,
    edit: &Rc<Edit>,
    connector: Option<&IdPath>,
    room: Option<&IdPath>,
) -> Result<()> {
    if ui.button(label) {
        emulate_at(project, None, connector, room)?;
    }
    if ui.popup_context_item(&im_str!("menu##{}", label)) {
        if MenuItem::new(im_str!("Emulate At Last Commit")).build(ui) {
            emulate_at(project, None, connector, room)?;
        }
        if MenuItem::new(im_str!("Emulate At This Commit")).build(ui) {
            emulate_at(project, Some(edit), connector, room)?;
        }
        ui.end_popup();
    }
    Ok(())
}

pub fn emulate_fourscreen(
    label: &ImStr,
    ui: &imgui::Ui,
    project: &Project,
    edit: &Rc<Edit>,
    room: &IdPath,
) -> Result<()> {
    if ui.button(label) {
        ui.open_popup(im_str!("screens"));
    }
    let mut ret = Ok(());
    ui.popup(im_str!("screens"), || {
        let mut r = Ok(());
        r = r.and(emulate_button(
            im_str!("Screen 1"),
            ui,
            project,
            edit,
            None,
            Some(&room.extend(0)),
        ));
        r = r.and(emulate_button(
            im_str!("Screen 2"),
            ui,
            project,
            edit,
            None,
            Some(&room.extend(1)),
        ));
        r = r.and(emulate_button(
            im_str!("Screen 3"),
            ui,
            project,
            edit,
            None,
            Some(&room.extend(2)),
        ));
        r = r.and(emulate_button(
            im_str!("Screen 4"),
            ui,
            project,
            edit,
            None,
            Some(&room.extend(3)),
        ));
        ret = r;
    });
    ret
}

pub fn emulate_at(
    project: &Project,
    edit: Option<&Rc<Edit>>,
    connector: Option<&IdPath>,
    room: Option<&IdPath>,
) -> Result<()> {
    let edit = edit.map_or_else(|| project.previous_commit(None), Rc::clone);

    // No specific location to emulate, so just emulate.
    if connector.is_none() && room.is_none() {
        return edit.emulate(None);
    }

    let (conn, page) = if let Some(room) = room {
        let connector = edit
            .overworld_connector(&room)
            .ok_or_else(|| ErrorKind::NotFound(format!("No connector for {}", room)))?;
        let conn = Connector::from_rom(&edit, connector)?;
        let page = room.usize_last()?;
        (conn, page)
    } else if let Some(connector) = connector {
        let conn = Connector::from_rom(&edit, connector.clone())?;
        let page = conn.entry;
        (conn, page)
    } else {
        unreachable!()
    };

    let config = Config::get(&edit.config())?;
    let overworld = &config.overworld.find(&conn.id)?;
    let mut index = conn.id.usize_last()?;
    let mut at = EmulateAt::default();

    let (region, bank) = if conn.dest_world == 0 {
        if conn.external {
            // Overworld 1 can be either DM or MZ depending on whether you
            // are transfering from overworld 0 or overworld 2.
            // Z2Edit represents overworld 1 as a subworld.  We search
            // available maps for the configured (overworld, subworld)
            // combination.
            let (ovw, sub) = if conn.dest_overworld == 1 {
                (overworld.overworld, 1)
            } else {
                (conn.dest_overworld, 0)
            };
            let mut r = 0;
            let mut b = 1;
            for map in config.overworld.map.iter() {
                if map.overworld == ovw && map.subworld == sub {
                    r = if map.subworld != 0 {
                        map.subworld as u8
                    } else {
                        map.overworld as u8
                    };
                    b = map.pointer.bank().unwrap().1 as u8;
                }
            }
            (r, b)
        } else {
            let r = if overworld.subworld != 0 {
                overworld.subworld as u8
            } else {
                overworld.overworld as u8
            };
            let b = overworld.pointer.bank().unwrap().1 as u8;
            (r, b)
        }
    } else {
        let r = conn.dest_overworld as u8;
        let rom = edit.rom.borrow();
        let b = rom
            .read(config.misc.world_to_bank + conn.dest_world)
            .unwrap_or(0);
        (r, b)
    };

    let town_code = config.overworld.town_code(index);
    if town_code.is_some() {
        // Always use the even connector for towns.
        index &= 0xFE;
    }

    at.bank = bank;
    at.region = region;
    at.world = conn.dest_world as u8;
    at.town_code = town_code.unwrap_or(0) as u8;
    at.palace_code = config.overworld.palace_code(index).unwrap_or(0) as u8;
    at.connector = index as u8;
    at.room = conn.dest_map as u8;
    at.page = page as u8;
    at.prev_region = overworld.overworld as u8;

    edit.emulate(Some(at))
}
