use crate::errors::*;
use crate::zelda2::sideview;
use ron;
use serde::{Deserialize, Serialize};

pub mod config {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum ObjectArea {
        Outdoors,
        Palace,
        Town,
        GreatPalace,
    }
    impl Default for ObjectArea {
        fn default() -> Self {
            ObjectArea::Outdoors
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum ObjectKind {
        Unknown,
        Small,
        Objset(i32),
        Extra,
    }
    impl Default for ObjectKind {
        fn default() -> Self {
            ObjectKind::Unknown
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Renderer {
        Unknown,
        Grid,
        Horizontal,
        Vertical,
        TopUnique,
        Item,
        Building,
        Window,
    }
    impl Default for Renderer {
        fn default() -> Self {
            Renderer::Grid
        }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Object {
        pub name: String,
        pub area: ObjectArea,
        pub kind: ObjectKind,
        pub render: Renderer,
        pub id: u8,
        pub width: usize,
        pub height: usize,
        pub metatile: Vec<u8>,
        pub fixed_y: Option<usize>,
        pub fixed_y_minus_param: Option<usize>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Config(pub Vec<Object>);

    impl Config {
        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/sideview_objects.ron"))
                .unwrap()
        }

        pub fn find(&self, id: u8, area: &ObjectArea, kind: &ObjectKind) -> Option<&Object> {
            for i in self.0.iter() {
                if i.id == id && i.area == *area && i.kind == *kind {
                    return Some(i);
                }
            }
            None
        }

        pub fn list(&self, area: &ObjectArea, kind: &ObjectKind) -> Vec<&Object> {
            let mut result = Vec::new();
            for i in self.0.iter() {
                if i.area == *area && i.kind == *kind {
                    result.push(i);
                }
            }
            result
        }
    }
}
