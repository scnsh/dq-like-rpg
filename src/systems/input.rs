use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use bevy::app::Events;
use crate::events::GameEvent;

pub fn input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    _map: Res<Map>,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<GameEvent>,
    mut player_query: Query<(&mut CharacterStatus, &mut Inventory, &Player)>,
    mut player_camera_query: Query<(&MapCamera, &mut Position)>,
    mut battle: ResMut<Battle>,
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

    if matches!(state.current(), GameState::MapView)
    {
        if let Some(direction) = direction {
            // プレイヤーの位置を更新
            for (_player_camera, mut position) in player_camera_query.iter_mut() {
                let mut new_position = (position.x as i32, position.y as i32);
                match direction {
                    MoveDirection::Up => new_position.1 += 1,
                    MoveDirection::Down => new_position.1 -= 1,
                    MoveDirection::Left => new_position.0 -= 1,
                    MoveDirection::Right => new_position.0 += 1,
                }
                if !_map.collisions.contains(&new_position) {
                    position.x = new_position.0;
                    position.y = new_position.1;
                    // events.send(GameEvent::PlayerMoved);
                    break;
                }
            }
        }
    }

    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::A){
        for (mut player_status, _inventory, _player) in player_query.iter_mut() {
            player_status.hp_current -= 10;
        }
    }

    if keyboard_input.just_pressed(KeyCode::B){
        match state.current() {
            GameState::MapView => {
                for (_player_camera, mut position) in player_camera_query.iter_mut() {
                    let enemy = field_to_enemy(
                        position_to_field(&_map, &(position.x, position.y)));
                    battle.enemy = enemy.clone();
                    state.set(GameState::BattleView).unwrap()
                }
            },
            GameState::BattleView => state.set(GameState::MapView).unwrap(),
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::B);
    }

    if keyboard_input.just_pressed(KeyCode::I){
        match state.current() {
            GameState::BattleView => {
                for (_player_camera, mut inventory, _player) in player_query.iter_mut() {
                    inventory.add_item(Item::SpellFire1);
                }
            },
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::I);
    }
    if keyboard_input.just_pressed(KeyCode::U){
        match state.current() {
            GameState::BattleView => {
                for (_player_camera, mut inventory, _player) in player_query.iter_mut() {
                    inventory.decrement_index();
                }
            },
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::U);
    }
    if keyboard_input.just_pressed(KeyCode::D){
        match state.current() {
            GameState::BattleView => {
                for (_player_camera, mut inventory, _player) in player_query.iter_mut() {
                    inventory.increment_index();
                }
            },
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::D);
    }
}