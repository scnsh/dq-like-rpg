use crate::components::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

pub fn spawn_map_entity(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>, // スプライト全体のハンドルとロード状態を管理
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, // テクスチャアトラス
    // mut textures: ResMut<Assets<Texture>>,
){
    // テクスチャは1つと仮定
    let sprite_handle = sprite_handles.handles[0].clone().typed::<Texture>();
    // let atlas_texture = textures.get_mut(sprite_handle.clone()).unwrap();

    let texture_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(16., 16.), 6, 1);
    let atlas_handle = texture_atlases.add(texture_atlas);

    // タイルマップの構成を決定
    let tilemap = Tilemap::builder()
        // .auto_chunk() // spawnする際に新しいchunkとして生成する
        .topology(GridTopology::Square) // tilemap の構成
        .dimensions(1, 1) // tilemap の数
        .chunk_dimensions(48, 32, 1) // chunk_mapの数
        .texture_dimensions(16, 16) // タイルのサイズ(px)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            2,
        )
        // .z_layers(3) // レイヤー数
        .texture_atlas(atlas_handle)
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

    // tilemapをワールドに追加
    commands
        .spawn_bundle(tilemap_components)
        .insert(GameState::MapView);
}