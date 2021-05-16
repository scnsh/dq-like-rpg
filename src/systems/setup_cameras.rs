use bevy::prelude::*;
use crate::components::*;

pub fn setup_cameras(
    mut commands: Commands)
{
    // 2D用カメラを追加する
    let mut camera = OrthographicCameraBundle::new_2d();
    // 描画範囲を絞る
    camera.orthographic_projection.scale = 0.3;
    commands.spawn_bundle(camera)
        .insert(MapCamera)
        .insert(Position { x: 0, y: 0 });
    // UI用カメラを追加する
    commands.spawn_bundle(UiCameraBundle::default());
}
