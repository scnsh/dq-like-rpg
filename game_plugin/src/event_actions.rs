use crate::actions::PlayerActions;
use crate::events::{GameEvent, RunState};
use crate::AppState;
use bevy::prelude::*;

pub struct EventActionsPlugin;

// This plugin execute actions from user input on Event scene.
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
        GameEvent::EnemyEncountered(_enemy) => {
            state.set(AppState::InGameBattle).unwrap();
        }
        GameEvent::TownArrived(_, _) => {
            state.set(AppState::InGameExplore).unwrap();
        }
        GameEvent::Win(_levelup) => {
            state.set(AppState::InGameExplore).unwrap();
        }
        // TODO: Return map with experience
        GameEvent::Lose => {
            state.set(AppState::Menu).unwrap();
        }
        // TODO: Save experience after game over
        GameEvent::WinLast => {
            state.set(AppState::Menu).unwrap();
        }
    }
    actions.reset_all(&mut keyboard_input);
}
