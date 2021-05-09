use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut state: ResMut<GameState>,
    mut player_position_query: Query<(Entity, &Player, &mut Position)>,
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
        for (entity, _player, mut position) in player_position_query.iter_mut() {
            match direction {
                MoveDirection::Up => position.y -= 2,
                MoveDirection::Down => position.y += 2,
                MoveDirection::Left => position.x -= 2,
                MoveDirection::Right => position.x += 2,
            }
        }
    }
}