pub mod config;
pub mod connectivity;
pub mod enemyattr;
pub mod hacks;
pub mod import;
pub mod items;
pub mod objects;
pub mod overworld;
pub mod palette;
pub mod project;
pub mod python;
pub mod sideview;
pub mod start;
pub mod text_encoding;
pub mod text_table;
pub mod xp_spells;

use crate::errors::*;
use project::RomData;

pub fn edit_factory(kind: &str, id: Option<&str>) -> Result<Box<dyn RomData>> {
    match kind {
        "Enemy" => enemyattr::Enemy::create(id),
        "EnemyGroup" => enemyattr::EnemyGroup::create(id),
        "Hacks" => hacks::Hacks::create(id),
        "ImportRom" => import::ImportRom::create(id),
        "Overworld" => overworld::Overworld::create(id),
        "Palette" => palette::Palette::create(id),
        "PaletteGroup" => palette::PaletteGroup::create(id),
        "PythonScript" => python::PythonScript::create(id),
        "Sideview" => sideview::Sideview::create(id),
        "Start" => start::Start::create(id),
        "TextTable" => text_table::TextTable::create(id),
        "ExperienceTable" => xp_spells::ExperienceTable::create(id),
        "ExperienceTableGroup" => xp_spells::ExperienceTableGroup::create(id),
        _ => Err(ErrorKind::NotImplemented(kind.to_string()).into()),
    }
}
