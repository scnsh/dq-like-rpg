use crate::resources::*;
use bevy::{
    prelude::*
};
use std::fmt;
use std::fmt::Display;

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

pub struct MapCamera;

// タイトル画面UIのルート
pub struct UiTitleRoot;

// タイトル画面UIのルート
pub struct UiStatusText;

// プレイヤーのステータス
pub struct PlayerStatus {
    pub lv: i32,
    pub exp: i32,
    pub hp_current: i32,
    pub hp_max: i32,
    pub mp_current: i32,
    pub mp_max: i32,
    pub attack: i32,
    pub defence: i32,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        PlayerStatus{
            lv: 1,
            exp: 0,
            hp_current: 100,
            hp_max: 100,
            mp_current: 100,
            mp_max: 100,
            attack: 10,
            defence: 10
        }
    }
}

impl Display for PlayerStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Lv {0:>2} Exp {1:>3}\n\
                     HP {2:>3} / {3:>3}\n\
                     MP {4:>3} / {5:>3}\n\
                     AT {6:>3} DF {7:>3}\n",
               self.lv, self.exp, self.hp_current, self.hp_max,
               self.mp_current, self.mp_max, self.attack, self.defence)
    }
}

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
pub struct AssetHandles {
    pub tilemap: Handle<Texture>,
    pub player: Handle<Texture>
    // pub atlas_loaded: bool,
}

