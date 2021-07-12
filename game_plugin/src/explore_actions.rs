use crate::actions::{Action, PlayerActions};
use crate::audio::{AudioEvent, AudioKind};
use crate::map::{Map, Position, MAP_SIZE};
use crate::setup::{MapCamera, MapCameraState};
use crate::AppState;
use bevy::prelude::*;

pub struct ExploreActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ExploreActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGameExplore)
                .with_system(update_position.system())
                .after("movement"),
        );
    }
}

fn update_position(
    actions: Res<PlayerActions>,
    map: Res<Map>,
    mut player_camera_query: Query<(&mut MapCamera, &mut Transform, &mut Position)>,
) {
    if let Some((mut map_camera, mut transform, mut position)) =
        player_camera_query.iter_mut().next()
    {
        if matches!(actions.action, None) {
            return;
        }
        if matches!(map_camera.state, MapCameraState::Moving) {
            return;
        }

        map_camera.direction = actions.action;

        let mut new_position = map_camera.destination.clone();
        match map_camera.direction {
            Some(Action::Up) => new_position.y += 1.,
            Some(Action::Down) => new_position.y -= 1.,
            Some(Action::Left) => new_position.x -= 1.,
            Some(Action::Right) => new_position.x += 1.,
            _ => {}
        }
        // 障害物に接触していれば移動しない
        if map
            .collisions
            .contains(&(new_position.x as i32, new_position.y as i32))
        {
            return;
        }
        map_camera.destination = new_position;

        // マップ端からワープする場合、次の位置と現在の位置の両方を更新する
        let width = MAP_SIZE[0] as f32;
        let height = MAP_SIZE[1] as f32;
        let left = &width / 2. - 1.;
        let right = -&width / 2.;
        let top = &height / 2. - 1.;
        let bottom = -&height / 2.;
        if new_position.x > left {
            map_camera.destination = Position {
                x: right as f32,
                y: new_position.y,
            };
            *position = Position {
                x: right - 1 as f32,
                y: new_position.y,
            };
            *transform = map.position_to_translation(&position, transform.translation.z);
        }
        if new_position.x < right {
            map_camera.destination = Position {
                x: left as f32,
                y: new_position.y,
            };
            *position = Position {
                x: left + 1 as f32,
                y: new_position.y,
            };
            *transform = map.position_to_translation(&position, transform.translation.z);
        }
        if new_position.y > top {
            map_camera.destination = Position {
                x: new_position.x as f32,
                y: bottom,
            };
            *position = Position {
                x: new_position.x,
                y: bottom - 1 as f32,
            };
            *transform = map.position_to_translation(&position, transform.translation.z);
        }
        if new_position.y < bottom {
            map_camera.destination = Position {
                x: new_position.x as f32,
                y: top,
            };
            *position = Position {
                x: new_position.x,
                y: top + 1 as f32,
            };
            *transform = map.position_to_translation(&position, transform.translation.z);
        }
    }
}
