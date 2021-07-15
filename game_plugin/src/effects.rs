use crate::audio::{AudioEvent, AudioKind};
use crate::character_status::Skill;
use crate::enemies::{Battle, Enemy};
use crate::inventory::Item;
use crate::loading::{EffectsAtlas, FontAssets};
use crate::player::{Player, PlayerBattleState};
use crate::setup::{render_layer, ForState, RenderLayer};
use crate::AppState;
use bevy::prelude::*;
use rand::Rng;

pub struct EffectsPlugin;

// This plugin is responsible to control the game effects
impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<EffectEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGameBattle)
                    .with_system(effects_event.system())
                    .with_system(handle_effect.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameBattle).with_system(clean_up_effects.system()),
            );
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum EffectKind {
    Attack,
    Heal,
    Fire,
    Ice,
    Death,
    Arrow,
    Wind,
}

pub struct EffectEvent {
    pub kind: EffectKind,
    pub damage_or_heal: i32,
    pub is_player_attack: bool,
}

pub struct Effect {
    finish_timer: Timer,
    update_timer: Timer,
}

pub struct EffectString;

pub fn skill_to_effect(skill: Skill) -> EffectKind {
    match skill {
        Skill::Sword => EffectKind::Attack,
        Skill::Wind => EffectKind::Wind,
        Skill::Arrow => EffectKind::Arrow,
        Skill::Death => EffectKind::Death,
        Skill::Spell(item) => match item {
            Item::SpellHeal(_) => EffectKind::Heal,
            Item::SpellFire(_) => EffectKind::Fire,
            Item::SpellIce(_) => EffectKind::Ice,
            _ => panic!("select item cannot use."),
        },
    }
}

fn effects_event(
    mut commands: Commands,
    mut event_reader: EventReader<EffectEvent>,
    texture_atlas: Res<EffectsAtlas>,
    battle: Res<Battle>,
    mut windows: ResMut<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    font_assets: Res<FontAssets>,
    mut audio_event_writer: EventWriter<AudioEvent>,
) {
    let window = windows.get_primary_mut().unwrap();
    for event in event_reader.iter() {
        let texture_atlas_handle = texture_atlas.get_handle_for_effect(&event.kind);
        let effect_length = texture_atlas.get_length_for_effect(&event.kind);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        battle.enemy_root_offset.x,
                        battle.enemy_root_offset.y,
                        render_layer(RenderLayer::BattleEffect) as f32,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Effect {
                finish_timer: Timer::from_seconds(effect_length as f32 * 0.1, false),
                update_timer: Timer::from_seconds(0.1, true),
            })
            .insert(ForState {
                states: vec![AppState::InGameBattle],
            })
            .with_children(|child_builder| {
                if !&event.is_player_attack {
                    child_builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(window.width(), window.height())),
                            material: materials.add(Color::rgba(1., 0., 0., 0.1).into()),
                            transform: Transform {
                                translation: Vec3::new(0., 0., -5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![AppState::InGameBattle],
                        });
                }
            })
            .id();

        commands
            .spawn_bundle(NodeBundle {
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
                finish_timer: Timer::from_seconds(effect_length as f32 * 0.1, false),
                update_timer: Timer::from_seconds(0.1, true),
            })
            .insert(EffectString)
            .insert(ForState {
                states: vec![AppState::InGameBattle],
            })
            .with_children(|child_builder| {
                child_builder.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("{:?}", event.damage_or_heal),
                        TextStyle {
                            font: font_assets.pixel_mplus.clone(),
                            font_size: 120.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            });
        if matches!(&event.is_player_attack, true) {
            if matches!(&event.kind, EffectKind::Heal) {
                audio_event_writer.send(AudioEvent::Play(AudioKind::SEHeal));
            } else {
                audio_event_writer.send(AudioEvent::Play(AudioKind::SEAttack));
            }
        }
    }
}

fn handle_effect(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: QuerySet<(
        Query<(
            Entity,
            &mut Effect,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        )>,
        Query<(Entity, &mut Effect, &EffectString)>,
    )>,
    mut player_query: Query<&mut Player>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    let elapsed = time.delta();
    let mut rng = rand::thread_rng();
    for mut enemy_transform in enemy_query.iter_mut() {
        for mut player in player_query.iter_mut() {
            for (entity, mut effect, mut sprite, texture_atlas_handle) in query.q0_mut().iter_mut()
            {
                effect.update_timer.tick(elapsed);
                if effect.update_timer.finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index =
                        ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                }
                enemy_transform.translation.x = rng.gen_range(-1.0..1.0);
                enemy_transform.translation.y = rng.gen_range(-1.0..1.0);

                effect.finish_timer.tick(elapsed);
                if effect.finish_timer.finished() {
                    commands.entity(entity).despawn_recursive();
                    match player.battle_state {
                        PlayerBattleState::Attack => {
                            player.battle_state = PlayerBattleState::Defense
                        }
                        PlayerBattleState::Defense => {
                            player.battle_state = PlayerBattleState::Select
                        }
                        _ => info!("unexpected effect"),
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

fn clean_up_effects(
    mut commands: Commands,
    mut query: QuerySet<(
        Query<(Entity, &Effect, &TextureAtlasSprite, &Handle<TextureAtlas>)>,
        Query<(Entity, &Effect, &EffectString)>,
    )>,
) {
    for (entity, _effect, _sprite, _texture_atlas_handle) in query.q0_mut().iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _effect, _string) in query.q1_mut().iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
