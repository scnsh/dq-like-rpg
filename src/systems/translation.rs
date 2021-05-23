use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use rand::Rng;
use crate::events::GameEvent;

// Positionを持つEntityに対してそれが更新されたら、Transoform の位置を移動させる
pub fn translation(
    map: Res<Map>,
    mut battle: ResMut<Battle>,
    mut state: ResMut<State<GameState>>,
    // mut query: Query<(&mut Transform, &Position), Changed<Position>>,
    mut queries: QuerySet<(
        Query<(&mut Transform, &Position), Changed<Position>>,
        Query<(&Position), (Changed<Position>, With<MapCamera>)>,
    )>,
    mut events_writer: EventWriter<GameEvent>,
){
    for (mut transform, position) in queries.q0_mut().iter_mut(){
        *transform =
            position_to_translation(&map, &(*position), transform.translation.z);
    }
    // エンカウント判定
    // 移動アニメーションを追加して、その動作の再生後に移動させる
    for (position) in queries.q1_mut().iter_mut(){
        // TODO: 地形に応じて確率を変えたい
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let enemy = field_to_enemy(
                position_to_field(&map, &(position.x, position.y)));
            events_writer.send(GameEvent::EnemyEncountered(enemy));
            // battle.enemy = enemy;
            // state.set(GameState::BattleView).unwrap();
        }
    }
}