use bevy::prelude::*;
use crate::events::GameEvent;
use crate::components::{Position, Player, MapCamera, position_to_field, UiEventText, CharacterStatus, Inventory, MapField, PlayerBattleState, EffectSpawnEvent, skill_to_effect};
use crate::resources::{Map, GameState, Battle, RunState, Skill, Enemy};
use rand::Rng;
use crate::systems::attack;

pub fn map_event_listener(
    mut events_reader: EventReader<GameEvent>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    mut battle: ResMut<Battle>,
    position_query: Query<&Position, With<MapCamera>>,
    mut player_status_query: Query<(&mut CharacterStatus, &mut Inventory), With<Player>>,
    mut enemy_status_query: Query<(&mut CharacterStatus, &Skill, &Enemy), Without<Player>>,
    mut runstate: ResMut<RunState>
){
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
                        map.got_item((position.x as i32, position.y as i32));
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
            _ => {panic!("unhandled event!!")}
        }
    }
}

pub fn battle_system(
    mut state: ResMut<State<GameState>>,
    mut player_status_query: Query<(&mut CharacterStatus, &mut Inventory, &mut Player), Changed<Player>>,
    mut enemy_status_query: Query<(&mut CharacterStatus, &Skill, &Enemy), Without<Player>>,
    mut effect_spawn_events: EventWriter<EffectSpawnEvent>,
    mut runstate: ResMut<RunState>
){
    for (mut player_status, mut inventory, mut player) in player_status_query.iter_mut() {
        for (mut enemy_status, skill, enemy) in enemy_status_query.iter_mut() {
            match player.battle_state {
                PlayerBattleState::Attack => {
                    //プレイヤーの攻撃を実施、ダメージor回復量を取得
                    let dmg_or_heal = attack(
                        &mut player_status,
                        &mut enemy_status,
                        inventory.skill()
                    );
                    // エフェクトを表示
                    // TODO: 数字も表示する
                    effect_spawn_events.send(EffectSpawnEvent {
                        kind: skill_to_effect(inventory.skill()),
                        damage_or_heal: dmg_or_heal,
                        is_player_attack: true
                    });
                },
                // 敵が攻撃を開始
                PlayerBattleState::Defense => {
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
                        player.battle_state = PlayerBattleState::Select
                    }
                    // 敵の攻撃を実施
                    let dmg = attack(
                        &mut enemy_status,
                        &mut player_status,
                        *skill
                    );
                    // エフェクトを表示
                    effect_spawn_events.send(EffectSpawnEvent {
                        kind: skill_to_effect(*skill),
                        damage_or_heal: dmg,
                        is_player_attack: false
                    });
                }
                // 自分の攻撃を選択
                PlayerBattleState::Select => {
                    // 自分のHPが0になったら敗北
                    if player_status.hp_current <= 0 {
                        runstate.event = Option::from(GameEvent::Lose);
                        state.set(GameState::Event).unwrap();
                        player.battle_state = PlayerBattleState::Select
                    }
                }
            }
        }
    }
}