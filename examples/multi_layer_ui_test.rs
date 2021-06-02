use bevy::prelude::*;
use bevy::render::RenderStage::Render;
use bevy::render::camera::{RenderLayers, ActiveCameras, Camera};
use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

/// An implementation of the classic game "Breakout"
const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(setup_cameras.system())
        .add_system(setup_ui_layer1.system())
        .add_system(setup_ui_layer2.system())
        .add_system(keyboard_input_system.system())
        .add_system(text_update_system.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

pub fn setup_cameras(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    // 2D用カメラを追加する
    let mut map_camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(map_camera);

    // UI用カメラを追加する
    commands.spawn_bundle(UiCameraBundle::default())
        .insert(RenderLayers::layer(2))
        .insert(UICamera);

    // // UI用カメラを追加する
    // commands.spawn_bundle(UiCameraBundle::default())
    //     .insert(RenderLayers::layer(1));
}

struct UICamera;
struct UIText;

pub fn setup_ui_layer1(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!\nlayer1",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(UIText)
        .insert(RenderLayers::layer(1));
}

pub fn setup_ui_layer2(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!\nlayer2",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(UIText)
        .insert(RenderLayers::layer(1));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RenderLayers, &mut UICamera)>,
) {
    if keyboard_input.pressed(KeyCode::Key2) {
        for (mut layers, _ui_camera) in query.iter_mut() {
            layers.with(2);
            // layers.without(1);
        }
        info!("'2' currently pressed");
    }
    if keyboard_input.pressed(KeyCode::Key1) {
        for (mut layers, _ui_camera) in query.iter_mut() {
            layers.with(1);
            // layers.without(2);
        }
        info!("'1' currently pressed");
    }
}


fn text_update_system(diagnostics: Res<Diagnostics>,
                      mut query: Query<(&mut RenderLayers, &mut UIText)>)
{
    for (mut render_layer, _text) in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                info!("called");
                render_layer.with(2);
            }
        }
    }
}
