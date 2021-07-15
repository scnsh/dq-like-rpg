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

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerMovement {
    Movement,
}

#[derive(Debug)]
pub enum PlayerBattleState {
    Select,
    Attack,
    Defense,
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
    // Delete player for second play
    for entity in player.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (camera, mut transform, mut position, mut map_camera) in camera_query.iter_mut() {
        *transform =
            map.position_to_translation(&Position { x: 0., y: 0. }, transform.translation.z);
        *position = Position { x: 0., y: 0. };
        *map_camera = MapCamera::default();

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

    app_state.set(AppState::InGameExplore).unwrap();
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
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
            if matches!(map_camera.state, MapCameraState::Moving) {
                map_camera.state = MapCameraState::Stop;

                let field = map.position_to_field(&position);
                match field {
                    Field::Town { item, visited } => {
                        events_writer.send(GameEvent::TownArrived(item, visited))
                    }
                    Field::Castle => {
                        let enemy = enemy_data.field_to_enemy(&map.position_to_field(&position));
                        events_writer.send(GameEvent::EnemyEncountered(enemy))
                    }
                    Field::Grass | Field::Forest | Field::Mountain => {
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
            if matches!(map_camera.state, MapCameraState::Stop) {
                map_camera.state = MapCameraState::Moving;
            }
            let move_direction = map_camera.direction;
            // for mini map
            let mut prev_position = position.clone();
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

            if let Some(mut tilemap) = mini_tilemap_query.iter_mut().next() {
                update_mini_tilemap(&mut tilemap, &mut prev_position, &position, map);
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

fn update_mini_tilemap(
    mini_tilemap: &mut Tilemap,
    old_position: &mut Position,
    new_position: &Position,
    map: Res<Map>,
) {
    let width = MAP_SIZE[0] as f32;
    let height = MAP_SIZE[1] as f32;
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
    mini_tilemap
        .insert_tile(Tile {
            point: (old_position.x as i32, old_position.y as i32),
            sprite_index: field.sprite_index(),
            ..Default::default()
        })
        .unwrap();

    mini_tilemap
        .insert_tile(Tile {
            point: (new_position.x as i32, new_position.y as i32),
            sprite_index: Field::Player.sprite_index(),
            ..Default::default()
        })
        .unwrap();
}

fn clean_up_player(mut commands: Commands, mut player_query: Query<(&mut Player, Entity)>) {
    for (_player, entity) in player_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
