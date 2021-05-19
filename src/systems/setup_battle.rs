use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use crate::events::*;
use crate::components::{BattleCamera, Position, render_layer, RenderLayer, AssetHandles, Player, position_to_translation, UiBattle};
use crate::resources::{Battle};
use core::cmp;
use bevy::render::renderer::RenderResource;

pub fn setup_battle(
    // mut battle_events: EventReader<GameEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut battle: ResMut<Battle>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Player, &Transform)>,
    mut windows: ResMut<Windows>,
    mut ui_battle_query: Query<(Entity, &mut Visible), (With<UiBattle>)>
){
    println!("called");
    // 参考
    // https://github.com/StarArawn/bevy_roguelike_prototype/blob/main/src/game/gameplay/scenes/battle.rs

    // プレイヤーの現在位置を取得
    let (_player, player_transform) = player_query.single().unwrap();
    let enemy = asset_handles.enemy_0.clone();
    let background = asset_handles.battle_background.clone();

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
        .with_children(|child_builder| {
            // let mut battle_camera = OrthographicCameraBundle::new_2d();
            // battle_camera.orthographic_projection.scale = 0.3;
            // child_builder.spawn_bundle(battle_camera)
            //     .insert(BattleCamera)
            //     .insert(RenderLayers::layer(1));
            /// 背景を追加
            child_builder.spawn_bundle(SpriteBundle{
                sprite: Sprite::new(Vec2::new(window.height(), window.width())),
                material: materials.add(background.into()),
                transform: Transform{
                    translation: Vec3::new(0.,
                                           0.,
                                           render_layer(RenderLayer::BattleBackGround) as f32),
                    ..Default::default()
                },
                ..Default::default()
            });
            /// 敵を追加
            child_builder.spawn_bundle(SpriteBundle {
                material: materials.add(enemy.clone().into()),
                transform: Transform{
                    translation: Vec3::new(enemy_root_offset.x,
                                           enemy_root_offset.y,
                                           render_layer(RenderLayer::BattleForeGround) as f32),
                    scale: Vec3::new(enemy_scale,
                                     enemy_scale,
                                     1.),
                    ..Default::default()
                },
                ..Default::default()
            });
        }).id();

    // battle用のコンポーネントを保持
    battle.entity = Some(battle_entity);

    // 戦闘用のUIを表示するように変更
    for (_entity, mut visible) in ui_battle_query.iter_mut() {
        visible.is_visible = true;
    }
        // }
    // }
}

pub fn cleanup_battle_view(
    mut commands: Commands,
    battle: Res<Battle>,
    mut ui_battle_query: Query<(Entity, &mut Visible), (With<UiBattle>)>
){
    commands.entity(battle.entity.unwrap()).despawn_recursive();
    for (_entity, mut visible) in ui_battle_query.iter_mut() {
        visible.is_visible = false;
    }
}