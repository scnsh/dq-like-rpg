use bevy::prelude::*;
use crate::events::GameEvent;
use crate::components::{Position, Player, MapCamera, position_to_field, UiEventText, CharacterStatus, Inventory, MapField};
use crate::resources::{Map, GameState, Battle, RunState, Skill, Enemy};
use rand::Rng;
use crate::systems::attack;

// カメラを追従させる
pub fn event_listener(
    mut events_reader: EventReader<GameEvent>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    mut battle: ResMut<Battle>,
    position_query: Query<&Position, With<MapCamera>>,
    mut player_status_query: Query<(&mut CharacterStatus, &mut Inventory), With<Player>>,
    mut enemy_status_query: Query<(&mut CharacterStatus, &Skill, &Enemy), Without<Player>>,
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
            GameEvent::TownArrived(item, visited) => {
                let position = position_query.single().unwrap();
                for (mut player_status, mut inventory) in player_status_query.iter_mut() {
                    if !visited {
                        // インベントリにアイテムを追加
                        inventory.add_item(item.clone());
                        // アイテム取得した状態を街に更新
                        map.got_item((position.x, position.y));
                        // 能力値計算(宝箱獲得で変わる可能性があるため)
                        let current_lv = player_status.lv;
                        println!("current_lv {}", player_status.lv);
                        player_status.level_up(current_lv, &inventory);
                        println!("get item");
                    }
                    // HP,MP回復
                    player_status.heal2max();

                    // Eventのシーンに遷移
                    runstate.event = Option::from(GameEvent::TownArrived(item.clone(), *visited));
                    state.set(GameState::Event).unwrap();
                }
            }
            GameEvent::PlayerAttack => {
                for (mut player_status, inventory) in player_status_query.iter_mut() {
                    for (mut enemy_status, skill, enemy) in enemy_status_query.iter_mut() {
                        //プレイヤーの攻撃
                        attack(
                            &mut player_status,
                            &mut enemy_status,
                            inventory.skill()
                        );
                        // 敵のHPが0になったら勝利
                        if enemy_status.hp_current <= 0 {
                            if matches!(enemy, Enemy::Boss){
                                // 最終戦闘に勝利
                                runstate.event = Option::from(GameEvent::WinLast);
                                state.set(GameState::Event).unwrap();
                            }else{
                                // 経験値を追加する
                                let levelup = player_status.add_exp(enemy_status.hp_max/10, &inventory);
                                runstate.event = Option::from(GameEvent::Win(levelup));
                                state.set(GameState::Event).unwrap();
                            }
                        }
                        //敵の攻撃
                        attack(
                            &mut enemy_status,
                            &mut player_status,
                            *skill
                        );
                        // 自分のHPが0になったら敗北
                        if player_status.hp_current <= 0 {
                            runstate.event = Option::from(GameEvent::Lose);
                            state.set(GameState::Event).unwrap();
                        }
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
