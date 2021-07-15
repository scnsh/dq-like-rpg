use crate::actions::Action;
use crate::map::Position;
use crate::AppState;
use bevy::prelude::*;

pub struct SetupPlugin;

// This plugin used for setup whole system
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

pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(Debug)]
pub enum MapCameraState {
    Stop,
    Moving,
}

pub struct MapCamera {
    pub direction: Option<Action>,
    pub destination: Position,
    pub state: MapCameraState,
}
impl Default for MapCamera {
    fn default() -> Self {
        MapCamera {
            direction: None,
            destination: Position { x: 0., y: 0. },
            state: MapCameraState::Stop,
        }
    }
}

#[derive(Clone, Copy)]
pub enum RenderLayer {
    MapBackGround,
    MapForeGround,
    Player,
    BattleBackGround,
    BattleForeGround,
    BattleEffect,
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

fn setup_camera(mut commands: Commands) {
    let mut map_camera = OrthographicCameraBundle::new_2d();

    map_camera.orthographic_projection.scale = 0.3;
    commands
        .spawn_bundle(map_camera)
        .insert(MapCamera::default())
        .insert(Position { x: 0., y: 0. })
        .insert(Timer::from_seconds(0.25, true));

    commands.spawn_bundle(UiCameraBundle::default());
}

fn state_enter_despawn(
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
