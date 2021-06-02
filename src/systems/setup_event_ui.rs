use bevy::prelude::*;
use crate::components::{UiEvent, UiEventText};
use crate::events::GameEvent;
use crate::resources::{ForState, GameState, RunState};

pub fn setup_event_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    runstate: Res<RunState>
){

    commands.
        spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(90.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(0.),
                    top: Val::Percent(5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
            // visible: Visible {
            //     is_visible: false,
            //     is_transparent: false,
            // },
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Event],
        })
        .with_children(|parent|{
            /// ステータスウインドウ(中身)
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    padding: Rect::all(Val::Px(10.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                material: materials.add(Color::BLACK.into()),
                // visible: Visible {
                //     is_visible: false,
                //     is_transparent: false,
                // },
                ..Default::default()
            })
            .insert(ForState {
                states: vec![GameState::Event],
            })
            .with_children(|parent|{
                // テキスト
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        runstate.event_text(),
                        TextStyle {
                            font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                            font_size: 90.0,
                            color: Color::WHITE,
                        },
                        TextAlignment{
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    // visible: Visible {
                    //     is_visible: false,
                    //     is_transparent: false,
                    // },
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Event],
                })
                .insert(UiEventText);
            });
        }).id();
}