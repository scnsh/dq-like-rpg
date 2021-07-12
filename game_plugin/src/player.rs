use crate::actions::Action;
use crate::character_status::CharacterStatus;
use crate::enemies::EnemyData;
use crate::events::GameEvent;
use crate::inventory::Inventory;
use crate::loading::PlayerAtlas;
use crate::map::{Field, Map, MiniMap, Position, MAP_SIZE};
use crate::setup::{render_layer, MapCamera, MapCameraState, RenderLayer};
use crate::AppState;
use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use bevy_tilemap::{Tile, Tilemap};
use rand::Rng;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGameMap)
                .with_system(spawn_player.system())
                .after("spawn_map"),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGameExplore)
                .with_system(animate_player.system())
                .with_system(move_player.system().label(PlayerMovement::Movement)),
        )
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(clean_up_player.system()));
    }
}

// システムラベル(SystemLabel)
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerMovement {
    Input,
    Movement,
}

// バトルの状態
#[derive(Debug)]
pub enum PlayerBattleState {
    Select,  // プレイヤーの入力待ち
    Attack,  // プレイヤー攻撃中
    Defense, // 相手の攻撃中
}

pub struct Player {
    pub battle_state: PlayerBattleState,
}

fn spawn_player(
    mut commands: Commands,
    map: Res<Map>,
    player: Query<Entity, With<Player>>,
    texture_atlas: Res<PlayerAtlas>,
    mut camera_query: Query<(Entity, &mut Transform, &mut Position, &mut MapCamera)>,
    mut app_state: ResMut<State<AppState>>,
) {
    // Playerを削除する(2日目以降)
    for entity in player.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (camera, mut transform, mut position, mut map_camera) in camera_query.iter_mut() {
        // カメラ位置をリセットする(GameOver後のリスタートも想定する)
        *transform =
            map.position_to_translation(&Position { x: 0., y: 0. }, transform.translation.z);
        *position = Position { x: 0., y: 0. };
        *map_camera = MapCamera::default();

        // 主人公を追加する
        // let you_sprite = asset_server.load("textures/player/player.png");
        // let you_sprite = asset_handles.player.clone();
        // let texture_atlas = TextureAtlas::from_grid(you_sprite, Vec2::new(14., 20.), 2, 1);
        // let texture_atlas_handle = texture_atlas.texture_atlases.add(texture_atlas);

        // let position = Position { x: 0, y: 0 };
        // let transform = position_to_translation(&map, &position, render_layer(RenderLayer::Player) as f32);
        let player = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.player.clone(),
                transform: Transform::from_xyz(
                    0.,
                    0.,
                    -transform.translation.z + render_layer(RenderLayer::Player) as f32,
                ),
                ..Default::default()
            })
            .insert(RenderLayers::layer(0))
            .insert(Player {
                battle_state: PlayerBattleState::Select,
            })
            .insert(CharacterStatus::default())
            .insert(Inventory::default())
            // .insert(position)
            .insert(Timer::from_seconds(0.5, true))
            .id();
        commands.entity(camera).push_children(&[player]);
    }

    // 次の画面に遷移する
    app_state.set(AppState::InGameExplore).unwrap();
}

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        // 時間を進ませる
        timer.tick(time.delta());
        // 時間が経過すれば、アトラスから次のIndexを設定する
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn move_player(
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

                let field = map.position_to_field(&position);
                match field {
                    Field::Town { item, visited } => {
                        // 街に着いた
                        events_writer.send(GameEvent::TownArrived(item, visited))
                    }
                    Field::Castle => {
                        // ボス戦闘
                        let enemy = enemy_data.field_to_enemy(&map.position_to_field(&position));
                        events_writer.send(GameEvent::EnemyEncountered(enemy))
                    }
                    Field::Grass | Field::Forest | Field::Mountain => {
                        // ランダム戦闘
                        let field = map.position_to_field(&position);
                        let mut rng = rand::thread_rng();
                        if rng.gen_bool((1. / enemy_data.field_to_rate(&field) as f32) as f64) {
                            let enemy = enemy_data.field_to_enemy(&field);
                            events_writer.send(GameEvent::EnemyEncountered(enemy));
                        }
                    }
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
            let mut prev_position = position.clone();
            // if let Some(mut tilemap) = mini_tilemap_query.iter_mut().next() {
            //     let mut prev_position = position.clone();
            //     let width = MAP_SIZE[0] as f32;
            //     let height = MAP_SIZE[1] as f32;
            //     let left = &width / 2. - 1.;
            //     let right = -&width / 2.;
            //     let top = &height / 2. - 1.;
            //     let bottom = -&height / 2.;
            //     if position.x < right {
            //         prev_position.x = left
            //     }
            //     if position.x > left {
            //         prev_position.x = right
            //     }
            //     if position.y > top {
            //         prev_position.y = bottom
            //     }
            //     if position.y < bottom {
            //         prev_position.y = top
            //     }
            //     // 移動元を戻す(端から端へワープする時以外、この場合は元の位置にいるわけでないため)
            //     let field = position_to_field(&map, &prev_position);
            //     tilemap
            //         .insert_tile(Tile {
            //             point: (prev_position.x as i32, prev_position.y as i32),
            //             sprite_index: field.sprite_index(),
            //             ..Default::default()
            //         })
            //         .unwrap();
            match move_direction {
                Some(Action::Left) => {
                    position.x = get_new_position(position.x, -velocity, map_camera.destination.x);
                }
                Some(Action::Right) => {
                    position.x = get_new_position(position.x, velocity, map_camera.destination.x);
                }
                Some(Action::Up) => {
                    position.y = get_new_position(position.y, velocity, map_camera.destination.y);
                }
                Some(Action::Down) => {
                    position.y = get_new_position(position.y, -velocity, map_camera.destination.y);
                }
                _ => {}
            }
            *transform = map.position_to_translation(&position, transform.translation.z);
            // info!(
            //     "{0:?}, {1:?}, {2:?}",
            //     position, map_camera.destination, prev_position
            // );

            if let Some(mut tilemap) = mini_tilemap_query.iter_mut().next() {
                update_mini_tilemap(&mut tilemap, &mut prev_position, &position, map);
            }
            // // 移動先を更新する
            // tilemap
            //     .insert_tile(Tile {
            //         point: (position.x as i32, position.y as i32),
            //         sprite_index: MapField::Player.sprite_index(),
            //         ..Default::default()
            //     })
            //     .unwrap();
        }
    }
}

