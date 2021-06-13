use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use bevy::app::Events;
use crate::events::GameEvent;
use crate::systems::attack;

pub fn player_battle_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    _map: Res<Map>,
    mut player_query: Query<(&mut Inventory, &mut Player)>,
) {
    // 十字キー操作
    let direction = if keyboard_input.just_pressed(KeyCode::Up) {
        // keyboard_input.reset(KeyCode::Up);
        Some(MoveDirection::Up)
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        // keyboard_input.reset(KeyCode::Down);
        Some(MoveDirection::Down)
    } else {
        None
    };

    for (mut inventory, mut _player) in player_query.iter_mut() {
        if let Some(direction) = direction {
            // インベントリのカーソル位置を更新
            match direction {
                MoveDirection::Up => inventory.decrement_index(),
                MoveDirection::Down => inventory.increment_index(),
                _ => {},
            }
        }
    }

    // 決定ボタン操作
    if keyboard_input.just_pressed(KeyCode::Return) {
        for (mut _inventory, mut player) in player_query.iter_mut() {
            println!("pressed with {0:?}", player.battle_state);
            if matches!(player.battle_state, PlayerBattleState::Select) {
                // state を更新
                player.battle_state = PlayerBattleState::Attack;
            }
        }
        keyboard_input.reset(KeyCode::Return);
    }
}