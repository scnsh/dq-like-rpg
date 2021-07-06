use crate::loading::TileMapAtlas;
use crate::AppState;
use bevy::prelude::*;
use bevy_tilemap::prelude::{GridTopology, LayerKind, TilemapBundle};
use bevy_tilemap::{Tile, Tilemap, TilemapLayer};
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct GeneratePlugin;

impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let map = Map::generate_map();
        app.insert_resource(map)
            .add_system_set(
                SystemSet::on_enter(AppState::InGameGenerate)
                    .with_system(spawn_map.system())
                    .label("generate")
                    .with_system(spawn_mini_map.system())
                    .label("generate"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameMap).with_system(update_mini_map.system()),
            );
    }
}

// マップフィールドの属性
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Field {
    Grass,
    Forest,
    Mountain,
    Water,
    Town { item: Item, visited: bool },
    Castle,
    Player, // for minimap
    Blink,  // for minimap
}
impl Field {
    pub fn sprite_index(&self) -> usize {
        match &self {
            Field::Grass => 0,
            Field::Forest => 1,
            Field::Mountain => 2,
            Field::Water => 3,
            Field::Town {
                item: _,
                visited: _,
            } => 4,
            Field::Castle => 5,
            Field::Player => 6,
            Field::Blink => 7,
        }
    }
}

pub const MAP_SIZE: [u32; 2] = [64, 48];
pub const MAP_TEXTURE_SIZE: [u32; 2] = [16, 16];
pub const CHUNK_SIZE: [u32; 2] = [3, 3];

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub collisions: HashSet<(i32, i32)>,
    pub blinks_on_mini_tiles: HashSet<(i32, i32)>,
    pub blink_status: bool,
    pub fields: HashMap<(i32, i32), Field>,
    pub tiles: Vec<Tile<P>>,
    pub mini_tiles: Vec<Tile<P>>,
}

impl Map {
    pub fn got_item(&mut self, pos: (i32, i32)) {
        let town = self.fields.get(&pos).unwrap().clone();
        match town {
            Field::Town { item, visited: _ } => {
                self.fields.insert(
                    pos,
                    Field::Town {
                        item,
                        visited: true,
                    },
                );
            }
            _ => panic!("got item should be called on 'Town' Field"),
        }
    }

    pub fn generate_map() -> Self {
        // chunkの縦・横のサイズを取得
        let width = MAP_SIZE[0] as i32;
        let height = MAP_SIZE[1] as i32;
        let mut map = Map {
            width: MAP_SIZE[0],
            height: MAP_SIZE[1],
            tile_size: MAP_TEXTURE_SIZE[0] as f32,
            collisions: HashSet::new(),
            blinks_on_mini_tiles: HashSet::new(),
            blink_status: false,
            fields: HashMap::with_capacity((width * height) as usize),
            tiles: Vec::new(),
            mini_tiles: Vec::new(),
        };

        // ワールド全体をgrassで埋める
        for y in 0..height {
            for x in 0..width {
                // -chunk_width/2 < x < chunk_width/2,  -chunk_height/2 < y < chunk_height/2
                let pos = (x - width / 2, y - height / 2); // -chunk_height/2 < y < chunk_height/2
                                                           // デフォルトの tile set の z-order は 0
                                                           // // 小さい方が他よりも後ろにレンダリングされ, 0 は 最後尾 で 背景に使うのが最適
                map.fields.insert(pos, Field::Grass);
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
                        map.fields.insert(pos, MapField::Mountain);
                    }
                }
                // 1/12 で水
                // マップ端には作らないように(ワープした先が水にならないように)
                if 0 < y && y < height - 1 && 0 < x && x < width - 1 {
                    if rng.gen_bool(1. / 12.) {
                        map.fields.insert(pos, MapField::Water);
                    }
                }
                // 1/6 で森
                if rng.gen_bool(1. / 6.) {
                    map.fields.insert(pos, MapField::Forest);
                }
            }
        }

        // ゲーム開始位置付近は平地にする
        for y in -1..2 {
            for x in -1..2 {
                // let pos = (x, y); // -chunk_height/2 < y < chunk_height/2
                map.fields.insert((x, y), MapField::Grass);
            }
        }

        // 敵城の生成 マップ周縁(0.8 ~ 1.0、0~0.2の比率)に生成
        let castle_x =
            (width as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - width / 2;
        let castle_y =
            (height as f32 * rng_multi_range((0.05, 0.2), (0.8, 0.95))) as i32 - height / 2;
        map.fields.insert((castle_x, castle_y), MapField::Castle);

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
            map.fields.insert(
                (town_x, town_y),
                MapField::Town {
                    item,
                    visited: false,
                },
            );
        }

        for (pos, field) in map.fields.iter_mut() {
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
                    map.tiles.push(tile);
                }
            }
            let mini_tile = Tile {
                point: pos.clone(),
                // 初期位置はプレイヤーの色で塗りつぶす
                sprite_index: if pos.0 == 0 && pos.1 == 0 {
                    MapField::Player.sprite_index()
                } else {
                    field.sprite_index()
                },
                ..Default::default()
            };
            map.mini_tiles.push(mini_tile.clone());
            // 水は通れない設定
            if matches!(field, MapField::Water) {
                map.collisions.insert(pos.clone());
            }
            // 町と城はminimap上で点滅する
            match map.field {
                MapField::Town {
                    item: _,
                    visited: _,
                } => {
                    map.blinks_on_mini_tiles.insert(pos.clone());
                }
                MapField::Castle => {
                    map.blinks_on_mini_tiles.insert(pos.clone());
                }
                _ => {}
            }
        }

        map
    }
}

