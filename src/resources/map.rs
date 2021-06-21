use std::collections::{HashSet, HashMap};
use bevy_tilemap::Tilemap;
use crate::components::MapField;
use bevy::sprite::collide_aabb::collide;

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub collisions: HashSet<(i32, i32)>,
    pub fields: HashMap<(i32, i32), MapField>,
}

impl Map {
    pub fn got_item(&mut self, pos: (i32, i32)) {
        let mut town = self.fields.get(&pos).unwrap();
        match town{
            MapField::Town{item, visited} =>{
                self.fields.insert(pos, MapField::Town{item:*item, visited: true});
            }
            _ => panic!("got item should be called on 'Town' MapField")
        }
    }
}

