use crate::actions::{Action, PlayerActions};
use crate::character_status::CharacterStatus;
use crate::enemies::Battle;
use crate::inventory::Inventory;
use crate::player::{Player, PlayerBattleState};
use crate::AppState;
use bevy::prelude::*;

pub struct BattleActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for BattleActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Battle>().add_system_set(
            SystemSet::on_update(AppState::InGameBattle)
                .with_system(exec_actions.system())
                .after("battle"),
        );
    }
}

fn exec_actions(
    actions: Res<PlayerActions>,
    mut player_query: Query<(&mut CharacterStatus, &mut Inventory, &mut Player, Entity)>,
) {
    if matches!(actions.action, None) {
        return;
    }

    if let Some((mut _character_status, mut inventory, mut player, _entity)) =
        player_query.iter_mut().next()
    {
        match actions.action {
            Some(Action::Up) => inventory.decrement_index(),
            Some(Action::Down) => inventory.increment_index(),
            Some(Action::Return) => {
                if matches!(player.battle_state, PlayerBattleState::Select) {
                    // state を更新
                    player.battle_state = PlayerBattleState::Attack;
                }
            }
            _ => {}
        }
    }
}
