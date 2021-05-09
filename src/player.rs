use bevy::{
    prelude::*,
    render::camera::Camera,
};
use bevy_tilemap::prelude::*;

use super::components::*;
use super::game::*;

#[derive(Default)]
pub struct Player {}

#[derive(Bundle)]
pub struct PlayderBundle {
    pub player: Player,
    pub position: Position,
    pub render: Render,
}

pub fn character_movement(
    game_state: Res<GameState>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut map_query: Query<(&mut Tilemap, &mut Timer)>,
    mut player_query: Query<(&mut Position, &Render, &Player)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    // world_props: Res<WorldProps>,
){
    // マップがまだロードされていなければ、待つ
    if !game_state.map_loaded {
        return;
    }

    for (mut map, mut timer) in map_query.iter_mut() {
        // マップのロード待ち
        timer.tick(time.delta());
        if !timer.finished(){
            continue;
        }

        for (mut position, render, mut player) in player_query.iter_mut(){
            for key in keyboard_input.get_pressed(){
                for (camera, mut camera_transform) in camera_query.iter_mut(){
                    // 現在位置を取得
                    let previous_position = *position;

                    // キャラクターを操作する
                    use KeyCode::*;
                    match key {
                        // 上方向への移動
                        W | Numpad8 | Up | K => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, 1),
                                &mut map,
                                // world_props.tilemap_width,
                                // world_props.tilemap_height,
                                // world_props.tile_size,
                            );
                        }
                        // 左方向への移動
                        A | Numpad4 | Left | H => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (-1, 0),
                                &mut map,
                                // world_props.tilemap_width,
                                // world_props.tilemap_height,
                                // world_props.tile_size,
                            );
                        }
                        // 下方向への移動
                        S | Numpad2 | Down | J => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (0, -1),
                                &mut map,
                                // world_props.tilemap_width,
                                // world_props.tilemap_height,
                                // world_props.tile_size,
                            );
                        }
                        // 右方向への移動
                        D | Numpad6 | Right | L => {
                            try_move_player(
                                &mut position,
                                &mut camera_transform.translation,
                                (1, 0),
                                &mut map,
                                // world_props.tilemap_width,
                                // world_props.tilemap_height,
                                // world_props.tile_size,
                            );
                        }
                        _ => {}
                    }

                    // スプライトを移動させる
                    // move_sprite(&mut map, previous_position, *position, render);
                }
            }
        }
    }
}


// プレイヤーの位置更新を試みる
pub fn try_move_player(
    position: &mut Position,
    camera_transition: &mut Vec3,
    delta_xy: (i32, i32),
    map: &mut Tilemap,
    // tilemap_width: i32,
    // tilemap_height: i32,
    // tile_size: i32,
    // collisions: &HashSet<(i32, i32)>,
) {
    let new_pos = (&position.x + delta_xy.0, &position.y + delta_xy.1);
    let width = (map.width().unwrap() * map.chunk_width()) as i32;
    let height = (map.height().unwrap() * map.chunk_height()) as i32;
    // // 移動できない場所の場合は更新しない
    // if !self.collisions.contains(&new_pos) {
    //     position.x = new_pos.0;
    //     position.y = new_pos.1;
    // }
    if 0 <= new_pos.0 && new_pos.0 < width
        && 0 <= new_pos.1 && new_pos.1 < height
    {
        position.x = new_pos.0;
        position.y = new_pos.1;
        // プレイヤー移動分に合わせてカメラの位置も更新する
        camera_transition.x = camera_transition.x + (delta_xy.0 as f32 * map.tile_width() as f32);
        camera_transition.y = camera_transition.y + (delta_xy.1 as f32 * map.tile_height() as f32);
    }
}