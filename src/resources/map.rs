use std::collections::HashSet;
use bevy_tilemap::Tilemap;

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub collisions: HashSet<(i32, i32)>
}
