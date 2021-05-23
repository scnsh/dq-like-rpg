use bevy::prelude::*;
use crate::events::GameEvent;
use crate::components::{Position, Player, MapCamera, position_to_field};
use crate::resources::{Map, field_to_enemy, GameState, Battle};
use rand::Rng;

// カメラを追従させる
pub fn event_listener(
    mut events_reader: EventReader<GameEvent>,
    // mut events_writer: EventWriter<GameEvent>,
    // mut queries: QuerySet<(
    //     Query<&Position, With<Player>>,
    //     Query<&mut Position, With<MapCamera>>,
    // )>,
    map: Res<Map>,
    mut state: ResMut<State<GameState>>,
    mut battle: ResMut<Battle>,
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
                battle.enemy = *enemy;
                state.set(GameState::BattleView).unwrap();
            },
            _ => {}
        }
    }
    // for event in new_events {
    //     events_writer.send(event);
    // }
}
