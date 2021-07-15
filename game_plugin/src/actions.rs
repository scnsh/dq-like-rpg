#[cfg(debug_assertions)]
use crate::character_status::Skill;
#[cfg(debug_assertions)]
use crate::effects::{skill_to_effect, EffectEvent};
#[cfg(debug_assertions)]
use crate::enemies::EnemyData;
#[cfg(debug_assertions)]
use crate::events::GameEvent;
#[cfg(debug_assertions)]
use crate::inventory::{Inventory, Item};
#[cfg(debug_assertions)]
use crate::map::{Map, Position};
#[cfg(debug_assertions)]
use crate::player::Player;
use crate::player::PlayerMovement;
#[cfg(debug_assertions)]
use crate::setup::MapCamera;
use crate::AppState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerActions>()
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(set_menu_actions.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameExplore)
                    .with_system(set_movement_actions.system())
                    .label("movement")
                    .before(PlayerMovement::Movement),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameBattle)
                    .with_system(set_battle_actions.system())
                    .label("battle"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameEvent)
                    .with_system(set_event_actions.system())
                    .label("event"),
            );
        #[cfg(debug_assertions)]
        {
            app.add_system_set(
                SystemSet::on_update(AppState::InGameExplore)
                    .with_system(explore_debug_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameBattle)
                    .with_system(battle_debug_input.system()),
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Return,
}

#[derive(Default)]
pub struct PlayerActions {
    pub action: Option<Action>,
}
impl PlayerActions {
    pub fn reset_all(&self, keyboard_input: &mut ResMut<Input<KeyCode>>) {
        keyboard_input.reset(KeyCode::W);
        keyboard_input.reset(KeyCode::Up);
        keyboard_input.reset(KeyCode::S);
        keyboard_input.reset(KeyCode::Down);
        keyboard_input.reset(KeyCode::A);
        keyboard_input.reset(KeyCode::Left);
        keyboard_input.reset(KeyCode::D);
        keyboard_input.reset(KeyCode::Right);
        keyboard_input.reset(KeyCode::Return);
    }
}

fn set_menu_actions(
    mut state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(AppState::InGameMap).unwrap();
        // TODO:  https://github.com/bevyengine/bevy/issues/1700
        keyboard_input.reset(KeyCode::Space);
    }
}

fn set_movement_actions(mut actions: ResMut<PlayerActions>, keyboard_input: Res<Input<KeyCode>>) {
    if Action::Up.just_released(&keyboard_input)
        || Action::Up.pressed(&keyboard_input)
        || Action::Left.just_released(&keyboard_input)
        || Action::Left.pressed(&keyboard_input)
        || Action::Down.just_released(&keyboard_input)
        || Action::Down.pressed(&keyboard_input)
        || Action::Right.just_released(&keyboard_input)
        || Action::Right.pressed(&keyboard_input)
    {
        let mut player_movement = None;

        if Action::Up.just_released(&keyboard_input) || Action::Down.just_released(&keyboard_input)
        {
            if Action::Up.pressed(&keyboard_input) {
                player_movement = Option::from(Action::Up);
            } else if Action::Down.pressed(&keyboard_input) {
                player_movement = Option::from(Action::Down);
            } else {
                player_movement = None;
            }
        } else if Action::Up.just_pressed(&keyboard_input) {
            player_movement = Option::from(Action::Up);
        } else if Action::Down.just_pressed(&keyboard_input) {
            player_movement = Option::from(Action::Down);
        } else {
            if let Some(action) = player_movement {
                if matches!(action, Action::Up | Action::Down) {
                    player_movement = None;
                }
            }
        }

        if Action::Right.just_released(&keyboard_input)
            || Action::Left.just_released(&keyboard_input)
        {
            if Action::Right.pressed(&keyboard_input) {
                player_movement = Option::from(Action::Right);
            } else if Action::Left.pressed(&keyboard_input) {
                player_movement = Option::from(Action::Left);
            } else {
                player_movement = None;
            }
        } else if Action::Right.just_pressed(&keyboard_input) {
            player_movement = Option::from(Action::Right);
        } else if Action::Left.just_pressed(&keyboard_input) {
            player_movement = Option::from(Action::Left);
        } else {
            if let Some(action) = player_movement {
                if matches!(action, Action::Right | Action::Left) {
                    player_movement = None;
                }
            }
        }
        // match player_movement {
        //     Some(action) => actions.action = Option::from(action),
        //     _ => {}
        // }
        if let Some(action) = player_movement {
            actions.action = Option::from(action);
        }
    } else {
        actions.action = None;
    }
}

