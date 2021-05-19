use bevy::prelude::*;
use crate::components::*;
use bevy::render::camera::RenderLayers;

pub fn setup_cameras(
    mut commands: Commands)
{
    // 2D用カメラを追加する
    let mut map_camera = OrthographicCameraBundle::new_2d();
    // 描画範囲を絞る
    map_camera.orthographic_projection.scale = 0.3;
    commands.spawn_bundle(map_camera)
        .insert(MapCamera)
        .insert(Position { x: 0, y: 0 });
        // MapView は Layer0 に描画する
        // .insert(RenderLayers::layer(0));

    // UI用カメラを追加する
    commands.spawn_bundle(UiCameraBundle::default());
}