fn get_new_position(position: f32, velocity: f32, destination: f32) -> f32 {
    if velocity < 0. {
        return (position + velocity).clamp(destination, position);
    }
    return (position + velocity).clamp(position, destination);
}

fn update_mini_tilemap(
    mini_tilemap: &mut Tilemap,
    old_position: &mut Position,
    new_position: &Position,
    map: Res<Map>,
) {
    let width = MAP_SIZE[0] as f32;
    let height = MAP_SIZE[1] as f32;
    // マップの外側から移動した場合はワープ元の位置を更新する
    if old_position.x < -&width / 2. {
        old_position.x = &width / 2. - 1.
    }
    if old_position.x > &width / 2. - 1. {
        old_position.x = -&width / 2.
    }
    if old_position.y > &height / 2. - 1. {
        old_position.y = -&height / 2.
    }
    if old_position.y < -&height / 2. {
        old_position.y = &height / 2. - 1.
    }
    let field = map.position_to_field(&old_position);
    // 移動元を更新
    mini_tilemap
        .insert_tile(Tile {
            point: (old_position.x as i32, old_position.y as i32),
            sprite_index: field.sprite_index(),
            ..Default::default()
        })
        .unwrap();

    // 移動先を更新する
    mini_tilemap
        .insert_tile(Tile {
            point: (new_position.x as i32, new_position.y as i32),
            sprite_index: Field::Player.sprite_index(),
            ..Default::default()
        })
        .unwrap();
}

fn clean_up_player(mut commands: Commands, mut player_query: Query<(&mut Player, Entity)>) {
    // Playerを削除する
    for (_player, entity) in player_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
