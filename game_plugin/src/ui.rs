use crate::audio::{AudioEvent, AudioKind};
use crate::character_status::CharacterStatus;
use crate::enemies::EnemyData;
use crate::events::{level, GameEvent, RunState};
use crate::inventory::Inventory;
use crate::loading::FontAssets;
use crate::map::{Map, Position};
use crate::player::Player;
use crate::setup::{ForState, MapCamera};
use crate::AppState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGameExplore)
                .with_system(setup_status_ui.system())
                .with_system(setup_explore_inventory_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGameExplore).with_system(update_status_ui.system()),
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

// ステータスウインドウを表示
fn setup_status_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
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
            states: vec![AppState::InGameExplore, AppState::InGameBattle],
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
                    states: vec![AppState::InGameExplore, AppState::InGameBattle],
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
                            states: vec![AppState::InGameExplore, AppState::InGameBattle],
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
                                            font: font_assets.pixel_mplus.clone(),
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
                                    states: vec![AppState::InGameExplore, AppState::InGameBattle],
                                });
                        });
                });
        });
}

// ステータス画面(プレイヤー)を更新する
fn update_status_ui(
    query: Query<&CharacterStatus, (With<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusPlayerText>>,
) {
    for player_status in query.iter() {
        for mut text in status_query.iter_mut() {
            text.sections[0].value = format!("{}", player_status);
        }
    }
}

fn setup_explore_inventory_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
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
            states: vec![AppState::InGameExplore],
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
                    states: vec![AppState::InGameExplore],
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
                            states: vec![AppState::InGameExplore],
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
                                            font: font_assets.pixel_mplus.clone(),
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
                                    states: vec![AppState::InGameExplore],
                                });
                        });
                });
        })
        .id();

    audio_event_writer.send(AudioEvent::Play(AudioKind::BGMExplore));
}

fn setup_battle_inventory_ui(
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
            states: vec![AppState::InGameBattle],
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
                    states: vec![AppState::InGameBattle],
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
                            states: vec![AppState::InGameBattle],
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
                                    states: vec![AppState::InGameBattle],
                                });
                        });
                });
        });
}

fn setup_enemy_status_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
    enemy_data: Res<EnemyData>,
    player_query: Query<&CharacterStatus, With<Player>>,
    player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
) {
    // 敵のステータスを表示
    let (_camera, _player_transform, position) = player_camera_query.single().unwrap();
    let field = map.position_to_field(position);
    let player_status = player_query.single().unwrap();
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
            states: vec![AppState::InGameBattle],
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
                    states: vec![AppState::InGameBattle],
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
                            states: vec![AppState::InGameBattle],
                        })
                        .insert(UiStatusEnemyText);
                });
        });
}

// ステータス画面(エネミー)を更新する
fn update_enemy_status_ui(
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
fn update_battle_inventory_ui(
    query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut queries: Query<&mut Text, With<UiStatusInventoryText>>,
) {
    for inventory in query.iter() {
        for mut text in queries.iter_mut() {
            text.sections[0].value = format!("{}", inventory.skill_list());
        }
    }
}

fn event_text(state: &RunState) -> String {
    match &state.event {
        None => panic!("can't convert text from None."),
        Some(event) => match event {
            GameEvent::EnemyEncountered(enemy) => {
                format!("Battle!!!\n{0:?} appeared.\n", enemy)
            }
            GameEvent::TownArrived(item, visited) => {
                if *visited {
                    format!("Town\nGet healed up your HP!\n")
                } else {
                    format!("Town\nGet healed up your HP!\nGet a {:?}!", item)
                }
            }
            GameEvent::Win(levelup) => {
                if *levelup {
                    return format!("You Win!\nLevel Up!\n");
                }
                return format!("You Win!\n");
            }
            GameEvent::Lose => {
                format!("You Lose!\n")
            }
            GameEvent::WinLast => {
                format!("You won the last battle!\nYou saved the kingdom!")
            }
        },
    }
}

fn setup_event_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    runstate: Res<RunState>,
    mut audio_event_writer: EventWriter<AudioEvent>,
) {
    commands
        .spawn_bundle(NodeBundle {
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
            states: vec![AppState::InGameEvent],
        })
        .with_children(|parent| {
            // ステータスウインドウ(中身)
            parent
                .spawn_bundle(NodeBundle {
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
                    states: vec![AppState::InGameEvent],
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
                                event_text(&*runstate),
                                TextStyle {
                                    font: font_assets.pixel_mplus.clone(),
                                    font_size: 90.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
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
                            states: vec![AppState::InGameEvent],
                        })
                        .insert(UiEventText);
                });
        })
        .id();

    match runstate.event.as_ref().unwrap() {
        GameEvent::TownArrived(_, _) => {
            audio_event_writer.send(AudioEvent::Play(AudioKind::SETown));
        }
        GameEvent::Win(_) | GameEvent::WinLast => {
            audio_event_writer.send(AudioEvent::Play(AudioKind::BGMWin));
        }
        GameEvent::Lose => {
            audio_event_writer.send(AudioEvent::Play(AudioKind::BGMLose));
        }
        _ => {}
    }
}
