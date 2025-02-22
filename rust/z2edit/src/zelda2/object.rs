use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Renderer {
    #[default]
    Grid,
    Horizontal,
    Vertical,
    TopUnique,
    Item,
    Building,
    Window,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub render: Renderer,
    pub width: usize,
    pub height: usize,
    pub metatile: Vec<u8>,
    pub fixed_y: Option<usize>,
    pub fixed_y_minus_param: Option<usize>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BackgroundTiles {
    pub ceiling: [u8; 2],
    pub floor: [u8; 2],
    pub background: u8,
    pub alternate: Option<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderInfo {
    pub background: IndexMap<String, BackgroundTiles>,
    pub small: IndexMap<u8, Object>,
    pub objset0: IndexMap<u8, Object>,
    pub objset1: IndexMap<u8, Object>,
    pub extra: IndexMap<u8, Object>,
}
