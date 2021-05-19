use bevy::prelude::*;
use crate::components::*;


pub fn setup_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let lv = 1;
    let exp = 0;
    let hp_current = 100;
    let hp_max = 100;
    let mp_current = 100;
    let mp_max = 100;
    let attack = 10;
    let defence = 10;

    // 親ノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(UiRoot)
        .with_children(|parent| {
            /// ステータスウインドウ
            /// 左上のウインドウ(枠線)
            parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.), Val::Percent(25.)),
                        // 枠線
                        border: Rect::all(Val::Px(2.0)),
                        // ウインドウの外側のマージン
                        margin: Rect::all(Val::Percent(5.0)),
                        // 左下が原点なので、左上に寄せるために設定
                        align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
                    ..Default::default()
            })
            .with_children(|parent| {
                // 左上のウインドウ(中身)
                parent
                    .spawn_bundle(NodeBundle{
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
                    .with_children(|parent| {
                        // テキスト
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                format!("Lv {0:>2} Exp {1:>3}\n\
                                HP {2:>3} / {3:>3}\n\
                                MP {4:>3} / {5:>3}\n\
                                AT {6:>3} DF {7:>3}\n\
                                ", lv, exp, hp_current, hp_max, mp_current, mp_max, attack, defence),
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
                        .insert(UiStatusText);
                    });
                });
            /// エネミーウインドウ
            /// 右側のウインドウ(枠線)
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(66.), Val::Percent(97.0)),
                    border: Rect::all(Val::Px(2.0)),
                    // ウインドウの外側のマージン
                    margin: Rect::all(Val::Px(10.0)),
                    // // 左下が原点なので、左上に寄せるために設定
                    // flex_direction: FlexDirection::ColumnReverse,
                    // align_self: AlignSelf::FlexEnd,
                    // align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
                // 最初は見えない
                visible: Visible{
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            })
            .insert(UiBattle)
            .with_children(|parent| {
                parent.spawn_bundle(NodeBundle{
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        padding: Rect::all(Val::Px(10.)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    visible: Visible{
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(UiBattle)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Bird Lv 2 HP 100 / 200 AT 50 DF 30 ",
                            TextStyle {
                                font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..Default::default()
                            },
                        ),
                        visible: Visible{
                            is_visible: false,
                            is_transparent: false,
                        },
                        ..Default::default()
                    })
                    .insert(UiBattle);
                });
            });
        });
}

pub fn update_status_ui(
    query: Query<&PlayerStatus, Changed<PlayerStatus>>,
    mut status_query: Query<&mut Text, With<UiStatusText>>
){
    for player_status in query.iter(){
        for mut text in status_query.iter_mut(){
            text.sections[0].value = format!("{}", player_status);
        }
    }
}