pub fn spawn_map(
    mut commands: Commands,
    map: Res<Map>,
    texture_atlas: Res<TileMapAtlas>,
    // asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
) {
    // テクスチャは1つと仮定
    // let sprite_handle = asset_handles.tilemap.clone();
    //
    // let texture_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(16., 16.), 6, 1);
    // let atlas_handle = texture_atlases.add(texture_atlas);

    // タイルマップの構成を決定
    let mut tilemap = Tilemap::builder()
        .auto_chunk() // spawnする際に新しいchunkとして生成する
        .topology(GridTopology::Square) // tilemap の構成
        .dimensions(CHUNK_SIZE[0], CHUNK_SIZE[1]) // tilemap の数
        .chunk_dimensions(MAP_SIZE[0], MAP_SIZE[1], 1) // chunk_mapの数
        .texture_dimensions(MAP_TEXTURE_SIZE[0], MAP_TEXTURE_SIZE[1]) // タイルのサイズ(px)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            render_layer(RenderLayer::MapBackGround),
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            render_layer(RenderLayer::MapForeGround),
        )
        .texture_atlas(texture_atlas.tilemap)
        .finish()
        .unwrap();

    // tilemap コンポーネントを含むエンティティを作成
    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands.spawn_bundle(tilemap_components).insert(TileMap);

    // TileMapにTileを追加
    tilemap.insert_tiles(map.tiles).unwrap();

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

pub fn spawn_mini_map(
    mut commands: Commands,
    map: Res<Map>,
    texture_atlas: Res<TileMapAtlas>,
    camera_query: Query<(Entity, &Transform, &MapCamera)>,
) {
    // minimapをワールドに追加
    // let sprite_handle = asset_handles.mini_tilemap.clone();
    // let texture_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(1., 1.), 8, 1);
    // let atlas_handle = texture_atlases.add(texture_atlas);

    let mut mini_tilemap = Tilemap::builder()
        .auto_chunk() // spawnする際に新しいchunkとして生成する
        .topology(GridTopology::Square)
        .dimensions(1, 1)
        .chunk_dimensions(MAP_SIZE[0], MAP_SIZE[1], 1)
        .texture_dimensions(1, 1)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            render_layer(RenderLayer::Player),
        )
        .texture_atlas(texture_atlas.mini_tilemap)
        .finish()
        .unwrap();

    let (camera, transform, _map_camera) = camera_query.single().unwrap();
    // tilemap コンポーネントを含むエンティティを作成
    let mini_tilemap_components = TilemapBundle {
        tilemap: mini_tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        // カメラからの相対位置にminimapを表示する
        transform: Transform::from_xyz(
            MAP_SIZE[0] as f32 * 1.5,
            MAP_SIZE[1] as f32 * 1.5,
            -transform.translation.z + render_layer(RenderLayer::Player) as f32,
        ),
        global_transform: Default::default(),
    };

    let minimap = commands
        .spawn_bundle(mini_tilemap_components)
        .insert(TileMap)
        .insert(MiniMap)
        .insert(Timer::from_seconds(1.0, true))
        .id();
    // 相対位置にするためにカメラの子エンティティとする
    commands.entity(camera).push_children(&[minimap]);

    // TileMapにTileを追加
    mini_tilemap.insert_tiles(map.mini_tiles).unwrap();
    // ワールドに追加
    mini_tilemap.spawn_chunk((0, 0)).unwrap();
}

pub fn update_minimap(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut Tilemap), With<MiniMap>>,
    map: ResMut<Map>,
) {
    for (mut timer, mut tilemap) in query.iter_mut() {
        // 時間を進ませる
        timer.tick(time.delta());
        // 時間が経過すれば、アトラスから次のIndexを設定する
        if timer.finished() {
            let mut map_field = MapField::Blink;
            if map.blink_status {
                map_field = position_to_field(
                    &map,
                    &Position {
                        x: blink.0 as f32,
                        y: blink.1 as f32,
                    },
                );
            }
            map.blink_status = !map.blink_status;
            for blink in &map.blinks {
                tilemap
                    .insert_tile(Tile {
                        point: (blink.0, blink.1),
                        sprite_index: map_field.sprite_index(),
                        ..Default::default()
                    })
                    .unwrap();
            }
        }
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
