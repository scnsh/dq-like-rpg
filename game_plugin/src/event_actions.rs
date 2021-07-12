use crate::actions::PlayerActions;
use crate::audio::{AudioEvent, AudioKind};
use crate::events::{GameEvent, RunState};
use crate::AppState;
use bevy::prelude::*;

pub struct EventActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for EventActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGameEvent)
                .with_system(update_events.system())
                .after("event"),
        );
    }
}

fn update_events(
    actions: Res<PlayerActions>,
    runstate: Res<RunState>,
    mut state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if matches!(actions.action, None) {
        return;
    }

    let event = runstate.event.as_ref().unwrap();
    match event {
        //バトル画面に遷移
        GameEvent::EnemyEncountered(_enemy) => {
            state.set(AppState::InGameBattle).unwrap();
        }
        //マップ画面に遷移
        GameEvent::TownArrived(_, _) => {
            state.set(AppState::InGameExplore).unwrap();
        }
        //勝ったのでマップ画面に遷移
        GameEvent::Win(_levelup) => {
            state.set(AppState::InGameExplore).unwrap();
        }
        //負けたのでタイトルに遷移
        // TODO: 経験値を更新してマップに戻らせる
        GameEvent::Lose => {
            // // Playerを削除する
            // for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
            //     commands.entity(entity).despawn_recursive();
            // }
            // // Tilemapを削除する
            // for (entity, _tilemap) in tilemap.iter_mut() {
            //     commands.entity(entity).despawn_recursive();
            // }
            state.set(AppState::Menu).unwrap();
        }
        // TODO: タイトルに戻って経験値引き継ぎ要素を入れる
        GameEvent::WinLast => {
            // // Playerを削除する
            // for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
            //     commands.entity(entity).despawn_recursive();
            // }
            // // Tilemapを削除する
            // for (entity, _tilemap) in tilemap.iter_mut() {
            //     commands.entity(entity).despawn_recursive();
            // }
            state.set(AppState::Menu).unwrap();
        }
    }
    actions.reset_all(&mut keyboard_input);
}
