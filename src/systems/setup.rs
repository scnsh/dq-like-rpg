use crate::components::*;
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use std::collections::HashMap;

pub fn setup(
    mut asset_handles: ResMut<AssetHandles>, // Assetsのハンドル集合
    asset_server: Res<AssetServer>,          // アセットサーバー
    mut audio_state: ResMut<AudioState>,
) {
    // assets/textures 以下の各ファイルを読み込む

    // map作成用の texture atlas を読み込む
    asset_handles.tilemap = asset_server.load("textures/tiles/land.png").clone();
    asset_handles.mini_tilemap = asset_server.load("textures/tiles/miniland.png").clone();

    // プレイヤー用の texture を読み込む
    asset_handles.player = asset_server.load("textures/player/player.png").clone();

    // バトル用の texture を読み込む
    asset_handles.battle_background = asset_server.load("textures/battle/background.png").clone();
    // バトル用の effect を読み込む
    asset_handles.battle_effects = HashMap::new();
    asset_handles.battle_effects.insert(
        EffectKind::Attack,
        (asset_server.load("textures/effects/sword.png").clone(), 5),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Heal,
        (asset_server.load("textures/effects/heal.png").clone(), 8),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Fire,
        (asset_server.load("textures/effects/fire.png").clone(), 8),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Ice,
        (asset_server.load("textures/effects/ice.png").clone(), 8),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Death,
        (asset_server.load("textures/effects/death.png").clone(), 8),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Arrow,
        (asset_server.load("textures/effects/arrow.png").clone(), 9),
    );
    asset_handles.battle_effects.insert(
        EffectKind::Wind,
        (asset_server.load("textures/effects/wind.png").clone(), 8),
    );

    // 敵のtextureを読み込む
    asset_handles.enemies = Vec::new();
    asset_handles.enemies.push(
        asset_server
            .load("textures/enemies/GD_Goblin(Green).png")
            .clone(),
    );
    asset_handles.enemies.push(
        asset_server
            .load("textures/enemies/GD_Skeleton.png")
            .clone(),
    );
    asset_handles
        .enemies
        .push(asset_server.load("textures/enemies/GD_Griffin.png").clone());
    asset_handles
        .enemies
        .push(asset_server.load("textures/enemies/GD_Lich.png").clone());

    // assets/audio 以下の各ファイルを読み込む
    audio_state.channels.insert(
        String::from("bgm"),
        (
            AudioChannel::new("bgm".to_owned()),
            ChannelAudioState::default(),
        ),
    );
    audio_state.channels.insert(
        String::from("se"),
        (
            AudioChannel::new("se".to_owned()),
            ChannelAudioState::default(),
        ),
    );

    // bgm(ループ再生)
    audio_state.sound_handles.insert(
        AudioKind::BGMMap,
        asset_server.load("audio/bgm/bgm_maoudamashii_8bit01.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMBattle,
        asset_server.load("audio/bgm/bgm_maoudamashii_8bit18.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMLose,
        asset_server.load("audio/bgm/bgm_maoudamashii_8bit20.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMWin,
        asset_server.load("audio/bgm/bgm_maoudamashii_8bit24.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMBattleLast,
        asset_server.load("audio/bgm/bgm_maoudamashii_8bit25.ogg"),
    );
    // se(one-shot)
    audio_state.sound_handles.insert(
        AudioKind::SEAttack,
        asset_server.load("audio/se/se_maoudamashii_retro03.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::SEHeal,
        asset_server.load("audio/se/se_maoudamashii_retro08.ogg"),
    );
    audio_state.sound_handles.insert(
        AudioKind::SETown,
        asset_server.load("audio/se/se_maoudamashii_retro22.ogg"),
    );
}
