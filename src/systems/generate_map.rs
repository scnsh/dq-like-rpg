use crate::resources::*;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::components::{MapField, MiniMap};
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub fn generate_map(
    mut map: ResMut<Map>,
    mut tilemap_query: QuerySet<(
        Query<(&mut Tilemap, Without<MiniMap>)>,
        Query<(&mut Tilemap, With<MiniMap>)>,
    )>,
) {
    // chunkの縦・横のサイズを取得
    let width = MAP_SIZE[0] as i32;
    let height = MAP_SIZE[1] as i32;

    // ワールド全体をgrassで埋める
    let mut fields = HashMap::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            // -chunk_width/2 < x < chunk_width/2,  -chunk_height/2 < y < chunk_height/2
            let pos = (x - width / 2, y - height / 2); // -chunk_height/2 < y < chunk_height/2
                                                       // デフォルトの tile set の z-order は 0
                                                       // // 小さい方が他よりも後ろにレンダリングされ, 0 は 最後尾 で 背景に使うのが最適
            fields.insert(pos, MapField::Grass);
        }
    }

    // 山・森・水を配置する
    let mut rng = rand::thread_rng();
    for y in 0..height {
        for x in 0..width {
            let pos = (x - width / 2, y - height / 2); // -chunk_height/2 < y < chunk_height/2
                                                       // 1/60 の確率で山生成開始
            if rng.gen_bool(1. / 60.) {
                // 山の散布回数
                let max = rng.gen_range(10, 60);
                for _i in 0..max {
                    // ランダム移動で山を生成
                    let pos = (
                        (pos.0 + rng.gen_range(-3, 4)).clamp(-&width / 2, &width / 2 - 1),
                        (pos.1 + rng.gen_range(-3, 4)).clamp(-&height / 2, &height / 2 - 1),
                    );
                    fields.insert(pos, MapField::Mountain);
                }
            }
            // 1/12 で水
            // マップ端には作らないように(ワープした先が水にならないように)
            if 0 < y && y < height - 1 && 0 < x && x < width - 1 {
                if rng.gen_bool(1. / 12.) {
                    fields.insert(pos, MapField::Water);
                }
            }
            // 1/6 で森
            if rng.gen_bool(1. / 6.) {
                fields.insert(pos, MapField::Forest);
            }
        }
    }

    // ゲーム開始位置付近は平地にする
    for y in -1..2 {
        for x in -1..2 {
            // let pos = (x, y); // -chunk_height/2 < y < chunk_height/2
            fields.insert((x, y), MapField::Grass);
        }
    }

    // 敵城の生成 マップ周縁(0.8 ~ 1.0、0~0.2の比率)に生成
    let castle_x = (width as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - width / 2;
    let castle_y = (height as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - height / 2;
    fields.insert((castle_x, castle_y), MapField::Castle);

    // 街の生成 開始位置を避けて (0.55~1.0, 0~0.45の比率)に生成
    for item in generate_items() {
        let town_x =
            (width as f32 * rng_multi_range((0.05, 0.45), (0.55, 0.95))) as i32 - width / 2;
        let town_y =
            (height as f32 * rng_multi_range((0.05, 0.45), (0.55, 0.95))) as i32 - height / 2;
        match fields[&(town_x, town_y)] {
            // 街や城と重複した場合は追加しない
            MapField::Town {
                item: _,
                visited: _,
            } => continue,
            MapField::Castle => continue,
            _ => {}
        }
        fields.insert(
            (town_x, town_y),
            MapField::Town {
                item,
                visited: false,
            },
        );
    }

    // タイルを追加
    let mut tiles = Vec::new();
    let mut mini_tiles = Vec::new();
    map.collisions = HashSet::new();
    map.fields = HashMap::new();
    for (pos, field) in fields.iter_mut() {
        // マップ外にも描画する
        for x in 0..CHUNK_SIZE[0] as i32 {
            for y in 0..CHUNK_SIZE[1] as i32 {
                let y = y - CHUNK_SIZE[1] as i32 / 2;
                let x = x - CHUNK_SIZE[0] as i32 / 2;
                let tile = Tile {
                    point: (
                        pos.0 + x * MAP_SIZE[0] as i32,
                        pos.1 + y * MAP_SIZE[1] as i32,
                    ),
                    sprite_index: field.sprite_index(),
                    ..Default::default()
                };

                tiles.push(tile.clone());
            }
        }
        let tile = Tile {
            point: pos.clone(),
            sprite_index: if pos.0 == 0 && pos.1 == 0 {
                MapField::Player.sprite_index()
            } else {
                field.sprite_index()
            },
            ..Default::default()
        };
        mini_tiles.push(tile.clone());
        map.fields.insert(pos.clone(), field.clone());
        if matches!(field, MapField::Water) {
            map.collisions.insert(pos.clone());
        }
        match field {
            MapField::Town {
                item: _,
                visited: _,
            } => {
                map.blinks.insert(pos.clone());
            }
            MapField::Castle => {
                map.blinks.insert(pos.clone());
            }
            _ => {}
        }
    }
    map.width = width as u32;
    map.height = height as u32;
    map.tile_size = MAP_TEXTURE_SIZE[0] as f32; // 正方形を仮定

    if let Some((mut tilemap, _without)) = tilemap_query.q0_mut().iter_mut().next() {
        // TileMapにTileを追加
        tilemap.insert_tiles(tiles).unwrap();

        // ワールドに追加
        tilemap.spawn_chunk((-1, 0)).unwrap();
        tilemap.spawn_chunk((0, 0)).unwrap();
        tilemap.spawn_chunk((1, 0)).unwrap();
        tilemap.spawn_chunk((-1, 1)).unwrap();
        tilemap.spawn_chunk((0, 1)).unwrap();
        tilemap.spawn_chunk((1, 1)).unwrap();
        tilemap.spawn_chunk((-1, -1)).unwrap();
        tilemap.spawn_chunk((0, -1)).unwrap();
        tilemap.spawn_chunk((1, -1)).unwrap();
    }

    if let Some((mut tilemap, _with)) = tilemap_query.q1_mut().iter_mut().next() {
        // TileMapにTileを追加
        tilemap.insert_tiles(mini_tiles).unwrap();

        // ワールドに追加
        tilemap.spawn_chunk((0, 0)).unwrap();
    }
}

fn rng_multi_range(range1: (f32, f32), range2: (f32, f32)) -> f32 {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        rng.gen_range(range1.0, range1.1)
    } else {
        rng.gen_range(range2.0, range2.1)
    }
}
