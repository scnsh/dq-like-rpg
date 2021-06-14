use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

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
    // バトル用の effect を読み込む
    asset_handles.battle_effects = HashMap::new();
    asset_handles.battle_effects.insert(EffectKind::Attack, (asset_server.load("images/effects/sword.png").clone(), 5));
    asset_handles.battle_effects.insert(EffectKind::Heal, (asset_server.load("images/effects/heal.png").clone(), 8));
    asset_handles.battle_effects.insert(EffectKind::Spell, (asset_server.load("images/effects/fire.png").clone(), 8));

    // 敵のtextureを読み込む
    asset_handles.enemies = Vec::new();
    asset_handles.enemies.push(asset_server.load("images/enemies/goblin.png").clone());
    asset_handles.enemies.push(asset_server.load("images/enemies/elf.png").clone());
    asset_handles.enemies.push(asset_server.load("images/enemies/bird.png").clone());
    asset_handles.enemies.push(asset_server.load("images/enemies/boss.png").clone());
}
