use crate::components::*;
use crate::events::GameEvent;
use crate::resources::*;
use bevy::prelude::*;
use bevy_tilemap::{Tile, Tilemap};
use rand::Rng;

pub fn translation_animation(
    time: Res<Time>,
    mut map_camera_query: Query<(&mut Transform, &mut Position, &mut MapCamera)>,
    mut events_writer: EventWriter<GameEvent>,
    map: Res<Map>,
    enemy_data: Res<EnemyData>,
    mut mini_tilemap_query: Query<&mut Tilemap, With<MiniMap>>,
) {
    if let Some((mut transform, mut position, mut map_camera)) = map_camera_query.iter_mut().next()
    {
        let velocity = time.delta_seconds() * 3.;
        if map_camera.destination == *position {
            // 移動状態から停止状態に遷移
            if matches!(map_camera.state, MapCameraState::Moving) {
                map_camera.state = MapCameraState::Stop;

                let field = position_to_field(&map, &position);
                match field {
                    MapField::Town { item, visited } => {
                        // 街に着いた
                        events_writer.send(GameEvent::TownArrived(item, visited))
                    }
                    MapField::Castle => {
                        // ボス戦闘
                        let enemy = enemy_data.field_to_enemy(&position_to_field(&map, &position));
                        events_writer.send(GameEvent::EnemyEncountered(enemy))
                    }
                    // MapField::Grass | MapField::Forest | MapField::Mountain => {
                    //     // ランダム戦闘
                    //     let field = &position_to_field(&map, &position);
                    //     let mut rng = rand::thread_rng();
                    //     if rng.gen_bool((1. / enemy_data.field_to_rate(field) as f32) as f64) {
                    //         let enemy = enemy_data.field_to_enemy(field);
                    //         events_writer.send(GameEvent::EnemyEncountered(enemy));
                    //     }
                    // }
                    _ => {}
                }
            }
        } else {
            // 停止状態から移動状態に遷移
            if matches!(map_camera.state, MapCameraState::Stop) {
                map_camera.state = MapCameraState::Moving;
            }
            // 位置を更新する
            let move_direction = map_camera.direction;
            // mini map 向け
            if let Some(mut tilemap) = mini_tilemap_query.iter_mut().next() {
                let mut prev_position = position.clone();
                let width = MAP_SIZE[0] as f32;
                let height = MAP_SIZE[1] as f32;
                let left = &width / 2. - 1.;
                let right = -&width / 2.;
                let top = &height / 2. - 1.;
                let bottom = -&height / 2.;
                if position.x < right {
                    prev_position.x = left
                }
                if position.x > left {
                    prev_position.x = right
                }
                if position.y > top {
                    prev_position.y = bottom
                }
                if position.y < bottom {
                    prev_position.y = top
                }
                // 移動元を戻す(端から端へワープする時以外、この場合は元の位置にいるわけでないため)
                let field = position_to_field(&map, &prev_position);
                tilemap
                    .insert_tile(Tile {
                        point: (prev_position.x as i32, prev_position.y as i32),
                        sprite_index: field.sprite_index(),
                        ..Default::default()
                    })
                    .unwrap();
                match move_direction {
                    MoveDirection::Left => {
                        position.x =
                            get_new_position(position.x, -velocity, map_camera.destination.x);
                    }
                    MoveDirection::Right => {
                        position.x =
                            get_new_position(position.x, velocity, map_camera.destination.x);
                    }
                    MoveDirection::Up => {
                        position.y =
                            get_new_position(position.y, velocity, map_camera.destination.y);
                    }
                    MoveDirection::Down => {
                        position.y =
                            get_new_position(position.y, -velocity, map_camera.destination.y);
                    }
                    _ => {}
                }
                // info!("{0:?}, {1:?}",position, map_camera.destination);
                *transform = position_to_translation(&map, &position, transform.translation.z);

                // 移動先を更新する
                tilemap
                    .insert_tile(Tile {
                        point: (position.x as i32, position.y as i32),
                        sprite_index: MapField::Player.sprite_index(),
                        ..Default::default()
                    })
                    .unwrap();
            }
        }
    }
}

fn get_new_position(position: f32, velocity: f32, destination: f32) -> f32 {
    if velocity < 0. {
        return (position + velocity).clamp(destination, position);
    }
    return (position + velocity).clamp(position, destination);
}
