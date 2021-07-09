use crate::character_status::CharacterStatus;
use crate::effects::skill_to_effect;
use crate::explore_actions::ExploreAction;
use crate::inventory::Inventory;
use crate::map::{Map, MAP_SIZE};
use crate::player::{Player, PlayerBattleState};
use crate::{AppState, GameState};
use bevy::prelude::*;

pub struct BattleActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for BattleActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BattleActions>().add_system_set(
            SystemSet::on_update(AppState::InGameBattle)
                .with_system(set_battle_actions.system())
                .label("movement")
                .with_system(exec_actions.system())
                .after("movement")
                .with_system(debug_input.system()),
        );
    }
}

#[derive(Clone, Copy)]
pub enum BattleAction {
    Up,
    Down,
    Enter,
    None,
}

#[derive(Default)]
pub struct MapActions {
    pub player_commands: BattleAction,
}

fn set_battle_actions(mut actions: ResMut<BattleActions>, keyboard_input: Res<Input<KeyCode>>) {
    if BattleAction::Up.just_released(&keyboard_input)
        || BattleAction::Up.pressed(&keyboard_input)
        || BattleAction::Down.just_released(&keyboard_input)
        || BattleAction::Down.pressed(&keyboard_input)
        || BattleAction::Enter.just_released(&keyboard_input)
        || BattleAction::Enter.pressed(&keyboard_input)
    {
        let mut player_command = BattleAction::None;

        if BattleAction::Up.just_released(&keyboard_input)
            || BattleAction::Down.just_released(&keyboard_input)
        {
            if BattleAction::Up.pressed(&keyboard_input) {
                player_command = BattleAction::Up;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_command = BattleAction::Down;
            } else {
                player_command = BattleAction::None;
            }
        } else if BattleAction::Up.just_pressed(&keyboard_input) {
            player_command = BattleAction::Up;
        } else if BattleAction::Down.just_pressed(&keyboard_input) {
            player_command = BattleAction::Down;
        } else {
            player_command = BattleAction::None;
        }

        if BattleAction::Enter.just_released(&keyboard_input) {
            if BattleAction::Enter.pressed(&keyboard_input) {
                player_command = BattleAction::Enter;
            } else {
                player_command = BattleAction::None;
            }
        } else if BattleAction::Enter.just_pressed(&keyboard_input) {
            player_command = BattleAction::Enter;
        } else {
            player_command = BattleAction::None;
        }

        if player_command != BattleAction::None {
            actions.player_commands = player_command;
        }
    } else {
        actions.player_commands = BattleAction::None;
    }
}

fn debug_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut effect_spawn_events: EventWriter<EffectSpawnEvent>,
) {
    // デバッグ機能
    if keyboard_input.just_pressed(KeyCode::B) {
        state.set(AppState::InGameExplore).unwrap();
        keyboard_input.reset(KeyCode::B);
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        effect_spawn_events.send(EffectSpawnEvent {
            kind: skill_to_effect(Skill::Wind),
            damage_or_heal: 10,
            is_player_attack: true,
        });
        keyboard_input.reset(KeyCode::E);
    }
    if keyboard_input.just_pressed(KeyCode::I) {
        for (_player_camera, mut inventory, _player, _entity) in player_query.iter_mut() {
            inventory.add_item(Item::SpellFire(1));
        }
        keyboard_input.reset(KeyCode::I);
    }
}

fn exec_actions(
    mut actions: ResMut<BattleActions>,
    mut player_query: Query<(&mut CharacterStatus, &mut Inventory, &mut Player, Entity)>,
    map: Res<Map>,
) {
    if matches!(actions.player_commands, BattleAction::None) {
        return;
    }

    if let Some((mut _character_status, mut inventory, mut player, _entity)) =
        player_query.iter_mut().next()
    {
        match actions.player_commands {
            BattleAction::Up => inventory.decrement_index(),
            BattleAction::Down => inventory.increment_index(),
            BattleAction::Enter => {
                if matches!(player.battle_state, PlayerBattleState::Select) {
                    // state を更新
                    player.battle_state = PlayerBattleState::Attack;
                }
            }
            _ => {}
        }
    }
}

impl BattleAction {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            BattleAction::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            BattleAction::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            BattleAction::Enter => keyboard_input.just_released(KeyCode::Return),
            _ => {}
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            BattleAction::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            BattleAction::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            BattleAction::Enter => keyboard_input.pressed(KeyCode::Return),
            _ => {}
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            BattleAction::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            BattleAction::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            BattleAction::Enter => keyboard_input.just_pressed(KeyCode::Return),
            _ => {}
        }
    }
}
