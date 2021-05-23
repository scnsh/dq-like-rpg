use std::collections::{HashSet, HashMap};
use bevy_tilemap::Tilemap;
use crate::components::MapField;

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub collisions: HashSet<(i32, i32)>,
    pub fields: HashMap<(i32, i32), MapField>,
}

