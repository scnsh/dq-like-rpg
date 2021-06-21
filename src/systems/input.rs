use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use bevy::app::Events;
use crate::events::GameEvent;

pub fn input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    _map: Res<Map>,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<GameEvent>,
    mut player_query: Query<(&mut CharacterStatus, &mut Inventory, &mut Player, Entity)>,
    mut player_camera_query: Query<(&mut MapCamera, &mut Position)>,
    mut battle: ResMut<Battle>,
    runstate: Res<RunState>,
    enemy_data: Res<EnemyData>,
    mut effect_spawn_events: EventWriter<EffectSpawnEvent>,
    mut tilemap: Query<(Entity, &TileMap)>,
) {
    if state.current() == &GameState::Map {
        if let Some((mut map_camera, mut position)) = player_camera_query.iter_mut().next() {
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
                MoveDirection::None
            };
            if matches!(direction, MoveDirection::Up | MoveDirection::Down |
                MoveDirection::Left | MoveDirection::Right) {

                map_camera.direction = direction;

                // プレイヤーの位置を更新
                let mut new_position = map_camera.destination.clone();
                match direction {
                    MoveDirection::Up => new_position.y += 1.,
                    MoveDirection::Down => new_position.y -= 1.,
                    MoveDirection::Left => new_position.x -= 1.,
                    MoveDirection::Right => new_position.x += 1.,
                    _ => {},
                }
                // 障害物に接触していなければ更新
                if !_map.collisions.contains(&(new_position.x as i32, new_position.y as i32)) {
                    map_camera.destination = new_position;
                    // events.send(GameEvent::PlayerMoved);
                }
            }

            // デバッグ機能
            if keyboard_input.just_pressed(KeyCode::B) {
                let enemy = enemy_data.field_to_enemy(
                    &position_to_field(&_map, &position));
                battle.enemy = enemy.clone();
                // state.set(GameState::Battle).unwrap()
                events.send(GameEvent::EnemyEncountered(battle.enemy));

                keyboard_input.reset(KeyCode::B);
            }
            if keyboard_input.just_pressed(KeyCode::T) {
                events.send(GameEvent::TownArrived(Item::SpellFire(1), false));
                keyboard_input.reset(KeyCode::T);
            }
        }
    }


    if state.current() == &GameState::Battle {
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

        for (mut _character_status, mut inventory, _player, _entity) in player_query.iter_mut() {
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
            for (mut _character_status, mut _inventory, mut player, _entity) in player_query.iter_mut() {
                println!("pressed with {0:?}", player.battle_state);
                if matches!(player.battle_state, PlayerBattleState::Select) {
                    // state を更新
                    player.battle_state = PlayerBattleState::Attack;
                }
            }
            keyboard_input.reset(KeyCode::Return);
        }

        // デバッグ機能
        if keyboard_input.just_pressed(KeyCode::B) {
            state.set(GameState::Map).unwrap();
            keyboard_input.reset(KeyCode::B);
        }
        if keyboard_input.just_pressed(KeyCode::E) {
            effect_spawn_events.send(EffectSpawnEvent {
                kind: skill_to_effect(Skill::Wind),
                damage_or_heal: 10,
                is_player_attack: true
            });
            keyboard_input.reset(KeyCode::E);
        }
        if keyboard_input.just_pressed(KeyCode::I) {
            for (_player_camera, mut inventory, _player, _entity) in player_query.iter_mut() {
                inventory.add_item(Item::SpellFire(1));
            }
            keyboard_input.reset(KeyCode::I);
        }
    }

    if state.current() == &GameState::Event {
        // 決定ボタン操作
        if keyboard_input.just_pressed(KeyCode::Return) {
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
                    // Playerを削除する
                    for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                    // Tilemapを削除する
                    for (entity, _tilemap) in tilemap.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                    state.set(GameState::Title).unwrap();
                }
                // TODO: タイトルに戻って経験値引き継ぎ要素を入れる
                GameEvent::WinLast => {
                    // Playerを削除する
                    for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                    state.set(GameState::Title).unwrap();
                }
                _ => panic!("unexpected event"),
            }
        }
    }

    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::A) {
        for (mut player_status, _inventory, _player, _entity) in player_query.iter_mut() {
            player_status.hp_current -= 10;
        }
    }
}