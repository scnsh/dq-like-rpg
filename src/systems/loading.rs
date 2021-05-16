use crate::components::*;
use crate::resources::*;

use bevy::{asset::LoadState, prelude::*};


pub fn loading(
    asset_server: Res<AssetServer>, // アセットサーバー
    mut game_state: ResMut<State<GameState>>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    // textures: ResMut<Assets<Texture>>,
) {
    // asset handles に登録された テクスチャが全て読み込まれているか確認する
    let ids = &[
        asset_handles.tilemap.id,
        asset_handles.player.id
    ];
    if matches!(
        asset_server.get_group_load_state(ids.iter().cloned()),
        LoadState::Loaded
    ){
        // 次のStateへ進む
        game_state.set(GameState::Generating).unwrap();
    }
}