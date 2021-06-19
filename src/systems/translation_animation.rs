use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use rand::Rng;
use crate::events::GameEvent;

pub fn translation_animation(
    time: Res<Time>,
    mut map_camera_query: Query<(&mut Transform, &mut Position, &mut MapCamera)>,
    mut events_writer: EventWriter<GameEvent>,
    map: Res<Map>,
    enemy_data: Res<EnemyData>,
){
    if let Some((mut transform, mut position, mut map_camera)) = map_camera_query.iter_mut().next() {
        println!("{0:?}, {1:?}, {2:?}", map_camera.state, map_camera.destination, position);
        let velocity = time.delta_seconds() * 3.;
        if map_camera.destination == *position{
            // 移動状態から停止状態に遷移
            if matches!(map_camera.state, MapCameraState::Moving){
                map_camera.state = MapCameraState::Stop;
                // TODO: 地形に応じて確率を変えたい
                let field = position_to_field(&map, &position);
                match field {
                    MapField::Town{item, visited} => {
                        // 街に着いた
                        events_writer.send(GameEvent::TownArrived(item, visited))
                    },
                    MapField::Castle => {
                        // ボス戦闘
                        let enemy = enemy_data.field_to_enemy(
                            &position_to_field(&map, &position));
                        events_writer.send(GameEvent::EnemyEncountered(enemy))
                    },
                    MapField::Grass | MapField::Forest | MapField::Mountain => {
                        // ランダム戦闘
                        let field = &position_to_field(&map, &position);
                        let mut rng = rand::thread_rng();
                        if rng.gen_bool((1. / enemy_data.field_to_rate(field) as f32) as f64) {
                            let enemy = enemy_data.field_to_enemy(field);
                            events_writer.send(GameEvent::EnemyEncountered(enemy));
                        }
                    }
                    _ => {}
                }
            }
        }
        else{
            // 停止状態から移動状態に遷移
            if matches!(map_camera.state, MapCameraState::Stop){
                map_camera.state = MapCameraState::Moving;
            }
            // 位置を更新する
            if let move_direction = map_camera.direction{
                match move_direction {
                    MoveDirection::Left => {
                        position.x = get_new_position(position.x, -velocity, map_camera.destination.x);
                    },
                    MoveDirection::Right => {
                        position.x = get_new_position(position.x, velocity, map_camera.destination.x);
                    },
                    MoveDirection::Up => {
                        position.y = get_new_position(position.y, velocity, map_camera.destination.y);
                    },
                    MoveDirection::Down => {
                        position.y = get_new_position(position.y, -velocity, map_camera.destination.y);
                    },
                    _ => {}
                }
                // info!("{0:?}, {1:?}",position, map_camera.destination);
                *transform =
                    position_to_translation(&map, &position, transform.translation.z);
            }
        }
    }
}

fn get_new_position(
    position: f32,
    velocity: f32,
    destination: f32,
) -> f32 {
    println!("{0}, {1}, {2}", position, velocity, destination);
    if velocity < 0.{
        return (position + velocity).clamp(destination, position);
    }
    return (position + velocity).clamp(position, destination);
}