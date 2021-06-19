use bevy::prelude::*;
use crate::components::*;
use crate::resources::{GameState, ForState};


pub fn setup_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&CharacterStatus, With<Player>>,
){
    let player_status = player_query.single().unwrap();
    /// ステータスウインドウ(常に表示)
    /// 左上の位置を absolute に指定
    let status_window = commands.
        spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(25.), Val::Percent(25.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(2.),
                    top: Val::Percent(2.),
                    ..Default::default()
                },
                // justify_content: JustifyContent::SpaceBetween,
                // flex_direction: FlexDirection::ColumnReverse,
                // 枠線
                border: Rect::all(Val::Px(2.0)),
                // ウインドウの外側のマージン
                margin: Rect::all(Val::Percent(3.0)),
                // // 左下が原点なので、左上に寄せるために設定
                // align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // material: materials.add(Color::NONE.into()),
            material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Map, GameState::Battle],
        })
            .with_children(|parent| {
                /// ステータスウインドウ(背景)
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        // 枠線
                        // border: Rect::all(Val::Px(2.0)),
                        // // ウインドウの外側のマージン
                        // margin: Rect::all(Val::Percent(5.0)),
                        // // 左下が原点なので、左上に寄せるために設定
                        // align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    // material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
                    ..Default::default()
                })
                    .insert(ForState {
                        states: vec![GameState::Map, GameState::Battle],
                    })
                    .with_children(|parent| {
                        /// ステータスウインドウ(中身)
                        parent.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                padding: Rect::all(Val::Px(10.)),
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            material: materials.add(Color::BLACK.into()),
                            ..Default::default()
                        })
                            .insert(ForState {
                                states: vec![GameState::Map, GameState::Battle],
                            })
                            .with_children(|parent| {
                                // テキスト
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{}", player_status),
                                        TextStyle {
                                            font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        Default::default()
                                        // TextAlignment{
                                        //     vertical: VerticalAlign::Center,
                                        //     horizontal: HorizontalAlign::Center,
                                        // }
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiStatusPlayerText)
                                .insert(ForState {
                                    states: vec![GameState::Map, GameState::Battle],
                                });
                            });
                    });
            })
        .id();
}

// ステータス画面(プレイヤー)を更新する
pub fn update_status_ui(
    query: Query<&CharacterStatus, (With<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusPlayerText>>
){
    for (player_status) in query.iter(){
        for mut text in status_query.iter_mut(){
            text.sections[0].value = format!("{}", player_status);
        }
    }
}

