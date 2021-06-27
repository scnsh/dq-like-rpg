use crate::components::*;
use crate::resources::*;

use bevy::{asset::LoadState, prelude::*};

pub fn loading(
    asset_server: Res<AssetServer>, // アセットサーバー
    mut game_state: ResMut<State<GameState>>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    // textures: ResMut<Assets<Texture>>,
    mut audio_state: ResMut<AudioState>,
) {
    // asset handles に登録された assets が全て読み込まれているか確認する
    let mut ids = vec![
        asset_handles.tilemap.id,
        asset_handles.player.id,
        asset_handles.battle_background.id,
    ];
    for enemy in &asset_handles.enemies {
        ids.push(enemy.id);
    }
    for (_kind, (texture, _size)) in asset_handles.battle_effects.iter() {
        ids.push(texture.id);
    }
    for (_kind, audio) in &audio_state.sound_handles {
        ids.push(audio.id);
    }
    if matches!(
        asset_server.get_group_load_state(ids.iter().cloned()),
        LoadState::Loaded
    ) {
        audio_state.audio_loaded = true;
        // 次のStateへ進む
        game_state.set(GameState::Generating).unwrap();
    }
}
