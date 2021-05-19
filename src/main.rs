#![allow(clippy::all)]

mod components;
mod resources;
mod systems;
mod events;

use crate::components::*;
use crate::resources::*;
// use crate::systems::*;
use bevy::{
    // asset::LoadState,
    prelude::*,
    // sprite::TextureAtlasBuilder,
    // utils::HashSet,
    window::WindowMode,
};
use bevy_tilemap::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "RPG".to_string(),
            width: 1024.,
            height: 1024.,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<AssetHandles>()
        .init_resource::<GameState>()
        .init_resource::<Map>()
        .init_resource::<Battle>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins) // TileMap用のデフォルトプラグイン
        .add_state(GameState::default())
        .add_startup_system(systems::setup_cameras.system())
        .add_startup_system(systems::setup_title_ui.system())
        .add_system(systems::print_keyboard_event.system())
        .add_system(systems::input.system())
        .add_system_set(
            SystemSet::on_update(GameState::Title)
                .with_system(systems::gamestart_keyboard.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Title)
                .with_system(systems::cleanup_title_ui.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(systems::setup.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(systems::loading.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Generating)
                .with_system(systems::spawn_map_entity.system())
                .label("spawn_map")
        )
        .add_system_set(
            SystemSet::on_update(GameState::Generating)
                .with_system(systems::generate_map.system())
                .label("generate_map")
                .after("spawn_map")
        )
        .add_system_set(
            SystemSet::on_update(GameState::Generating)
                .with_system(systems::spawn_player.system())
                .with_system(systems::setup_status_ui.system())
                .after("generate_map")
        )
        // .add_system_set(
        //     SystemSet::on_enter(GameState::MapView)
        // )
        .add_system_set(
            SystemSet::on_update(GameState::MapView)
                .with_system(systems::animate_sprite_system.system())
                .with_system(systems::translation.system())
                .with_system(systems::update_status_ui.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::BattleView)
                .with_system(systems::setup_battle.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::BattleView)
                .with_system(systems::cleanup_battle_view.system())
        )
        // .add_system(character_movement.system())
        .run();
}
