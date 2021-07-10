mod actions;
mod audio;
mod battle_actions;
mod character_status;
mod effects;
mod enemies;
mod event_actions;
mod events;
mod explore_actions;
mod inventory;
mod loading;
mod map;
mod menu;
mod player;
mod setup;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::battle_actions::BattleActionsPlugin;
use crate::enemies::EnemiesPlugin;
use crate::explore_actions::ExploreActionsPlugin;
use crate::inventory::InventoryPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

use crate::actions::ActionsPlugin;
use crate::effects::EffectsPlugin;
use crate::event_actions::EventActionsPlugin;
use crate::events::EventsPlugin;
use crate::setup::SetupPlugin;
use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct GamePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    InGameMap,
    InGameExplore,
    InGameBattle,
    InGameEvent,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Loading)
            .add_plugin(SetupPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(ExploreActionsPlugin)
            .add_plugin(BattleActionsPlugin)
            .add_plugin(EventActionsPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(EffectsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //     .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

// #![allow(clippy::all)]
//
// mod components;
// mod events;
// mod resources;
// mod systems;
//
// use crate::components::*;
// use crate::events::GameEvent;
// use crate::resources::*;
// use bevy::{prelude::*, window::WindowMode};
// use bevy_kira_audio::AudioPlugin;
// use bevy_tilemap::prelude::*;
//
// fn main() {
//     let mut app = App::build();
//     app.insert_resource(WindowDescriptor {
//         title: "dq-like-rpg".to_string(),
//         width: 1024.,
//         height: 768.,
//         vsync: false,
//         resizable: false,
//         mode: WindowMode::Windowed,
//         ..Default::default()
//     })
//     .add_plugins(DefaultPlugins);
//
//     #[cfg(target_arch = "wasm32")]
//     app.add_plugin(bevy_webgl2::WebGL2Plugin);
//
//     app.add_event::<GameEvent>()
//         .add_event::<EffectSpawnEvent>()
//         .add_event::<AudioEvent>()
//         .init_resource::<AssetHandles>()
//         .init_resource::<GameState>()
//         .init_resource::<EnemyData>()
//         .init_resource::<Map>()
//         .init_resource::<Inventory>()
//         .init_resource::<Battle>()
//         .init_resource::<RunState>()
//         .init_resource::<AudioState>()
//         .add_plugins(TilemapDefaultPlugins) // TileMap用のデフォルトプラグイン
//         .add_plugin(AudioPlugin)
//         .add_state(GameState::default())
//         .add_startup_system(systems::setup_cameras.system())
//         .add_system(systems::print_keyboard_event.system())
//         .add_system(systems::audio_event_listener.system())
//         .add_system(
//             systems::input
//                 .system()
//                 .label(PlayerMovement::Input)
//                 .before(PlayerMovement::Movement),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Title)
//                 .with_system(systems::setup_title_ui.system())
//                 .with_system(systems::state_enter_despawn.system()),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Title)
//                 .with_system(systems::gamestart_keyboard.system())
//                 .with_system(systems::update_title_ui.system()),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Loading)
//                 .with_system(systems::setup.system())
//                 .with_system(systems::setup_loading_ui.system())
//                 .with_system(systems::state_enter_despawn.system()),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Loading)
//                 .with_system(systems::loading.system())
//                 .with_system(systems::update_loading_ui.system()),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Generating)
//                 .with_system(systems::spawn_map_entity.system())
//                 .label("spawn_map"),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Generating)
//                 .with_system(systems::generate_map.system())
//                 .label("generate_map")
//                 .after("spawn_map"),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Generating)
//                 .with_system(systems::spawn_player.system())
//                 .after("generate_map"),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Map)
//                 .with_system(systems::setup_status_ui.system())
//                 .with_system(systems::setup_map_ui.system())
//                 .with_system(systems::state_enter_despawn.system()),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Map)
//                 .with_system(systems::animate_sprite_system.system())
//                 .with_system(
//                     systems::translation_animation
//                         .system()
//                         .label(PlayerMovement::Movement),
//                 )
//                 .with_system(systems::update_status_ui.system())
//                 .with_system(systems::map_event_listener.system()),
//         )
//         .add_system_set(
//             SystemSet::on_exit(GameState::Map).with_system(systems::clean_up_map.system()),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Battle)
//                 .with_system(systems::setup_status_ui.system())
//                 .with_system(systems::setup_battle.system())
//                 .with_system(systems::state_enter_despawn.system()),
//         )
//         .add_system_set(
//             SystemSet::on_update(GameState::Battle)
//                 .with_system(systems::update_enemy_status_ui.system())
//                 .with_system(systems::update_battle_inventory_ui.system())
//                 .with_system(systems::update_status_ui.system())
//                 .with_system(systems::battle_system.system())
//                 .with_system(systems::spawn_effect_event.system())
//                 .with_system(systems::handle_effect.system()),
//         )
//         .add_system_set(
//             SystemSet::on_exit(GameState::Battle).with_system(systems::clean_up_battle.system()),
//         )
//         .add_system_set(
//             SystemSet::on_enter(GameState::Event)
//                 .with_system(systems::setup_event_ui.system())
//                 .with_system(systems::state_enter_despawn.system()),
//         )
//         .add_system_set(
//             SystemSet::on_exit(GameState::Event).with_system(systems::clean_up_event.system()),
//         )
//         .run();
// }
