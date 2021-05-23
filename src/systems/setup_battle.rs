use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use crate::events::*;
use crate::components::{BattleCamera, Position, render_layer, RenderLayer, AssetHandles, Player, position_to_translation, UiBattle, CharacterStatus, MapField, position_to_field, MapCamera};
use crate::resources::{Battle, create_enemy, Enemy, field_to_enemy, Map};
use core::cmp;
use bevy::render::renderer::RenderResource;
use bevy_tilemap::Tilemap;

pub fn setup_battle(
    // mut battle_events: EventReader<GameEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut battle: ResMut<Battle>,
    asset_handles: Res<AssetHandles>, // スプライト全体のハンドルとロード状態を管理
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
    mut windows: ResMut<Windows>,
    mut ui_battle_query: Query<(Entity, &mut Visible), (With<UiBattle>)>,
    // mut tilemap_query: Query<&mut Tilemap>,
){
    // 参考
    // https://github.com/StarArawn/bevy_roguelike_prototype/blob/main/src/game/gameplay/scenes/battle.rs

    // プレイヤーの現在位置を取得
    let (_player, player_transform, _position) = player_camera_query.single().unwrap();

    // 敵の種類を取得(現在の地形によって変わる
    // let mut enemy = Enemy::Goblin;
    // for mut tilemap in tilemap_query.iter_mut() {
    //     if let Some(row_tile) = tilemap.get_tile((position.x, position.y), 1) {
    //         enemy = match row_tile.index {
    //             1 => Enemy::Elf,
    //             2 => Enemy::Bird,
    //             5 => Enemy::Boss,
    //             _ => panic!()
    //         };
    //     }
    // }
    // let enemy = field_to_enemy(position_to_field(&map, &(position.x, position.y)));

    if let Some(sprite) = asset_handles.enemies.get(battle.enemy as usize) {
        let enemy_sprite = sprite;
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
                    .insert(create_enemy(battle.enemy));
            }).id();

        // battle用のコンポーネントを保持
        battle.entity = Some(battle_entity);

        // 戦闘用のUIを表示するように変更
        for (_entity, mut visible) in ui_battle_query.iter_mut() {
            visible.is_visible = true;
        }
    }
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