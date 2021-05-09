#![allow(clippy::all)]

mod components;
mod resources;
mod systems;

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
        .init_resource::<SpriteHandles>()
        .init_resource::<GameState>()
        .init_resource::<Map>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins) // TileMap用のデフォルトプラグイン
        .add_state(GameState::default())
        .add_startup_system(systems::setup_cameras.system())
        .add_system_set(
            SystemSet::on_update(GameState::Title)
                .with_system(systems::gamestart_keyboard.system())
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(systems::setup.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading).with_system(systems::loading.system())
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
                .after("generate_map")
        )
        .add_system_set(
            SystemSet::on_update(GameState::MapView)
                .with_system(animate_sprite_system.system())
                .with_system(systems::input.system())
                .with_system(systems::translation.system())
        )
        // .add_system(character_movement.system())
        .run();
}

///
/// component
///


fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
){
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut(){
        // 時間を進ませる
        timer.tick(time.delta());
        // 時間が経過すれば、アトラスから次のIndexを設定する
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
