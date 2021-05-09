use crate::components::*;
use crate::resources::*;

use bevy::{asset::LoadState, prelude::*};


pub fn loading(
    asset_server: Res<AssetServer>, // アセットサーバー
    mut game_state: ResMut<State<GameState>>,
    sprite_handles: Res<SpriteHandles>, // スプライト全体のハンドルとロード状態を管理
    textures: ResMut<Assets<Texture>>,
) {
    // texture と TextureAtlas を全てロードし終わったら以下の処理を実施する
    if asset_server.get_group_load_state(
        textures.iter().map(|(handle_id, _)| handle_id)) == LoadState::Loaded &&
        asset_server.get_group_load_state(
            sprite_handles.handles.iter().map(|handle| handle.id)) == LoadState::Loaded
    {
        // 次のStateへ進む
        game_state.set(GameState::Generating).unwrap();
    }
}