use crate::generate::Map;
use crate::loading::FontAssets;
use crate::AppState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGameMap)
                .with_system(setup_status_ui.system())
                .with_system(setup_map_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGameMap).with_system(update_status_ui.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGameBattle)
                .with_system(setup_status_ui.system())
                .with_system(setup_battle_inventory_ui.system())
                .with_system(setup_enemy_status_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGameBattle)
                .with_system(update_status_ui.system())
                .with_system(update_battle_inventory_ui.system())
                .with_system(update_enemy_status_ui.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGameEvent).with_system(setup_event_ui.system()),
        );
    }
}

// ステータス画面のテキスト
pub struct UiStatusPlayerText;
// 敵画面のテキスト
pub struct UiStatusEnemyText;
// インベントリ画面のテキスト
pub struct UiStatusInventoryText;
// イベント画面のテキスト
pub struct UiEventText;
// タイトル画面のテキスト
pub struct UiTitleText;

// ステータスウインドウを表示
fn setup_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&CharacterStatus, With<Player>>,
) {
    let player_status = player_query.single().unwrap();
    // ステータスウインドウ(常に表示)
    // 左上の位置を absolute に指定
    commands
        .spawn_bundle(NodeBundle {
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
            // ステータスウインドウ(背景)
            parent
                .spawn_bundle(NodeBundle {
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
                    // ステータスウインドウ(中身)
                    parent
                        .spawn_bundle(NodeBundle {
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
                            parent
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{}", player_status),
                                        TextStyle {
                                            font: asset_server
                                                .load("fonts/PixelMplus12-Regular.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        Default::default(), // TextAlignment{
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
        });
}

// ステータス画面(プレイヤー)を更新する
pub fn update_status_ui(
    query: Query<&CharacterStatus, (With<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusPlayerText>>,
) {
    for player_status in query.iter() {
        for mut text in status_query.iter_mut() {
            text.sections[0].value = format!("{}", player_status);
        }
    }
}

pub fn setup_battle_inventory_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    inventory_query: Query<&Inventory, With<Player>>,
) {
    // インベントリUIを表示
    let inventory = inventory_query.single().unwrap();
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
                // 枠線
                border: Rect::all(Val::Px(2.0)),
                // ウインドウの外側のマージン
                margin: Rect::all(Val::Percent(3.0)),
                // 左下に設定
                // align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Battle],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Battle],
                })
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
                            material: materials.add(Color::BLACK.into()),
                            // // 最初は見えない
                            // visible: Visible {
                            //     is_visible: false,
                            //     is_transparent: false,
                            // },
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![GameState::Battle],
                        })
                        .with_children(|parent| {
                            // テキスト
                            parent
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{}", inventory.skill_list()),
                                        TextStyle {
                                            font: font_assets.pixel_mplus.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            horizontal: HorizontalAlign::Left,
                                            ..Default::default()
                                        },
                                    ),
                                    // // 最初は見えない
                                    // visible: Visible {
                                    //     is_visible: false,
                                    //     is_transparent: false,
                                    // },
                                    ..Default::default()
                                })
                                .insert(UiStatusInventoryText)
                                .insert(ForState {
                                    states: vec![GameState::Battle],
                                });
                        });
                });
        });
}

pub fn setup_enemy_status_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
    enemy_data: Res<EnemyData>,
    player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
) {
    // 敵のステータスを表示
    let (_camera, _player_transform, position) = player_camera_query.single().unwrap();
    let field = position_to_field(&map, position);
    let enemy_status = enemy_data.create(
        &field,
        level(player_status.lv, enemy_data.field_to_enemy(&field)),
    );
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(66.), Val::Percent(97.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    ..Default::default()
                },
                border: Rect::all(Val::Px(2.0)),
                // ウインドウの外側のマージン
                margin: Rect::all(Val::Px(10.0)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(ForState {
            states: vec![GameState::Battle],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        padding: Rect::all(Val::Px(10.)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    visible: Visible {
                        is_visible: true,
                        is_transparent: true,
                    },
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![GameState::Battle],
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                enemy_status.enemy_text(),
                                TextStyle {
                                    font: font_assets.pixel_mplus.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![GameState::Battle],
                        })
                        .insert(UiStatusEnemyText);
                });
        });
}

// ステータス画面(エネミー)を更新する
pub fn update_enemy_status_ui(
    query: Query<&CharacterStatus, (Without<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusEnemyText>>,
) {
    for enemy_status in query.iter() {
        for mut text in status_query.iter_mut() {
            text.sections[0].value = enemy_status.enemy_text();
        }
    }
}

// ステータス画面(バトルインベントリ)を更新する
pub fn update_battle_inventory_ui(
    query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut queries: Query<&mut Text, With<UiStatusInventoryText>>,
) {
    for inventory in query.iter() {
        for mut text in queries.iter_mut() {
            text.sections[0].value = format!("{}", inventory.skill_list());
        }
    }
}
