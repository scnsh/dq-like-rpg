use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use bevy::app::Events;
use crate::events::GameEvent;

pub fn player_movement_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    _map: Res<Map>,
    mut state: ResMut<State<GameState>>,
    mut map_camera: Query<&mut MapCamera>
) {
    match state.current() {
        GameState::Map => {
            if let Some(mut map_camera) = map_camera.iter_mut().next() {
                // キーボード操作の入力を受け取る
                let direction = if keyboard_input.just_pressed(KeyCode::Up) {
                    MoveDirection::Up
                } else if keyboard_input.just_pressed(KeyCode::Down) {
                    MoveDirection::Down
                } else if keyboard_input.just_pressed(KeyCode::Left) {
                    MoveDirection::Left
                } else if keyboard_input.just_pressed(KeyCode::Right) {
                    MoveDirection::Right
                } else {
                    // 入力がなければ何もしない
                    return
                };
                map_camera.direction = direction;

                // プレイヤーの位置を更新
                let mut new_position = map_camera.destination.clone();
                match direction {
                    MoveDirection::Up => new_position.y += 1.,
                    MoveDirection::Down => new_position.y -= 1.,
                    MoveDirection::Left => new_position.x -= 1.,
                    MoveDirection::Right => new_position.x += 1.,
                }
                // 障害物に接触していなければ更新
                if !_map.collisions.contains(&(new_position.x as i32, new_position.y as i32)) {
                    map_camera.destination = new_position;
                    // events.send(GameEvent::PlayerMoved);
                }
            }
        },
        _ => info!("unhandled state"),
    }
}

// pub fn player_movement(
//     _map: Res<Map>,
//     mut player_camera_query: Query<(&MapCamera, &mut Position)>,
// ) {
//     if let Some((_player_camera, mut position)) = player_camera_query.iter_mut().next() {
//         // プレイヤーの位置を更新
//         let mut new_position = (position.x as i32, position.y as i32);
//         match direction {
//             MoveDirection::Up => new_position.1 += 1,
//             MoveDirection::Down => new_position.1 -= 1,
//             MoveDirection::Left => new_position.0 -= 1,
//             MoveDirection::Right => new_position.0 += 1,
//         }
//         // 障害物に接触していればその場にとどまる
//         if !_map.collisions.contains(&new_position) {
//             position.x = new_position.0;
//             position.y = new_position.1;
//             // events.send(GameEvent::PlayerMoved);
//         }
//     }
// }