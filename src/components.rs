use crate::resources::*;
use bevy::{
    prelude::*
};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn position_to_translation(
    map: &Res<Map>,
    position: &Position,
    z: f32,
) -> Transform {
    Transform::from_translation(Vec3::new(
        (position.x as f32 - 1.) / 2. * map.tile_size,
        (-(position.y as f32) - 1.) / 2. * map.tile_size,
         z,
    ))
}

pub struct Player;

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Render {
    pub sprite_index: usize,
    pub z_order: usize,
}

// スプライトのハンドル集合
// 全てのスプライトのロードが終わったかを確認する
#[derive(Default, Clone)]
pub struct SpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub atlas_loaded: bool,
}

