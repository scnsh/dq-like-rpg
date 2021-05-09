use crate::components::*;
use bevy::prelude::*;

pub fn setup(
    mut tile_sprite_handles: ResMut<SpriteHandles>, // スプライトのハンドル集合
    asset_server: Res<AssetServer>, // アセットサーバー
) {
    // assets/images 以下の各ファイルを全て読み込む
    tile_sprite_handles.handles = asset_server.load_folder("images/tiles").unwrap();
}
