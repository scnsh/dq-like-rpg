use bevy::prelude::*;
use crate::components::{EffectSpawnEvent, EffectKind, RenderLayer, Player, PlayerBattleState, render_layer, AssetHandles, EffectString};
use crate::resources::{GameState, ForState, Skill, Battle, Enemy};
use rand::Rng;
use bevy::math::Vec3Swizzles;

pub struct Effect {
    finish_timer: Timer,
    update_timer: Timer,
}

pub fn spawn_effect_event(
    mut commands: Commands,
    mut event_reader: EventReader<EffectSpawnEvent>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut battle: ResMut<Battle>,
    mut windows: ResMut<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
){
    let window = windows.get_primary_mut().unwrap();
    for event in event_reader.iter() {
        if let Some((texture_handle, columns)) = asset_handles.battle_effects.get(&event.kind){
            let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(120., 120.), *columns, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            println!("effect : {0:?}", battle.enemy_root_offset);
            let effect = commands
                .spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(battle.enemy_root_offset.x,
                                               battle.enemy_root_offset.y,
                                               render_layer(RenderLayer::BattleEffect) as f32),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Effect {
                    finish_timer: Timer::from_seconds(*columns as f32 * 0.1, false),
                    update_timer: Timer::from_seconds(0.1, true),
                })
                .insert(ForState {
                    states: vec![GameState::Battle],
                })
                .with_children(|child_builder| {
                    // 敵の攻撃の場合はダメージ表示用のテクスチャを表示する
                    if !&event.is_player_attack{
                        child_builder.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(window.width(), window.height())),
                            material: materials.add(Color::rgba(1., 0., 0., 0.1).into()),
                            transform: Transform {
                                translation: Vec3::new(0.,
                                                       0.,
                                                       -5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        }).insert(ForState {
                            states: vec![GameState::Battle],
                        });
                    }
                }).id();
            // commands.entity(battle.entity.unwrap()).push_children(&[effect]);

            commands.
                spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(35.), Val::Percent(100.)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            right: Val::Percent(0.),
                            top: Val::Percent(0.),
                            ..Default::default()
                        },
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .insert(Effect {
                    finish_timer: Timer::from_seconds(*columns as f32 * 0.1, false),
                    update_timer: Timer::from_seconds(0.1, true),
                })
                .insert(EffectString)
                .insert(ForState {
                    states: vec![GameState::Battle],
                })
                .with_children(|child_builder| {
                    // テキスト
                    child_builder.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.)),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        text: Text::with_section(
                            format!("{:?}",event.damage_or_heal),
                            TextStyle {
                                font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                font_size: 120.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        // visible: Visible {
                        //     is_visible: false,
                        //     is_transparent: false,
                        // },
                        ..Default::default()
                    });
                });
            //TODO: 音を鳴らす
        }
    };
}

pub fn handle_effect(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: QuerySet<(
        Query<(Entity, &mut Effect, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
        Query<(Entity, &mut Effect, &EffectString)>)>,
    mut player_query: Query<&mut Player>,
    mut enemy_query: Query<(&mut Transform), With<Enemy>>,
){
    let elapsed = time.delta();
    let mut rng = rand::thread_rng();
    for (mut enemy_transform) in enemy_query.iter_mut(){
        for mut player in player_query.iter_mut() {
            for (entity, mut effect, mut sprite, texture_atlas_handle) in query.q0_mut().iter_mut() {
                // Animationの更新
                effect.update_timer.tick(elapsed);
                if effect.update_timer.finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                }
                // Enemyを振動させる
                enemy_transform.translation.x = rng.gen_range(-1., 1.);
                enemy_transform.translation.y = rng.gen_range(-1., 1.);

                effect.finish_timer.tick(elapsed);
                if effect.finish_timer.finished() {
                    commands.entity(entity).despawn_recursive();
                    // プレイヤーの状態を更新する
                    match player.battle_state {
                        PlayerBattleState::Attack => player.battle_state = PlayerBattleState::Defense,
                        PlayerBattleState::Defense => player.battle_state = PlayerBattleState::Select,
                        _ => info!("unexpected effect")
                    }
                    // Enemyを一旦止める
                    enemy_transform.translation.x = 0.;
                    enemy_transform.translation.y = 0.;
                } else {

                }
            }
            for (entity, mut effect, _string) in query.q1_mut().iter_mut() {
                effect.finish_timer.tick(elapsed);
                if effect.finish_timer.finished() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }

    }
}

// Effectが残った状態で次の場面に切り替わらないように削除する
pub fn clean_up_effect(
    mut commands: Commands,
    mut query: QuerySet<(
        Query<(Entity, &Effect, &TextureAtlasSprite, &Handle<TextureAtlas>)>,
        Query<(Entity, &Effect, &EffectString)>)>,
){
    for (entity, _effect, _sprite, _texture_atlas_handle) in query.q0_mut().iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _effect, _string) in query.q1_mut().iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}