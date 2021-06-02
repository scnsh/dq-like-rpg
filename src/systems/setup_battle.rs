use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use crate::events::*;
use crate::components::{BattleCamera, Position, render_layer, RenderLayer, AssetHandles, Player, position_to_translation, UiBattle, CharacterStatus, MapField, position_to_field, MapCamera, UiStatusEnemyText, UiMap, UiStatusInventoryText, Inventory};
use crate::resources::{Battle, create_enemy, Enemy, field_to_enemy, Map, ForState, GameState};
use core::cmp;
use bevy::render::renderer::RenderResource;
use bevy_tilemap::Tilemap;

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut battle: ResMut<Battle>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
    mut windows: ResMut<Windows>,
    inventory_query: Query<&Inventory, With<Player>>,
){
    // 参考
    // https://github.com/StarArawn/bevy_roguelike_prototype/blob/main/src/game/gameplay/scenes/battle.rs

    // プレイヤーの現在位置を取得
    let (_player, player_transform, _position) = player_camera_query.single().unwrap();

    if let Some(sprite) = asset_handles.enemies.get(battle.enemy as usize) {
        let enemy_sprite = sprite;
        let background = asset_handles.battle_background.clone();
        let enemy_status = create_enemy(battle.enemy);

        // 敵の表示ウインドウの中心位置オフセットと表示のスケールを求める
        let window = windows.get_primary_mut().unwrap();
        //TODO: orthographic_projection_scale の値の影響をここで補正しないように  --> /.3
        let enemy_window_size = Vec2::new(window.width() as f32 * 2. / 3. / 3.,
                                          window.height() / 3. as f32);
        let enemy_root_offset = Vec2::new(enemy_window_size.x - window.width() as f32 / (2. * 3.), 0.);
        //TODO: 16 をテクスチャから読み込む用に
        let enemy_scale = cmp::min(enemy_window_size.x as i32, enemy_window_size.y as i32) as f32 / 16. * 0.5;

        // 背景と敵を追加
        let battle_entity = commands
            .spawn()
            // プレイヤーの現在位置を基準として表示する
            .insert(Transform::from_translation(Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                0.,
            )))
            .insert(GlobalTransform::default())
            .insert(ForState {
                states: vec![GameState::Battle],
            })
            .with_children(|child_builder| {
                // let mut battle_camera = OrthographicCameraBundle::new_2d();
                // battle_camera.orthographic_projection.scale = 0.3;
                // child_builder.spawn_bundle(battle_camera)
                //     .insert(BattleCamera)
                //     .insert(RenderLayers::layer(1));
                /// 背景を追加
                child_builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(window.height(), window.width())),
                    material: materials.add(background.into()),
                    transform: Transform {
                        translation: Vec3::new(0.,
                                               0.,
                                               render_layer(RenderLayer::BattleBackGround) as f32),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(ForState {
                        states: vec![GameState::Battle],
                    });
                /// 敵を追加
                child_builder.spawn_bundle(SpriteBundle {
                    material: materials.add(enemy_sprite.clone().into()),
                    transform: Transform {
                        translation: Vec3::new(enemy_root_offset.x,
                                               enemy_root_offset.y,
                                               render_layer(RenderLayer::BattleForeGround) as f32),
                        scale: Vec3::new(enemy_scale,
                                         enemy_scale,
                                         1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(enemy_status.clone())
                    .insert(ForState {
                        states: vec![GameState::Battle],
                    });
            }).id();

        // battle用のコンポーネントを保持
        battle.entity = Some(battle_entity);

        // // 戦闘用のUIを表示するように変更
        // for (_entity, mut visible) in ui_queries.q0_mut().iter_mut(){
        //     visible.is_visible = true;
        // }
        // // Map用のUIを非表示するように変更
        // for (_entity, mut visible) in ui_queries.q1_mut().iter_mut(){
        //     visible.is_visible = false;
        // }

        // // バトルウインドウに表示するStatusの初期値を設定
        // for mut text in status_query.iter_mut() {
        //     text.sections[0].value = enemy_status.enemy_text();
        // }

        let inventory = inventory_query.single().unwrap();
        let battle_inventory_window = commands.
            spawn_bundle(NodeBundle {
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
                // // 最初は見えない
                // visible: Visible {
                //     is_visible: false,
                //     is_transparent: false,
                // },
                material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
                ..Default::default()
            })
            .insert(ForState {
                states: vec![GameState::Battle],
            })
            .with_children(|parent| {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        // padding: Rect::all(Val::Px(10.)),
                        // // 枠線
                        // border: Rect::all(Val::Px(2.0)),
                        // // ウインドウの外側のマージン
                        // margin: Rect::all(Val::Percent(5.0)),
                        // 左下に設定
                        // align_self: AlignSelf::FlexStart,
                        ..Default::default()
                    },
                    // material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
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
                        // 左上のウインドウ(中身)
                        parent.spawn_bundle(NodeBundle {
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
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{}", inventory.ui_text()),
                                        TextStyle {
                                            font: asset_server.load("fonts/PixelMplus12-Regular.ttf"),
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
            })
            .id();

        let enemy_window = commands.
            spawn_bundle(NodeBundle {
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
                    // 左下が原点なので、左上に寄せるために設定
                    // flex_direction: FlexDirection::ColumnReverse,
                    // align_self: AlignSelf::FlexEnd,
                    // align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                // material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
                // // 最初は見えない
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
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        padding: Rect::all(Val::Px(10.)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    // material: materials.add(Color::BLACK.into()),
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
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                enemy_status.enemy_text(),
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
                            // visible: Visible {
                            //     is_visible: false,
                            //     is_transparent: false,
                            // },
                            ..Default::default()
                        })
                            .insert(ForState {
                                states: vec![GameState::Battle],
                            })
                            .insert(UiStatusEnemyText);
                    });
            })
            .id();
    }
}

// ステータス画面(エネミー)を更新する
pub fn update_enemy_status_ui(
    query: Query<&CharacterStatus, (Without<Player>, Changed<CharacterStatus>)>,
    mut status_query: Query<&mut Text, With<UiStatusEnemyText>>
){
    for enemy_status in query.iter(){
        for mut text in status_query.iter_mut(){
            text.sections[0].value = enemy_status.enemy_text();
        }
    }
}

// ステータス画面(バトルインベントリ)を更新する
pub fn update_battle_inventory_ui(
    query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut queries: Query<&mut Text, With<UiStatusInventoryText>>,
){
    for inventory in query.iter() {
        for mut text in queries.iter_mut(){
            text.sections[0].value = format!("{}", inventory.ui_text());
        }
    }
}