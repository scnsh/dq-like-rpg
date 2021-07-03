use crate::components::UiTitleText;
use crate::resources::{ForState, GameState};
use bevy::prelude::*;

pub fn setup_loading_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 親ノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                // 子のノードは画面上の上から下に並べる
                // flex_direction: FlexDirection::ColumnReverse,
                // 子のノードは左右に対して中央にCenteringして並べる
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // NONE = 黒
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Loading],
        })
        .with_children(|parent| {
            // 上部のタイトル
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(25.0)),
                        // ウインドウの外側のマージン
                        margin: Rect::all(Val::Px(20.0)),
                        // Vertical方向の中央揃え
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::BLACK.into()),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Loading],
                })
                .with_children(|parent| {
                    // テキスト
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.)),
                                // Horizontal方向の中央揃え
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Loading",
                                TextStyle {
                                    font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                    font_size: 80.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![GameState::Loading],
                        })
                        .insert(Timer::from_seconds(0.5, true))
                        .insert(UiTitleText);
                });
        });
}

pub fn update_loading_ui(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Text), With<UiTitleText>>,
) {
    for (mut timer, mut text) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if text.sections[0].value.len() > 10 {
                text.sections[0].value = format!("Loading");
            } else {
                text.sections[0].value = format!("{}.", text.sections[0].value);
            }
        }
    }
}
