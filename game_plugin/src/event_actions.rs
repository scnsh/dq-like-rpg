use crate::generate::{Map, MAP_SIZE};
use crate::{AppState, GameState};
use bevy::prelude::*;

pub struct EventActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for EventActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<EventActions>().add_system_set(
            SystemSet::on_update(AppState::InGameEvent)
                .with_system(set_event_actions.system())
                .label("event")
                .with_system(update_events.system())
                .after("event"),
        );
    }
}

#[derive(Clone, Copy)]
pub enum EventAction {
    Return,
    None,
}

#[derive(Default)]
pub struct EventActions {
    pub player_commands: EventAction,
}

fn set_event_actions(mut actions: ResMut<MapActions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Return.just_released(&keyboard_input)
        || GameControl::Return.pressed(&keyboard_input)
    {
        let mut player_commands = EventAction::None;

        if GameControl::Return.just_released(&keyboard_input) {
            if GameControl::Return.pressed(&keyboard_input) {
                player_commands = EventAction::Return;
            } else {
                player_commands = EventAction::None;
            }
        } else {
            player_commands = EventAction::None;
        }

        if player_commands != EventAction::None {
            actions.player_commands = player_commands;
        }
    } else {
        actions.player_commands = EventAction::None;
    }
}

fn update_events(
    mut actions: ResMut<EventActions>,
    runstate: Res<RunState>,
    mut state: ResMut<State<AppState>>,
    mut player_query: Query<(&mut CharacterStatus, &mut Inventory, &mut Player, Entity)>,
) {
    if matches!(map_camera.direction, MapAction::None) {
        return;
    }

    let event = runstate.event.as_ref().unwrap();
    match event {
        //バトル画面に遷移
        GameEvent::EnemyEncountered(_enemy) => {
            state.set(GameState::Battle).unwrap();
        }
        //マップ画面に遷移
        GameEvent::TownArrived(_, _) => {
            state.set(GameState::Map).unwrap();
        }
        //勝ったのでマップ画面に遷移
        GameEvent::Win(_levelup) => {
            state.set(GameState::Map).unwrap();
        }
        //負けたのでタイトルに遷移
        GameEvent::Lose => {
            // Playerを削除する
            for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
            // Tilemapを削除する
            for (entity, _tilemap) in tilemap.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
            state.set(GameState::Title).unwrap();
        }
        // TODO: タイトルに戻って経験値引き継ぎ要素を入れる
        GameEvent::WinLast => {
            // Playerを削除する
            for (_player_camera, _inventory, _player, entity) in player_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
            state.set(GameState::Title).unwrap();
        }
    }
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Return => keyboard_input.just_released(KeyCode::Return),
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Return => keyboard_input.pressed(KeyCode::Return),
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Return => keyboard_input.just_pressed(KeyCode::Return),
        }
    }
}
