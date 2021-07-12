use crate::actions::Action;
use crate::map::Position;
use crate::AppState;
use bevy::prelude::*;

pub struct SetupPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            .add_system_set(
                SystemSet::on_enter(AppState::Menu).with_system(state_enter_despawn.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InGameMap).with_system(state_enter_despawn.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InGameExplore)
                    .with_system(state_enter_despawn.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InGameBattle)
                    .with_system(state_enter_despawn.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InGameEvent)
                    .with_system(state_enter_despawn.system()),
            );
    }
}

/// 1つの状態でのみ必要とされるエンティティにタグを付けるコンポーネント
pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(Debug)]
pub enum MapCameraState {
    Stop,
    Moving,
}

pub struct MapCamera {
    // どちらにむかう入力が入っているかを保持する
    pub direction: Option<Action>,
    pub destination: Position,
    pub state: MapCameraState,
}
impl Default for MapCamera {
    fn default() -> Self {
        MapCamera {
            direction: None,                        // 開始時は向きなし
            destination: Position { x: 0., y: 0. }, // 開始時は下向き
            state: MapCameraState::Stop,
        }
    }
}

#[derive(Clone, Copy)]
pub enum RenderLayer {
    MapBackGround, // マップの背景
    MapForeGround, // マップの前景
    Player,
    BattleBackGround, // バトルの背景
    BattleForeGround, // バトルの前景
    BattleEffect,     // バトルのエフェクト
}

pub fn render_layer(layer: RenderLayer) -> usize {
    match layer {
        RenderLayer::MapBackGround => 0,
        RenderLayer::MapForeGround => 1,
        RenderLayer::Player => 2,
        RenderLayer::BattleBackGround => 3,
        RenderLayer::BattleForeGround => 4,
        RenderLayer::BattleEffect => 100,
    }
}

pub fn setup_camera(mut commands: Commands) {
    // 2D用カメラを追加する
    let mut map_camera = OrthographicCameraBundle::new_2d();

    // 描画範囲を絞る
    map_camera.orthographic_projection.scale = 0.3;
    commands
        .spawn_bundle(map_camera)
        .insert(MapCamera::default())
        .insert(Position { x: 0., y: 0. })
        .insert(Timer::from_seconds(0.25, true));

    // UI用カメラを追加する
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn state_enter_despawn(
    mut commands: Commands,
    state: ResMut<State<AppState>>,
    query: Query<(Entity, &ForState<AppState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.entity(entity).despawn();
        }
    }
}
