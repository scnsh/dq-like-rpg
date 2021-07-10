use crate::loading::FontAssets;
use crate::map::TileMap;
use crate::player::Player;
use crate::setup::ForState;
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(update_menu.system()));
    }
}

// タイトル画面のテキスト
pub struct UiTitleText;

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Query<Entity, With<Player>>,
    tilemap: Query<Entity, With<TileMap>>,
) {
    // 再プレイ時のために初期化
    // Playerを削除する
    for entity in player.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Tilemapを削除する
    for entity in tilemap.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // 親ノード
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                // 子のノードは画面上の上から下に並べる
                flex_direction: FlexDirection::ColumnReverse,
                // 子のノードは左右に対して中央にCenteringして並べる
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // NONE = 黒
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppState::Menu],
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
                    states: vec![AppState::Menu],
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
                                "DQ-like RPG",
                                TextStyle {
                                    font: font_assets.pixel_mplus.clone(),
                                    font_size: 80.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![AppState::Menu],
                        });
                });
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
                    states: vec![AppState::Menu],
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
                                "Press 'Space' to start",
                                TextStyle {
                                    font: font_assets.pixel_mplus.clone(),
                                    font_size: 80.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ForState {
                            states: vec![AppState::Menu],
                        })
                        .insert(Timer::from_seconds(1., true))
                        .insert(UiTitleText);
                });
        });
}

pub fn update_menu(time: Res<Time>, mut query: Query<(&mut Timer, &mut Text), With<UiTitleText>>) {
    for (mut timer, mut text) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if text.sections[0].value == "" {
                text.sections[0].value = format!("Press 'Space' to start");
            } else {
                text.sections[0].value = format!("");
            }
        }
    }
}
