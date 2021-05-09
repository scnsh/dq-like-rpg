use crate::resources::*;

use bevy::prelude::*;

pub fn gamestart_keyboard(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Loading).unwrap();
    }
}