use crate::errors::*;
use crate::nes::Address;
use ron;
use serde::{Deserialize, Serialize};

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Sprite {
        pub id: String,
        pub offset: u8,
        pub name: String,
        pub chr: Address,
        pub palette: u8,
        pub size: (u32, u32),
        #[serde(default)]
        pub sprites: Vec<i32>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub id: String,
        pub item: Vec<Sprite>,
        pub fake: Vec<Sprite>,
        pub sprite_table: Address,
    }

    impl Config {
        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/items.ron")).unwrap()
        }

        pub fn find(&self, id: u8) -> Result<&Sprite> {
            for item in self.item.iter() {
                if id == item.offset {
                    return Ok(item);
                }
            }
            for item in self.fake.iter() {
                if id == item.offset {
                    return Ok(item);
                }
            }
            Err(ErrorKind::NotFound(format!("Item {}", id)).into())
        }
    }
}
