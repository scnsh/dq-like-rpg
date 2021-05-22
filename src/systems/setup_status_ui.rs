use bevy::prelude::*;
use crate::components::*;
use crate::resources::GameState;


pub fn setup_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<State<GameState>>,
){
    // 親ノード
    commands.spawn_bundle(NodeBundle {
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
            /// 左側ウインドウ
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.), Val::Percent(100.)),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::ColumnReverse,
                    // // 枠線
                    // border: Rect::all(Val::Px(2.0)),
                    // // ウインドウの外側のマージン
                    // margin: Rect::all(Val::Percent(5.0)),
                    // // 左下が原点なので、左上に寄せるために設定
                    // align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
                .with_children(|parent| {
                    /// ステータスウインドウ
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(75.), Val::Percent(25.)),
                            // 枠線
                            border: Rect::all(Val::Px(2.0)),
                            // ウインドウの外側のマージン
                            margin: Rect::all(Val::Percent(5.0)),
                            // // 左下が原点なので、左上に寄せるために設定
                            // align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
                        ..Default::default()
                    })
                        .with_children(|parent| {
                            // 左上のウインドウ(中身)
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
                                .with_children(|parent| {
                                    // テキスト
                                    parent.spawn_bundle(TextBundle {
                                        style: Style {
                                            margin: Rect::all(Val::Px(5.)),
                                            ..Default::default()
                                        },
                                        text: Text::with_section(
                                            "update from update_status_ui()",
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
                })
                .with_children(|parent| {
                    /// バトルインベントリウインドウ
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(75.), Val::Percent(70.)),
                            // 枠線
                            border: Rect::all(Val::Px(2.0)),
                            // ウインドウの外側のマージン
                            margin: Rect::all(Val::Percent(5.0)),
                            // 左下に設定
                            // align_self: AlignSelf::FlexStart,
                            ..Default::default()
                        },
                        material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
                        // 最初は見えない
                        visible: Visible {
                            is_visible: false,
                            is_transparent: false,
                        },
                        ..Default::default()
                    })
                        .insert(UiBattle)
                        .with_children(|parent| {
                            // 左上のウインドウ(中身)
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    padding: Rect::all(Val::Px(10.)),
                                    align_items: AlignItems::FlexEnd,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                material: materials.add(Color::BLACK.into()),
                                // 最初は見えない
                                visible: Visible {
                                    is_visible: false,
                                    is_transparent: false,
                                },
                                ..Default::default()
                            })
                                .insert(UiBattle)
                                .with_children(|parent| {
                                    // テキスト
                                    parent.spawn_bundle(TextBundle {
                                        style: Style {
                                            margin: Rect::all(Val::Px(5.)),
                                            ..Default::default()
                                        },
                                        text: Text::with_section(
                                            format!("> Sword\n\
                                    > Spell :Heal1\n\
                                    > Spell :Fire1\n\
                                    > Spell :Ice1\n"),
                                            TextStyle {
                                                font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
                                                font_size: 30.0,
                                                color: Color::WHITE,
                                            },
                                            Default::default()
                                        ),
                                        // 最初は見えない
                                        visible: Visible {
                                            is_visible: false,
                                            is_transparent: false,
                                        },
                                        ..Default::default()
                                    })
                                        .insert(UiBattle);
                                });
                        });
                });
            /// エネミーウインドウ
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
                visible: Visible {
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            })
                .insert(UiBattle)
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            padding: Rect::all(Val::Px(10.)),
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        visible: Visible {
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
                                visible: Visible {
                                    is_visible: false,
                                    is_transparent: false,
                                },
                                ..Default::default()
                            })
                            .insert(UiBattle)
                            .insert(UiStatusText);
                        });
                });
        });
    // 次の画面に遷移する
    game_state.set(GameState::MapView).unwrap();
}

// ステータス画面(プレイヤー)を更新する
pub fn update_status_ui(
    query: Query<&CharacterStatus, (With<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusText>>
){
    for (player_status) in query.iter(){
        for mut text in status_query.iter_mut(){
            text.sections[0].value = format!("{}", player_status);
        }
    }
}

// ステータス画面(エネミー)を更新する
pub fn update_enemy_status_ui(
    query: Query<&CharacterStatus, (Without<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusText>>
){
    for enemy_status in query.iter(){
        for mut text in status_query.iter_mut(){
            text.sections[0].value = format!("{0} Lv {1:>2} HP {2:>3} / {3:>3} AT {4:>3} DF {5:>3}",
                                            enemy_status.name, enemy_status.lv,
                                             enemy_status.hp_current, enemy_status.hp_max,
                                             enemy_status.attack, enemy_status.defence);
        }
    }
}