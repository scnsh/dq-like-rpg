use crate::resources::*;
use bevy::prelude::*;

pub fn state_enter_despawn(
    mut commands: Commands,
    state: ResMut<State<GameState>>,
    query: Query<(Entity, &ForState<GameState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.entity(entity).despawn();
        }
    }
}
