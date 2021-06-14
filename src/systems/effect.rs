use bevy::prelude::*;
use crate::components::{EffectSpawnEvent, EffectKind, RenderLayer, Player, PlayerBattleState, render_layer, AssetHandles};
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
){
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
                }).id();
            // commands.entity(battle.entity.unwrap()).push_children(&[effect]);
            //TODO: 音を鳴らす
        }
    };
}

pub fn handle_effect(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Effect, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut player_query: Query<&mut Player>,
    mut enemy_query: Query<(&mut Transform), With<Enemy>>,
    mut battle: ResMut<Battle>,
){
    let elapsed = time.delta();
    let mut rng = rand::thread_rng();
    for (mut enemy_transform) in enemy_query.iter_mut(){
        for mut player in player_query.iter_mut() {
            for (entity, mut effect, mut sprite, texture_atlas_handle) in query.iter_mut() {
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
                    commands.entity(entity).despawn();
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
        }

    }
}

