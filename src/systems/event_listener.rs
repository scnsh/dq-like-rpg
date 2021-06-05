use bevy::prelude::*;
use crate::events::GameEvent;
use crate::components::{Position, Player, MapCamera, position_to_field, UiEventText, CharacterStatus, Inventory};
use crate::resources::{Map, GameState, Battle, RunState, Skill};
use rand::Rng;
use crate::systems::attack;

// カメラを追従させる
pub fn event_listener(
    mut events_reader: EventReader<GameEvent>,
    map: Res<Map>,
    mut state: ResMut<State<GameState>>,
    mut battle: ResMut<Battle>,
    mut player_status_query: Query<(&mut CharacterStatus, &Inventory), With<Player>>,
    mut enemy_status_query: Query<(&mut CharacterStatus, &Skill), Without<Player>>,
    mut runstate: ResMut<RunState>
){
    // let mut new_events = Vec::new();

    for event in events_reader.iter() {
        match event {
            GameEvent::PlayerMoved => {
                // // カメラの移動
                // let new_position = queries.q0().single().unwrap().clone();
                // for mut position in queries.q1_mut().iter_mut(){
                //     position.x = new_position.x;
                //     position.y = new_position.y;
                // }

                // // エンカウント判定
                // // TODO: 地形に応じて確率を変えたい
                // let mut rng = rand::thread_rng();
                // if rng.gen_bool(0.1) {
                //     let enemy = field_to_enemy(
                //         position_to_field(&map, &(new_position.x, new_position.y)));
                //     // new_events.push(GameEvent::EnemyEncountered(enemy));
                //     battle.enemy = enemy;
                //     state.set(GameState::BattleView).unwrap();
                // }
            },
            GameEvent::EnemyEncountered(enemy) => {
                // battle.enemy = *enemy;
                // state.set(GameState::Battle).unwrap();
                // for mut ui_text in status_query.iter_mut() {
                //     ui_text.event = GameEvent::EnemyEncountered(battle.enemy);
                // }
                runstate.event = Option::from(GameEvent::EnemyEncountered(*enemy));
                state.set(GameState::Event).unwrap();
            },
            GameEvent::TownArrived => {
                // TODO: Town到着の効果を反映させる
                runstate.event = Option::from(GameEvent::TownArrived);
                state.set(GameState::Event).unwrap();
            }
            GameEvent::PlayerAttack => {
                for (mut player_status, inventory) in player_status_query.iter_mut() {
                    for (mut enemy_status, skill) in enemy_status_query.iter_mut() {
                        //プレイヤーの攻撃
                        attack(
                            &mut player_status,
                            &mut enemy_status,
                            inventory.skill()
                        );
                        //敵の攻撃
                        attack(
                            &mut enemy_status,
                            &mut player_status,
                            *skill
                        );
                    }
                }
            }
            _ => {}
        }
    }
    // for event in new_events {
    //     events_writer.send(event);
    // }
}