#[cfg(debug_assertions)]
fn explore_debug_input(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut events: EventWriter<GameEvent>,
    enemy_data: Res<EnemyData>,
    map: Res<Map>,
    mut player_camera_query: Query<(&MapCamera, &Transform, &Position)>,
) {
    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::B) {
        if let Some((_map_camera, _transform, position)) = player_camera_query.iter_mut().next() {
            let enemy = enemy_data.field_to_enemy(&map.position_to_field(&position));
            events.send(GameEvent::EnemyEncountered(enemy.clone()));
        }
    }
    if keyboard_input.just_pressed(KeyCode::T) {
        events.send(GameEvent::TownArrived(Item::SpellFire(1), false));
    }
}

fn set_battle_actions(mut actions: ResMut<PlayerActions>, keyboard_input: Res<Input<KeyCode>>) {
    if Action::Up.just_released(&keyboard_input)
        || Action::Up.just_pressed(&keyboard_input)
        || Action::Down.just_released(&keyboard_input)
        || Action::Down.just_pressed(&keyboard_input)
        || Action::Return.just_released(&keyboard_input)
        || Action::Return.just_pressed(&keyboard_input)
    {
        let mut player_command = None;

        if Action::Up.just_released(&keyboard_input) || Action::Down.just_released(&keyboard_input)
        {
            if Action::Up.pressed(&keyboard_input) {
                player_command = Option::from(Action::Up);
            } else if Action::Down.pressed(&keyboard_input) {
                player_command = Option::from(Action::Down);
            } else {
                player_command = None;
            }
        } else if Action::Up.just_pressed(&keyboard_input) {
            player_command = Option::from(Action::Up);
        } else if Action::Down.just_pressed(&keyboard_input) {
            player_command = Option::from(Action::Down);
        } else {
            if let Some(action) = player_command {
                if matches!(action, Action::Up | Action::Down) {
                    player_command = None;
                }
            }
        }

        if Action::Return.just_released(&keyboard_input) {
            if Action::Return.pressed(&keyboard_input) {
                player_command = Option::from(Action::Return);
            } else {
                player_command = None;
            }
        } else if Action::Return.just_pressed(&keyboard_input) {
            player_command = Option::from(Action::Return);
        } else {
            if let Some(action) = player_command {
                if matches!(action, Action::Return) {
                    player_command = None;
                }
            }
        }

        actions.action = player_command;
    } else {
        actions.action = None;
    }
}

#[cfg(debug_assertions)]
fn battle_debug_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut effect_events: EventWriter<EffectEvent>,
    mut player_query: Query<(&mut Inventory, &mut Player)>,
) {
    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::B) {
        state.set(AppState::InGameExplore).unwrap();
        keyboard_input.reset(KeyCode::B);
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        effect_events.send(EffectEvent {
            kind: skill_to_effect(Skill::Wind),
            damage_or_heal: 10,
            is_player_attack: true,
        });
        keyboard_input.reset(KeyCode::E);
    }
    if keyboard_input.just_pressed(KeyCode::I) {
        for (mut inventory, _player) in player_query.iter_mut() {
            inventory.add_item(Item::SpellFire(1));
        }
        keyboard_input.reset(KeyCode::I);
    }
}

fn set_event_actions(mut actions: ResMut<PlayerActions>, keyboard_input: Res<Input<KeyCode>>) {
    if Action::Return.just_released(&keyboard_input) || Action::Return.pressed(&keyboard_input) {
        let mut player_command = None;
        if Action::Return.just_released(&keyboard_input) {
            if Action::Return.pressed(&keyboard_input) {
                player_command = Option::from(Action::Return);
            } else {
                player_command = None;
            }
        } else if Action::Return.just_pressed(&keyboard_input) {
            player_command = Option::from(Action::Return);
        } else {
            if let Some(action) = player_command {
                if matches!(action, Action::Return) {
                    player_command = None;
                }
            }
        }

        if let Some(action) = player_command {
            actions.action = Option::from(action);
        }
    } else {
        actions.action = None;
    }
}

impl Action {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            Action::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            Action::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            Action::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            Action::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
            Action::Return => keyboard_input.just_released(KeyCode::Return),
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            Action::Up => keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up),
            Action::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            Action::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            Action::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            Action::Return => keyboard_input.pressed(KeyCode::Return),
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            Action::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            Action::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            Action::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            Action::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
            Action::Return => keyboard_input.just_pressed(KeyCode::Return),
        }
    }
}
