use bevy::prelude::*;
use crate::components::{EffectSpawnEvent, EffectKind, RenderLayer, Player, PlayerBattleState, render_layer, AssetHandles};
use crate::resources::{GameState, ForState, Skill, Battle};

pub struct Effect {
    finish_timer: Timer,
    update_timer: Timer,
}

pub fn spawn_effect_event(
    mut commands: Commands,
    mut event_reader: EventReader<EffectSpawnEvent>,
    asset_server: Res<AssetServer>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut battle: ResMut<Battle>,
){
    for event in event_reader.iter() {
        let (texture_name, duration) = match event.kind {
            // EffectKind::Attack => ("images/water.png", 1.0),
            // EffectKind::Heal => ("images/water.png", 1.0),
            // EffectKind::Spell => ("images/water.png", 1.0),
            EffectKind::Attack => ("images/effects/pipo-btleffect001.png", 0.5),
            EffectKind::Heal => ("images/effects/pipo-btleffect001.png", 0.5),
            EffectKind::Spell => ("images/effects/pipo-btleffect001.png", 0.5),
        };
        let texture_handle = asset_server.load(texture_name);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(120., 120.), 5, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // let enemy_sprite = asset_handles.enemies.get(1).unwrap();
        println!("effect : {0:?}", battle.enemy_root_offset);
        let effect = commands
            .spawn_bundle(SpriteSheetBundle{
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(battle.enemy_root_offset.x,
                                           battle.enemy_root_offset.y,
                                           render_layer(RenderLayer::BattleEffect) as f32),
                    // scale: Vec3::new(1.,
                    //                  1.,
                    //                  1.),
                    ..Default::default()
                },
                // material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .insert(Effect {
                finish_timer: Timer::from_seconds(duration, false),
                update_timer: Timer::from_seconds(duration/5., true),
            })
            .insert(ForState {
                states: vec![GameState::Battle],
            }).id();
        // commands.entity(battle.entity.unwrap()).push_children(&[effect]);
        //TODO: 音を鳴らす
    };
}

pub fn handle_effect(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Transform, &mut Effect, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut player_query: Query<&mut Player>,
){
    let elapsed = time.delta();
    for mut player in player_query.iter_mut() {
        for (entity, mut _transform, mut effect, mut sprite, texture_atlas_handle) in query.iter_mut() {
            // Animationの更新
            effect.update_timer.tick(elapsed);
            if effect.update_timer.finished() {
                println!("animation {0}", sprite.index);
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }

            effect.finish_timer.tick(elapsed);
            if effect.finish_timer.finished() {
                commands.entity(entity).despawn();
                // プレイヤーの状態を更新する
                match player.battle_state {
                    PlayerBattleState::Attack => player.battle_state = PlayerBattleState::Defense,
                    PlayerBattleState::Defense => player.battle_state = PlayerBattleState::Select,
                    _ => info!("unexpected effect")
                }
            } else {

            }
        }
    }
}

