use crate::audio::AudioKind;
use crate::effects::EffectKind;
use crate::enemies::Enemy;
use crate::menu::UiTitleText;
use crate::setup::ForState;
use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(AppState::Loading, AppState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .with_collection::<TileMapSpriteSheet>()
            .init_resource::<TileMapAtlas>()
            .with_collection::<PlayerSpriteSheet>()
            .init_resource::<PlayerAtlas>()
            .with_collection::<EffectsSpriteSheet>()
            .init_resource::<EffectsAtlas>()
            .build(app);
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(setup_loading.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(update_loading.system()),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/PixelMplus12-Regular.ttf")]
    pub pixel_mplus: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/bgm/bgm_maoudamashii_8bit01.ogg")]
    pub bgm_map: Handle<AudioSource>,
    #[asset(path = "audio/bgm/bgm_maoudamashii_8bit18.ogg")]
    pub bgm_battle: Handle<AudioSource>,
    #[asset(path = "audio/bgm/bgm_maoudamashii_8bit20.ogg")]
    pub bgm_lose: Handle<AudioSource>,
    #[asset(path = "audio/bgm/bgm_maoudamashii_8bit24.ogg")]
    pub bgm_win: Handle<AudioSource>,
    #[asset(path = "audio/bgm/bgm_maoudamashii_8bit25.ogg")]
    pub bgm_battle_last: Handle<AudioSource>,
    #[asset(path = "audio/se/se_maoudamashii_retro03.ogg")]
    pub attack: Handle<AudioSource>,
    #[asset(path = "audio/se/se_maoudamashii_retro08.ogg")]
    pub heal: Handle<AudioSource>,
    #[asset(path = "audio/se/se_maoudamashii_retro22.ogg")]
    pub town: Handle<AudioSource>,
}

impl AudioAssets {
    pub fn get_handle_for_audio(&self, kind: AudioKind) -> Handle<AudioSource> {
        match kind {
            AudioKind::BGMExplore => self.bgm_map.clone(),
            AudioKind::BGMBattle => self.bgm_battle.clone(),
            AudioKind::BGMBattleLast => self.bgm_battle_last.clone(),
            AudioKind::BGMLose => self.bgm_lose.clone(),
            AudioKind::BGMWin => self.bgm_win.clone(),
            AudioKind::SEAttack => self.attack.clone(),
            AudioKind::SEHeal => self.heal.clone(),
            AudioKind::SETown => self.town.clone(),
        }
    }
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/battle/background.png")]
    pub battle_background: Handle<Texture>,
    #[asset(path = "textures/enemies/GD_Goblin(Green).png")]
    pub enemy_goblin: Handle<Texture>,
    #[asset(path = "textures/enemies/GD_Skeleton.png")]
    pub enemy_skeleton: Handle<Texture>,
    #[asset(path = "textures/enemies/GD_Griffin.png")]
    pub enemy_griffin: Handle<Texture>,
    #[asset(path = "textures/enemies/GD_Lich.png")]
    pub enemy_lich: Handle<Texture>,
}

impl TextureAssets {
    pub fn get_handle_for_enemy(&self, enemy: &Enemy) -> Handle<Texture> {
        match enemy {
            &Enemy::Goblin => self.enemy_goblin.clone(),
            &Enemy::Skeleton => self.enemy_skeleton.clone(),
            &Enemy::Griffin => self.enemy_griffin.clone(),
            &Enemy::Boss => self.enemy_lich.clone(),
        }
    }
}

#[derive(AssetCollection)]
pub struct TileMapSpriteSheet {
    #[asset(path = "textures/tiles/land.png")]
    pub tilemap: Handle<Texture>,
    #[asset(path = "textures/tiles/miniland.png")]
    pub mini_tilemap: Handle<Texture>,
}

pub struct TileMapAtlas {
    pub tilemap: Handle<TextureAtlas>,
    pub mini_tilemap: Handle<TextureAtlas>,
}

impl FromWorld for TileMapAtlas {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<TileMapSpriteSheet>()
            .expect("TileMapSpriteSheet not loaded");
        let mut atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("TextureAtlases missing");
        TileMapAtlas {
            tilemap: atlases.add(TextureAtlas::from_grid(
                assets.tilemap.clone(),
                Vec2::new(16., 16.),
                6,
                1,
            )),
            mini_tilemap: atlases.add(TextureAtlas::from_grid(
                assets.mini_tilemap.clone(),
                Vec2::new(1., 1.),
                8,
                1,
            )),
        }
    }
}

#[derive(AssetCollection)]
pub struct PlayerSpriteSheet {
    #[asset(path = "textures/player/player.png")]
    pub player: Handle<Texture>,
}

pub struct PlayerAtlas {
    pub player: Handle<TextureAtlas>,
}

