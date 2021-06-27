use crate::components::*;
use bevy::prelude::*;

pub fn setup_cameras(mut commands: Commands) {
    // 2D用カメラを追加する
    let mut map_camera = OrthographicCameraBundle::new_2d();

    // 描画範囲を絞る
    map_camera.orthographic_projection.scale = 0.3;
    commands
        .spawn_bundle(map_camera)
        .insert(MapCamera::default())
        .insert(Position { x: 0., y: 0. })
        .insert(Timer::from_seconds(0.25, true));
    // MapView は Layer0 に描画する
    // .insert(RenderLayers::layer(0));

    // UI用カメラを追加する
    commands.spawn_bundle(UiCameraBundle::default());
}
