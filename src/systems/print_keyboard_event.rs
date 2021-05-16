// use crate::resources::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub fn print_keyboard_event(
    // mut state: ResMut<KeyboardState>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
    // if let Some(key_code) = event.key_code {
    //     println!("{0:?}: {1:?}", event.state, key_code);
    // }
}