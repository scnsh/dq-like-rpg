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
    runstate: Res<RunState>,
    enemy_data: Res<EnemyData>,
    mut effect_spawn_events: EventWriter<EffectSpawnEvent>,
) {
    // 決定ボタン操作
    if keyboard_input.just_pressed(KeyCode::Return) {
        match state.current() {
            GameState::Event => {
                let event = runstate.event.as_ref().unwrap();
                match event {
                    //バトル画面に遷移
                    GameEvent::EnemyEncountered(_enemy) => {
                        state.set(GameState::Battle).unwrap();
                    },
                    //マップ画面に遷移
                    GameEvent::TownArrived(_, _) => {
                        state.set(GameState::Map).unwrap();
                    },
                    //勝ったのでマップ画面に遷移
                    GameEvent::Win(levelup) => {
                        state.set(GameState::Map).unwrap();
                    }
                    //負けたのでタイトルに遷移
                    // TODO: 経験値を更新してマップに戻らせる
                    GameEvent::Lose => {
                        state.set(GameState::Title).unwrap();
                    }
                    // TODO: タイトルに戻って経験値引き継ぎ要素を入れる
                    GameEvent::WinLast => {
                        state.set(GameState::Title).unwrap();
                    }
                    _ => panic!("unexpected event"),
                }
            }
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::Return);
    }

    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::A) {
        for (mut player_status, _inventory, _player) in player_query.iter_mut() {
            player_status.hp_current -= 10;
        }
    }

    if keyboard_input.just_pressed(KeyCode::B) {
        match state.current() {
            GameState::Map => {
                for (_player_camera, mut position) in player_camera_query.iter_mut() {
                    let enemy = enemy_data.field_to_enemy(
                        &position_to_field(&_map, &position));
                    battle.enemy = enemy.clone();
                    // state.set(GameState::Battle).unwrap()
                    events.send(GameEvent::EnemyEncountered(battle.enemy));
                }
            },
            GameState::Battle => state.set(GameState::Map).unwrap(),
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::B);
    }

    if keyboard_input.just_pressed(KeyCode::I) {
        match state.current() {
            GameState::Battle => {
                for (_player_camera, mut inventory, _player) in player_query.iter_mut() {
                    inventory.add_item(Item::SpellFire(1));
                }
            },
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::I);
    }

    if keyboard_input.just_pressed(KeyCode::T) {
        match state.current() {
            GameState::Map => events.send(GameEvent::TownArrived(Item::SpellFire(1), false)),
            _ => info!("unhandled key input"),
        }
        keyboard_input.reset(KeyCode::T);
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        effect_spawn_events.send(EffectSpawnEvent {
            kind: skill_to_effect(Skill::Sword)
        });
        keyboard_input.reset(KeyCode::E);
    }
}