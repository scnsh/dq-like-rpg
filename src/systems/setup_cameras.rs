use bevy::prelude::*;

pub fn setup_cameras(
    mut commands: Commands)
{
    // 2D用カメラを追加する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // // UI用カメラを追加する
    // commands.spawn_bundle(UiCameraBundle::default());
}
