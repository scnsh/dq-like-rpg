use crate::components::MapField;
use std::collections::{HashMap, HashSet};

pub const MAP_SIZE: [u32; 2] = [64, 48];
pub const MAP_TEXTURE_SIZE: [u32; 2] = [16, 16];
pub const CHUNK_SIZE: [u32; 2] = [3, 3];

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub collisions: HashSet<(i32, i32)>,
    pub blinks: HashSet<(i32, i32)>,
    pub fields: HashMap<(i32, i32), MapField>,
}

impl Map {
    pub fn got_item(&mut self, pos: (i32, i32)) {
        let town = self.fields.get(&pos).unwrap().clone();
        match town {
            MapField::Town { item, visited: _ } => {
                self.fields.insert(
                    pos,
                    MapField::Town {
                        item,
                        visited: true,
                    },
                );
            }
            _ => panic!("got item should be called on 'Town' MapField"),
        }
    }
}
