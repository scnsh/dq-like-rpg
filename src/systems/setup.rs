use crate::components::*;
use bevy::prelude::*;

pub fn setup(
    mut asset_handles: ResMut<AssetHandles>, // Assetsのハンドル集合
    asset_server: Res<AssetServer>, // アセットサーバー
) {
    // assets/images 以下の各ファイルを読み込む

    // map作成用の texture atlas を読み込む
    asset_handles.tilemap = asset_server.load("images/tiles/land.png").clone();

    // プレイヤー用の texture を読み込む
    asset_handles.player = asset_server.load("images/player/you.png").clone();

    // バトル用の texture を読み込む
    asset_handles.battle_background = asset_server.load("images/battle/background.png").clone();

    // 敵のtextureを読み込む
    asset_handles.enemy_0 = asset_server.load("images/enemies/bird.png").clone();
}
