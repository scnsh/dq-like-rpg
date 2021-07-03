#![allow(clippy::all)]

mod components;
mod events;
mod resources;
mod systems;

use crate::components::*;
use crate::events::GameEvent;
use crate::resources::*;
use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::AudioPlugin;
use bevy_tilemap::prelude::*;

fn main() {
    App::build()
        .add_event::<GameEvent>()
        .add_event::<EffectSpawnEvent>()
        .add_event::<AudioEvent>()
        .insert_resource(WindowDescriptor {
            title: "dq-like-rpg".to_string(),
            width: 1024.,
            height: 768.,
            vsync: false,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<AssetHandles>()
        .init_resource::<GameState>()
        .init_resource::<EnemyData>()
        .init_resource::<Map>()
        .init_resource::<Inventory>()
        .init_resource::<Battle>()
        .init_resource::<RunState>()
        .init_resource::<AudioState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins) // TileMap用のデフォルトプラグイン
        .add_plugin(AudioPlugin)
        .add_state(GameState::default())
        .add_startup_system(systems::setup_cameras.system())
        .add_system(systems::print_keyboard_event.system())
        .add_system(systems::audio_event_listener.system())
        .add_system(
            systems::input
                .system()
                .label(PlayerMovement::Input)
                .before(PlayerMovement::Movement),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Title)
                .with_system(systems::setup_title_ui.system())
                .with_system(systems::state_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Title)
                .with_system(systems::gamestart_keyboard.system())
                .with_system(systems::update_title_ui.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(systems::setup.system())
                .with_system(systems::setup_loading_ui.system())
                .with_system(systems::state_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(systems::loading.system())
                .with_system(systems::update_loading_ui.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Generating)
                .with_system(systems::spawn_map_entity.system())
                .label("spawn_map"),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Generating)
                .with_system(systems::generate_map.system())
                .label("generate_map")
                .after("spawn_map"),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Generating)
                .with_system(systems::spawn_player.system())
                .after("generate_map"),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Map)
                .with_system(systems::setup_status_ui.system())
                .with_system(systems::setup_map_ui.system())
                .with_system(systems::state_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Map)
                .with_system(systems::animate_sprite_system.system())
                .with_system(
                    systems::translation_animation
                        .system()
                        .label(PlayerMovement::Movement),
                )
                .with_system(systems::update_status_ui.system())
                .with_system(systems::map_event_listener.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Map).with_system(systems::clean_up_map.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Battle)
                .with_system(systems::setup_status_ui.system())
                .with_system(systems::setup_battle.system())
                .with_system(systems::state_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Battle)
                .with_system(systems::update_enemy_status_ui.system())
                .with_system(systems::update_battle_inventory_ui.system())
                .with_system(systems::update_status_ui.system())
                .with_system(systems::battle_system.system())
                .with_system(systems::spawn_effect_event.system())
                .with_system(systems::handle_effect.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Battle).with_system(systems::clean_up_battle.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Event)
                .with_system(systems::setup_event_ui.system())
                .with_system(systems::state_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Event).with_system(systems::clean_up_event.system()),
        )
        .run();
}
