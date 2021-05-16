use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    _map: Res<Map>,
    mut _state: ResMut<GameState>,
    mut player_camera_queries: QuerySet<(
        Query<(&mut PlayerStatus, &Player, &mut Position)>,
        Query<(&MapCamera, &mut Position)>,
    )>,
){
    // // プレイヤー操作中以外は終了
    // if state != GameState::MapView {
    //     return;
    // }

    // キーボード操作の入力を受け取る
    let direction = if keyboard_input.just_pressed(KeyCode::Up){
        Some(MoveDirection::Up)
    } else if keyboard_input.just_pressed(KeyCode::Down){
        Some(MoveDirection::Down)
    } else if keyboard_input.just_pressed(KeyCode::Left){
        Some(MoveDirection::Left)
    } else if keyboard_input.just_pressed(KeyCode::Right){
        Some(MoveDirection::Right)
    } else {
        None
    };

    if let Some(direction) = direction {
        // プレイヤーの位置を更新
        for (_player_status, _player, mut position) in player_camera_queries.q0_mut().iter_mut() {
            match direction {
                MoveDirection::Up => position.y -= 2,
                MoveDirection::Down => position.y += 2,
                MoveDirection::Left => position.x -= 2,
                MoveDirection::Right => position.x += 2,
            }
        }
        // マップ上のカメラの位置を更新
        for (_map_camera, mut position) in player_camera_queries.q1_mut().iter_mut() {
            match direction {
                MoveDirection::Up => position.y -= 2,
                MoveDirection::Down => position.y += 2,
                MoveDirection::Left => position.x -= 2,
                MoveDirection::Right => position.x += 2,
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::A){
        for (mut player_status, _player, mut _position) in player_camera_queries.q0_mut().iter_mut() {
            player_status.hp_current -= 10;
        }
    }
}