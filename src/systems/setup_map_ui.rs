use crate::components::*;
use crate::resources::{ForState, GameState};
use bevy::prelude::*;

pub fn setup_map_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory_query: Query<&Inventory, With<Player>>,
    mut audio_event_writer: EventWriter<AudioEvent>,
) {
    let inventory = inventory_query.single().unwrap();
    // インベントリウインドウ
    // 左下位置を absolute に指定
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(25.), Val::Percent(60.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(2.),
                    bottom: Val::Percent(2.),
                    ..Default::default()
                },
                // justify_content: JustifyContent::SpaceBetween,
                // 枠線はなし
                // border: Rect::all(Val::Px(2.0)),
                // ウインドウの外側のマージン
                margin: Rect::all(Val::Percent(3.0)),
                // 左下に設定
                // align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            // material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Map],
        })
        // .insert(UiMap)
        .with_children(|parent| {
            // 左上のウインドウ(中身)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        // padding: Rect::all(Val::Px(10.)),
                        // align_items: AlignItems::FlexEnd,
                        // justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    visible: Visible {
                        is_visible: false,
                        is_transparent: true,
                    },
                    // material: materials.add(Color::BLACK.into()),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Map],
                })
                // .insert(UiMap)
                .with_children(|parent| {
                    // 左上のウインドウ(中身)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                padding: Rect::all(Val::Px(10.)),
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            visible: Visible {
                                is_visible: false,
                                is_transparent: true,
                            },
                            // material: materials.add(Color::BLACK.into()),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![GameState::Map],
                        })
                        // .insert(UiMap)
                        .with_children(|parent| {
                            // テキスト
                            parent
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{}", inventory),
                                        TextStyle {
                                            font: asset_server
                                                .load("fonts/PixelMplus12-Regular.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            horizontal: HorizontalAlign::Left,
                                            ..Default::default()
                                        },
                                    ),
                                    // visible: Visible {
                                    //     is_visible: false,
                                    //     is_transparent: true,
                                    // },
                                    ..Default::default()
                                })
                                .insert(UiStatusInventoryText)
                                .insert(ForState {
                                    states: vec![GameState::Map],
                                });
                        });
                });
        })
        .id();

    audio_event_writer.send(AudioEvent::Play(AudioKind::BGMMap));
}
