use crate::resources::*;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use rand::Rng;
use crate::components::MapField;
use std::collections::HashMap;

pub fn generate_map(
    // mut commands: Commands,
    // assert_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut map: ResMut<Map>,
    mut tilemap_query: Query<&mut Tilemap>,
){
    for mut tilemap in tilemap_query.iter_mut() {
        // `auto_chunk` を builder で実行していないので手動でchunkを追加する必要がある
        // chunk は 1つ
        tilemap.insert_chunk((0, 0)).unwrap();

        // chunkの縦・横のサイズを取得
        let chunk_width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let chunk_height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        // ワールド全体をgrassで埋める
        let mut tiles = Vec::new();
        map.fields = HashMap::with_capacity((chunk_width * chunk_height) as usize);
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                // -chunk_width/2 < x < chunk_width/2,  -chunk_height/2 < y < chunk_height/2
                let pos = (x - chunk_width / 2,
                                    y - chunk_height / 2); // -chunk_height/2 < y < chunk_height/2
                // デフォルトの tile set の z-order は 0
                // 小さい方が他よりも後ろにレンダリングされ, 0 は 最後尾 で 背景に使うのが最適
                let tile = Tile {
                    point: pos, // tileの座標
                    sprite_index: MapField::Grass as usize, // grassのindex
                    ..Default::default()
                };
                tiles.push(tile);
                map.fields.insert(pos, MapField::Grass);
            }
        }

        // 水で囲む
        for x in 0..chunk_width {
            let x = x - &chunk_width / 2;
            let tile_lower = (x, -(chunk_height / 2));
            let tile_upper = (x, chunk_height / 2 - 1);
            // マップ上部
            tiles.push(Tile {
                point: tile_lower,
                sprite_index: MapField::Water as usize,
                sprite_order: 1,
                ..Default::default()
            });
            // マップ下部
            tiles.push(Tile {
                point: tile_upper,
                sprite_index: MapField::Water as usize,
                sprite_order: 1,
                ..Default::default()
            });
            // 通れない場所として登録
            map.collisions.insert(tile_lower);
            map.collisions.insert(tile_upper);
            map.fields.insert(tile_lower, MapField::Water);
            map.fields.insert(tile_upper, MapField::Water);
        }
        for y in 0..chunk_height {
            let y = y - &chunk_height / 2;
            let tile_left = (-chunk_width / 2, y);
            let tile_right = (chunk_width / 2 - 1, y);
            // マップ左端
            tiles.push(Tile {
                point: tile_left,
                sprite_index: MapField::Water as usize,
                sprite_order: 1,
                ..Default::default()
            });
            // マップ右端
            tiles.push(Tile {
                point: tile_right,
                sprite_index: MapField::Water as usize,
                sprite_order: 1,
                ..Default::default()
            });
            // 通れない場所として登録
            map.collisions.insert(tile_left);
            map.collisions.insert(tile_right);
            map.fields.insert(tile_left, MapField::Water);
            map.fields.insert(tile_right, MapField::Water);
        }
        // ランダムに通行不可領域を追加する
        // 5% に設定
        let range = (chunk_width * chunk_height) as usize / 20;
        let mut rng = rand::thread_rng();
        for _ in 0..range {
            // 座標をランダムに選択
            let x: i32 = rng.gen_range(-chunk_width / 2 + 1, chunk_width / 2 - 1);
            let y: i32 = rng.gen_range(-chunk_height / 2 + 1, chunk_height / 2 - 1);
            let coord = (x, y, 0i32);
            if coord != (0, 0, 0) {
                tiles.push(Tile {
                    point: (x, y),
                    sprite_index: MapField::Water as usize,
                    ..Default::default()
                });
                map.collisions.insert((x, y));
                map.fields.insert((x, y), MapField::Water);
            }
        }
        // 他の地形を追加する
        // 10%
        let range = (chunk_width * chunk_height) as usize / 10;
        for _ in 0..range {
            let x = rng.gen_range(-chunk_width / 2 + 1, chunk_width / 2 - 1);
            let y = rng.gen_range(-chunk_height / 2 + 1, chunk_height / 2 - 1);
            // 50% の確率で山と森で分ける
            if rng.gen_bool(0.5) {
                tiles.push(Tile {
                    point: (x, y),
                    sprite_index: MapField::Mountain as usize,
                    sprite_order: 1,
                    ..Default::default()
                });
                map.fields.insert((x, y), MapField::Mountain);
            } else {
                tiles.push(Tile {
                    point: (x, y),
                    sprite_index: MapField::Forest as usize,
                    sprite_order: 1,
                    ..Default::default()
                });
                map.fields.insert((x, y), MapField::Forest);
            }
        }

        map.width = chunk_width as u32;
        map.height = chunk_height as u32;
        map.tile_size = tilemap.tile_width() as f32; // 正方形を仮定

        // TileMapにTileを追加
        tilemap.insert_tiles(tiles).unwrap();

        // ワールドに追加
        tilemap.spawn_chunk((0, 0)).unwrap();
    }
}