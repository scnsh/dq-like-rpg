use crate::character_status::CharacterStatus;
use crate::effects::skill_to_effect;
use crate::enemies::Enemy;
use crate::inventory::Inventory;
use crate::map::{Map, Position};
use crate::player::{Player, PlayerBattleState};
use crate::setup::MapCamera;
use crate::AppState;
use bevy::prelude::*;
use rand::Rng;

pub struct EventsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<GameEvent>()
            .init_resource::<RunState>()
            .add_system_set(
                SystemSet::on_update(AppState::InGameExplore).with_system(explore_events.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameBattle).with_system(battle_events.system()),
            );
    }
}

#[derive(Debug)]
pub enum GameEvent {
    // 戦闘開始のイベント
    EnemyEncountered(Enemy),

    // 街に到着
    TownArrived(Item, bool),

    // // プレイヤーが移動
    // PlayerMoved,

    // 勝利
    Win(bool),

    // 敗北
    Lose,

    // 最終戦勝利
    WinLast,
    // // 攻撃
    // PlayerAttack,

    // エンティティが動いた時に実行される
    // EntityMoved(EntityId),

    // // 箱がスポットに置かれた or 置かれてないの時に実行される
    // BoxPlacedOnSpot(IsCorrectSpot),
}

#[derive(Debug, Default)]
pub struct RunState {
    pub event: Option<GameEvent>,
}

fn explore_events(
    mut events_reader: EventReader<GameEvent>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    position_query: Query<&Position, With<MapCamera>>,
    mut player_status_query: Query<(&mut CharacterStatus, &mut Inventory), With<Player>>,
    mut runstate: ResMut<RunState>,
) {
    for event in events_reader.iter() {
        match event {
            GameEvent::EnemyEncountered(enemy) => {
                runstate.event = Option::from(GameEvent::EnemyEncountered(*enemy));
                state.set(GameState::Event).unwrap();
            }
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
                        player_status.level_up(current_lv, &inventory);
                    }
                    // HP,MP回復
                    player_status.heal2max();

                    // Eventのシーンに遷移
                    runstate.event = Option::from(GameEvent::TownArrived(item.clone(), *visited));
                    state.set(GameState::Event).unwrap();
                }
            }
            _ => {
                panic!("unhandled event!!")
            }
        }
    }
}

fn battle_events(
    mut state: ResMut<State<GameState>>,
    mut player_status_query: Query<
        (&mut CharacterStatus, &Inventory, &mut Player),
        Changed<Player>,
    >,
    mut enemy_status_query: Query<(&mut CharacterStatus, &Skill, &Enemy), Without<Player>>,
    mut effect_spawn_events: EventWriter<EffectSpawnEvent>,
    mut runstate: ResMut<RunState>,
) {
    for (mut player_status, inventory, mut player) in player_status_query.iter_mut() {
        for (mut enemy_status, skill, enemy) in enemy_status_query.iter_mut() {
            match player.battle_state {
                PlayerBattleState::Attack => {
                    //プレイヤーの攻撃を実施、ダメージor回復量を取得
                    let dmg_or_heal =
                        attack(&mut player_status, &mut enemy_status, inventory.skill());
                    // エフェクトを表示
                    // TODO: 数字も表示する
                    effect_spawn_events.send(EffectSpawnEvent {
                        kind: skill_to_effect(inventory.skill()),
                        damage_or_heal: dmg_or_heal,
                        is_player_attack: true,
                    });
                }
                // 敵が攻撃を開始
                PlayerBattleState::Defense => {
                    // 敵のHPが0になったら勝利
                    if enemy_status.hp_current <= 0 {
                        if matches!(enemy, Enemy::Boss) {
                            // 最終戦闘に勝利
                            runstate.event = Option::from(GameEvent::WinLast);
                            state.set(GameState::Event).unwrap();
                        } else {
                            // 経験値を追加する
                            let levelup =
                                player_status.add_exp(enemy_status.hp_max / 10, &inventory);
                            runstate.event = Option::from(GameEvent::Win(levelup));
                            state.set(GameState::Event).unwrap();
                        }
                        player.battle_state = PlayerBattleState::Select
                    }
                    // 敵の攻撃を実施
                    let dmg = attack(&mut enemy_status, &mut player_status, *skill);
                    // エフェクトを表示
                    effect_spawn_events.send(EffectSpawnEvent {
                        kind: skill_to_effect(*skill),
                        damage_or_heal: dmg,
                        is_player_attack: false,
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

// 攻撃計算
pub fn attack(
    own_status: &mut CharacterStatus,
    other_status: &mut CharacterStatus,
    skill: Skill,
) -> i32 {
    // 行動種類
    let (attack, defence, heal, mp) = skill2param(own_status, other_status, skill);
    // MPが足りているか
    if own_status.mp_current < mp {
        // heal = 0;
        // dmg = 0;
        0
    } else {
        //MP消費
        own_status.mp_current = (own_status.mp_current - mp).clamp(0, 999);

        if heal > 0 {
            //回復
            own_status.hp_current = (own_status.hp_current + heal).clamp(1, own_status.hp_max);
            heal
        } else {
            //ダメージ
            let mut rng = rand::thread_rng();
            let mut dmg = attack + rng.gen_range(0, &attack) - rng.gen_range(0, defence);
            dmg = dmg.clamp(1, 999);
            other_status.hp_current = (other_status.hp_current - dmg).clamp(0, 999);
            dmg
        }
    }
}

// (attack, defence, heal, mp)
pub fn skill2param(
    own_status: &CharacterStatus,
    other_status: &CharacterStatus,
    skill: Skill,
) -> (i32, i32, i32, i32) {
    match skill {
        Skill::Sword => (own_status.attack / 2, other_status.defence, 0, 0),
        Skill::Spell(item) => {
            let spl = [0, 1, 3, 6];
            match item {
                Item::SpellHeal(lv) => (
                    0,
                    other_status.defence,
                    (lv * lv * 50) as i32,
                    (10 * lv) as i32,
                ),
                Item::SpellFire(lv) => (
                    spl[lv as usize] * 20,
                    other_status.defence,
                    0,
                    (25 * lv) as i32,
                ),
                Item::SpellIce(lv) => (spl[lv as usize] * 15, 1, 0, (25 * lv) as i32),
                _ => panic!("unexpected item"),
            }
        }
        Skill::Arrow => (own_status.attack / 2, other_status.defence / 4, 0, 0),
        Skill::Wind => (own_status.attack / 2, other_status.defence / 2, 0, 0),
        Skill::Death => (own_status.attack / 2, other_status.defence, 0, 0),
    }
}
