use crate::audio::{AudioEvent, AudioKind};
use crate::character_status::CharacterStatus;
use crate::loading::TextureAssets;
use crate::map::{Map, Position};
use crate::player::Player;
use crate::setup::{render_layer, MapCamera, RenderLayer};
use crate::AppState;
use bevy::prelude::*;
use core::fmt;
use std::array::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

pub struct EnemiesPlugin;

// This plugin is responsible to controll the game audio
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<EnemyData>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGameBattle).with_system(setup_battle.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameBattle).with_system(update_battle.system()),
            );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Enemy {
    Goblin,
    Skeleton,
    Griffin,
    Boss,
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy::Goblin
    }
}
impl Display for Enemy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnemyStatus {
    name: Enemy,
    rate: i32,
    img: usize,
    hp: i32,
    at: i32,
    df: i32,
    skl: Skill,
}

pub struct EnemyData {
    pub data: HashMap<MapField, EnemyStatus>,
}
impl Default for EnemyData {
    fn default() -> Self {
        EnemyData {
            data: HashMap::<_, _>::from_iter(IntoIter::new([
                (
                    MapField::Grass,
                    EnemyStatus {
                        name: Enemy::Goblin,
                        rate: 20,
                        img: 0,
                        hp: 50,
                        at: 10,
                        df: 5,
                        skl: Skill::Sword,
                    },
                ),
                (
                    MapField::Forest,
                    EnemyStatus {
                        name: Enemy::Skeleton,
                        rate: 10,
                        img: 1,
                        hp: 100,
                        at: 20,
                        df: 10,
                        skl: Skill::Sword,
                    },
                ),
                (
                    MapField::Mountain,
                    EnemyStatus {
                        name: Enemy::Griffin,
                        rate: 5,
                        img: 2,
                        hp: 200,
                        at: 40,
                        df: 30,
                        skl: Skill::Wind,
                    },
                ),
                (
                    MapField::Castle,
                    EnemyStatus {
                        name: Enemy::Boss,
                        rate: 1,
                        img: 3,
                        hp: 999,
                        at: 99,
                        df: 99,
                        skl: Skill::Death,
                    },
                ),
            ])),
        }
    }
}

impl EnemyData {
    pub fn create(&self, map_field: &MapField, level: i32) -> CharacterStatus {
        let &enemy_status = &self.data[map_field];
        return CharacterStatus {
            name: enemy_status.name.to_string(),
            lv: level,
            exp: 0,
            hp_current: (enemy_status.hp as f32 * (0.5 + level as f32 / 2.)) as i32,
            hp_max: (enemy_status.hp as f32 * (0.5 + level as f32 / 2.)) as i32,
            mp_current: 0,
            mp_max: 0,
            attack: (enemy_status.at as f32 * (0.5 + level as f32 / 2.)) as i32,
            defence: (enemy_status.df as f32 * (0.5 + level as f32 / 2.)) as i32,
        };
    }
    pub fn field_to_enemy(&self, map_field: &MapField) -> Enemy {
        let &enemy_status = &self.data[map_field];
        return enemy_status.name;
    }
    pub fn field_to_rate(&self, map_field: &MapField) -> i32 {
        let &enemy_status = &self.data[map_field];
        return enemy_status.rate;
    }
    pub fn field_to_enemy_skill(&self, map_field: &MapField) -> Skill {
        let &enemy_status = &self.data[map_field];
        return enemy_status.skl;
    }
    pub fn image_index(&self, map_field: &MapField) -> usize {
        let enemy_status = &self.data[map_field];
        return enemy_status.img;
    }
}

pub fn setup_battle(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut battle: ResMut<Battle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
    mut windows: ResMut<Windows>,
    map: Res<Map>,
    enemy_data: Res<EnemyData>,
    player_query: Query<&CharacterStatus, With<Player>>,
    mut audio_event_writer: EventWriter<AudioEvent>,
) {
    // 参考
    // https://github.com/StarArawn/bevy_roguelike_prototype/blob/main/src/game/gameplay/scenes/battle.rs

    // プレイヤーの現在位置を取得
    let (_camera, player_transform, position) = player_camera_query.single().unwrap();
    let enemy = enemy_data.field_to_enemy(&map_field);
    let map_field = map.position_to_field(position);
    let player_status = player_query.single().unwrap();
    let enemy_status = enemy_data.create(
        &map_field,
        level(player_status.lv, enemy_data.field_to_enemy(&map_field)),
    );
    let enemy_skill = enemy_data.field_to_enemy_skill(&map_field);
    let enemy_sprite = texture_assets.get_handle_for_enemy(&enemy);

    // 敵の表示ウインドウの中心位置オフセットと表示のスケールを求める
    let window = windows.get_primary_mut().unwrap();
    //TODO: orthographic_projection_scale の値の影響をここで補正しないように  --> /.3
    let enemy_window_size = Vec2::new(
        window.width() as f32 * 2. / 3. / 3.,
        window.height() / 3. as f32,
    );
    let enemy_root_offset = Vec2::new(enemy_window_size.x - window.width() as f32 / (2. * 3.), 0.);
    //TODO: 16 をテクスチャから読み込む用に
    let enemy_scale = 1.;
    // let enemy_scale =
    //     cmp::min(enemy_window_size.x as i32, enemy_window_size.y as i32) as f32 / 16. * 0.5;

    // 背景と敵を追加
    let battle_entity = commands
        .spawn()
        // プレイヤーの現在位置を基準として表示する
        .insert(Transform::from_translation(Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            0.,
        )))
        .insert(GlobalTransform::default())
        .insert(ForState {
            states: vec![GameState::Battle],
        })
        .with_children(|child_builder| {
            // let mut battle_camera = OrthographicCameraBundle::new_2d();
            // battle_camera.orthographic_projection.scale = 0.3;
            // child_builder.spawn_bundle(battle_camera)
            //     .insert(BattleCamera)
            //     .insert(RenderLayers::layer(1));
            // 背景を追加
            child_builder
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(window.width(), window.height())),
                    material: materials.add(Color::BLACK.into()),
                    transform: Transform {
                        translation: Vec3::new(
                            0.,
                            0.,
                            render_layer(RenderLayer::BattleBackGround) as f32,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Battle],
                });
            // 敵を追加
            child_builder
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        enemy_root_offset.x,
                        enemy_root_offset.y,
                        0.,
                    )),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Battle],
                })
                .with_children(|child_builder| {
                    child_builder
                        .spawn_bundle(SpriteBundle {
                            material: materials.add(enemy_sprite.clone().into()),
                            transform: Transform {
                                translation: Vec3::new(
                                    0.,
                                    0.,
                                    render_layer(RenderLayer::BattleForeGround) as f32,
                                ),
                                scale: Vec3::new(enemy_scale, enemy_scale, 1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(enemy_status.clone())
                        .insert(enemy_skill)
                        .insert(enemy)
                        .insert(ForState {
                            states: vec![GameState::Battle],
                        });
                });
        })
        .id();

    // battle用のコンポーネントを保持
    battle.entity = Some(battle_entity);
    battle.enemy_root_offset = Vec2::new(
        player_transform.translation.x + enemy_root_offset.x,
        player_transform.translation.y + enemy_root_offset.y,
    );

    // 音楽を再生
    if matches!(enemy, Enemy::Boss) {
        audio_event_writer.send(AudioEvent::Play(AudioKind::BGMBattleLast));
    } else {
        audio_event_writer.send(AudioEvent::Play(AudioKind::BGMBattle));
    }
}
