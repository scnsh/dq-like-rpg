use crate::components::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

pub fn spawn_map_entity(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, // テクスチャアトラス
    camera_query: Query<(Entity, &Transform, &MapCamera)>,
) {
    // テクスチャは1つと仮定
    let sprite_handle = asset_handles.tilemap.clone();

    let texture_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(16., 16.), 6, 1);
    let atlas_handle = texture_atlases.add(texture_atlas);

    // タイルマップの構成を決定
    let tilemap = Tilemap::builder()
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

    commands.spawn_bundle(tilemap_components).insert(TileMap);

    // minimapをワールドに追加
    let sprite_handle = asset_handles.mini_tilemap.clone();
    let texture_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(1., 1.), 8, 1);
    let atlas_handle = texture_atlases.add(texture_atlas);

    let mini_tilemap = Tilemap::builder()
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
        .texture_atlas(atlas_handle)
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
        .id();
    commands.entity(camera).push_children(&[minimap]);
}
