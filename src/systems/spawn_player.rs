use crate::components::*;
use crate::resources::*;
use bevy::{prelude::*, render::camera::RenderLayers};

pub fn spawn_player(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    map: Res<Map>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut camera_query: Query<(Entity, &mut Transform, &mut Position, &mut MapCamera)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (camera, mut transform, mut position, mut map_camera) in camera_query.iter_mut() {
        // カメラ位置をリセットする(GameOver後のリスタートも想定する)
        *transform =
            position_to_translation(&map, &Position { x: 0., y: 0. }, transform.translation.z);
        *position = Position { x: 0., y: 0. };
        *map_camera = MapCamera::default();

        // 主人公を追加する
        // let you_sprite = asset_server.load("textures/player/player.png");
        let you_sprite = asset_handles.player.clone();
        let texture_atlas = TextureAtlas::from_grid(you_sprite, Vec2::new(14., 20.), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // let position = Position { x: 0, y: 0 };
        // let transform = position_to_translation(&map, &position, render_layer(RenderLayer::Player) as f32);
        let player = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(
                    0.,
                    0.,
                    -transform.translation.z + render_layer(RenderLayer::Player) as f32,
                ),
                ..Default::default()
            })
            .insert(RenderLayers::layer(0))
            .insert(Player {
                battle_state: PlayerBattleState::Select,
            })
            .insert(CharacterStatus::default())
            .insert(Inventory::default())
            // .insert(position)
            .insert(Timer::from_seconds(0.5, true))
            .id();
        commands.entity(camera).push_children(&[player]);
    }

    // 次の画面に遷移する
    game_state.set(GameState::Map).unwrap();
}
