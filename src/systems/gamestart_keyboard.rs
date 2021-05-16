use crate::resources::*;

use bevy::prelude::*;

pub fn gamestart_keyboard(
    mut state: ResMut<State<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("pressed space");
        state.set(GameState::Loading).unwrap();
        // TODO:  https://github.com/bevyengine/bevy/issues/1700
        keyboard_input.reset(KeyCode::Space);
    }
}