impl FromWorld for PlayerAtlas {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<PlayerSpriteSheet>()
            .expect("PlayerSpriteSheet not loaded");
        let mut atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("TextureAtlases missing");
        PlayerAtlas {
            player: atlases.add(TextureAtlas::from_grid(
                assets.player.clone(),
                Vec2::new(14., 20.),
                2,
                1,
            )),
        }
    }
}

#[derive(AssetCollection)]
pub struct EffectsSpriteSheet {
    #[asset(path = "textures/effects/sword.png")]
    pub sword: Handle<Texture>,
    #[asset(path = "textures/effects/heal.png")]
    pub heal: Handle<Texture>,
    #[asset(path = "textures/effects/fire.png")]
    pub fire: Handle<Texture>,
    #[asset(path = "textures/effects/ice.png")]
    pub ice: Handle<Texture>,
    #[asset(path = "textures/effects/death.png")]
    pub death: Handle<Texture>,
    #[asset(path = "textures/effects/arrow.png")]
    pub arrow: Handle<Texture>,
    #[asset(path = "textures/effects/wind.png")]
    pub wind: Handle<Texture>,
}

pub struct EffectsAtlas {
    sword: Handle<TextureAtlas>,
    heal: Handle<TextureAtlas>,
    fire: Handle<TextureAtlas>,
    ice: Handle<TextureAtlas>,
    death: Handle<TextureAtlas>,
    arrow: Handle<TextureAtlas>,
    wind: Handle<TextureAtlas>,
}

impl FromWorld for EffectsAtlas {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<EffectsSpriteSheet>()
            .expect("EffectsSpriteSheet not loaded");
        let mut atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("TextureAtlases missing");
        EffectsAtlas {
            sword: atlases.add(TextureAtlas::from_grid(
                assets.sword.clone(),
                Vec2::new(120., 120.),
                5,
                1,
            )),
            heal: atlases.add(TextureAtlas::from_grid(
                assets.heal.clone(),
                Vec2::new(120., 120.),
                8,
                1,
            )),
            fire: atlases.add(TextureAtlas::from_grid(
                assets.fire.clone(),
                Vec2::new(120., 120.),
                8,
                1,
            )),
            ice: atlases.add(TextureAtlas::from_grid(
                assets.ice.clone(),
                Vec2::new(120., 120.),
                8,
                1,
            )),
            death: atlases.add(TextureAtlas::from_grid(
                assets.death.clone(),
                Vec2::new(120., 120.),
                8,
                1,
            )),
            arrow: atlases.add(TextureAtlas::from_grid(
                assets.arrow.clone(),
                Vec2::new(120., 120.),
                9,
                1,
            )),
            wind: atlases.add(TextureAtlas::from_grid(
                assets.wind.clone(),
                Vec2::new(120., 120.),
                8,
                1,
            )),
        }
    }
}

impl EffectsAtlas {
    pub fn get_handle_for_effect(&self, kind: &EffectKind) -> Handle<TextureAtlas> {
        match kind {
            &EffectKind::Attack => self.sword.clone(),
            &EffectKind::Heal => self.heal.clone(),
            &EffectKind::Fire => self.fire.clone(),
            &EffectKind::Ice => self.ice.clone(),
            &EffectKind::Death => self.death.clone(),
            &EffectKind::Arrow => self.arrow.clone(),
            &EffectKind::Wind => self.wind.clone(),
        }
    }
    pub fn get_length_for_effect(&self, kind: &EffectKind) -> i32 {
        match kind {
            &EffectKind::Attack => 5,
            &EffectKind::Heal => 8,
            &EffectKind::Fire => 8,
            &EffectKind::Ice => 8,
            &EffectKind::Death => 8,
            &EffectKind::Arrow => 9,
            &EffectKind::Wind => 8,
        }
    }
}

pub fn setup_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 親ノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                // 子のノードは画面上の上から下に並べる
                // flex_direction: FlexDirection::ColumnReverse,
                // 子のノードは左右に対して中央にCenteringして並べる
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // NONE = 黒
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppState::Loading],
        })
        .with_children(|parent| {
            // 上部のタイトル
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(25.0)),
                        // ウインドウの外側のマージン
                        margin: Rect::all(Val::Px(20.0)),
                        // Vertical方向の中央揃え
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::BLACK.into()),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![AppState::Loading],
                })
                .with_children(|parent| {
                    // テキスト
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.)),
                                // Horizontal方向の中央揃え
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Loading",
                                TextStyle {
                                    font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                    font_size: 80.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![AppState::Loading],
                        })
                        .insert(Timer::from_seconds(0.5, true))
                        .insert(UiTitleText);
                });
        });
}

pub fn update_loading(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Text), With<UiTitleText>>,
) {
    for (mut timer, mut text) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if text.sections[0].value.len() > 10 {
                text.sections[0].value = format!("Loading");
            } else {
                text.sections[0].value = format!("{}.", text.sections[0].value);
            }
        }
    }
}
