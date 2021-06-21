use crate::resources::*;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use rand::Rng;
use crate::components::MapField;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

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
        if !tilemap.contains_chunk((0,0)){
            tilemap.insert_chunk((0, 0)).unwrap();
        }

        // chunkの縦・横のサイズを取得
        let chunk_width = (tilemap.width().unwrap() * tilemap.chunk_width()) as i32;
        let chunk_height = (tilemap.height().unwrap() * tilemap.chunk_height()) as i32;

        // ワールド全体をgrassで埋める
        let mut tiles = Vec::new();
        let mut fields = HashMap::with_capacity((chunk_width * chunk_height) as usize);
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                // -chunk_width/2 < x < chunk_width/2,  -chunk_height/2 < y < chunk_height/2
                let pos = (x - chunk_width / 2,
                                    y - chunk_height / 2); // -chunk_height/2 < y < chunk_height/2
                // デフォルトの tile set の z-order は 0
                // // 小さい方が他よりも後ろにレンダリングされ, 0 は 最後尾 で 背景に使うのが最適
                // let tile = Tile {
                //     point: pos, // tileの座標
                //     sprite_index: MapField::Grass as usize, // grassのindex
                //     ..Default::default()
                // };
                // tiles.push(tile);
                fields.insert(pos, MapField::Grass);
            }
        }

        // 山・森・水を配置する
        let mut rng = rand::thread_rng();
        let left = &chunk_width / 2 - 1;
        let right = -&chunk_width / 2;
        let top = &chunk_height / 2 - 1;
        let bottom = -&chunk_height / 2;
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let pos = (x - chunk_width / 2,
                           y - chunk_height / 2); // -chunk_height/2 < y < chunk_height/2
                // 1/60 の確率で山生成開始
                if rng.gen_bool(1./60.) {
                    // 山の散布回数
                    let max = rng.gen_range(10, 60);
                    for i in 0..max {
                        // ランダム移動で山を生成
                        let pos = (
                            (pos.0 + rng.gen_range(-3, 4)).clamp( -&chunk_width / 2, &chunk_width / 2 - 1),
                            (pos.1 + rng.gen_range(-3, 4)).clamp( -&chunk_height / 2, &chunk_height / 2 - 1));
                        fields.insert(pos, MapField::Mountain);
                    }
                }
                // 1/12 で水
                if rng.gen_bool(1./12.) {
                    fields.insert(pos, MapField::Water);
                }
                // 1/6 で森
                if rng.gen_bool(1./6.) {
                    fields.insert(pos, MapField::Forest);
                }

                // 端は水
                if pos.0 == left || pos.0 == right {
                    fields.insert(pos, MapField::Water);
                }
                if pos.1 == top || pos.1 == bottom{
                    fields.insert(pos, MapField::Water);
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
        let castle_x = (chunk_width as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - chunk_width / 2;
        let castle_y = (chunk_height as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - chunk_height / 2;
        fields.insert((castle_x, castle_y), MapField::Castle);

        // 街の生成 開始位置を避けて (0.55~1.0, 0~0.45の比率)に生成
        for item in generate_items() {
            let town_x = (chunk_width as f32 * rng_multi_range((0.05, 0.45), (0.55, 0.95))) as i32 - chunk_width / 2;
            let town_y = (chunk_height as f32 * rng_multi_range((0.05, 0.45), (0.55, 0.95))) as i32 - chunk_height / 2;
            match fields[&(town_x, town_y)] {
                // 街や城と重複した場合は追加しない
                MapField::Town{item, visited } => continue,
                MapField::Castle => continue,
                _ => {}
            }
            fields.insert((town_x, town_y), MapField::Town{item, visited:false});
        }

        // タイルを追加
        map.collisions = HashSet::new();
        map.fields = HashMap::new();
        for (pos, field) in fields.iter_mut() {
            let tile = Tile {
                point: pos.clone(),
                sprite_index: field.sprint_index(),
                ..Default::default()
            };
            tiles.push(tile);
            map.fields.insert(pos.clone(), field.clone());
            if matches!(field, MapField::Water){
                map.collisions.insert(pos.clone());
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

fn rng_multi_range(range1: (f32, f32), range2: (f32, f32)) -> f32 {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5){
        rng.gen_range(range1.0, range1.1)
    }else{
        rng.gen_range(range2.0, range2.1)
    }
}