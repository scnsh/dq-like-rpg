use bevy::prelude::*;
use bevy_tilemap::{Tilemap, Tile};
use crate::components::{MiniMap, MapField, position_to_field, Position};
use crate::resources::Map;

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut mini_tilemap_query: Query<&mut Tilemap, With<MiniMap>>,
    map: Res<Map>,
){
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut(){
        // 時間を進ませる
        timer.tick(time.delta());
        // 時間が経過すれば、アトラスから次のIndexを設定する
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;

            if let Some(mut tilemap) = mini_tilemap_query.iter_mut().next() {
                for blink in &map.blinks {
                    let mut map_field = MapField::Blink;
                    if sprite.index % 2 == 0{
                        map_field = position_to_field(&map, &Position { x: blink.0 as f32, y: blink.1 as f32 });
                    }
                    tilemap.insert_tile(Tile {
                        point: (blink.0, blink.1),
                        sprite_index: map_field.sprite_index(),
                        ..Default::default()
                    });
                }
            }
        }
    }

}
