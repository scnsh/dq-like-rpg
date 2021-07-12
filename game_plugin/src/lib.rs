mod actions;
mod audio;
mod battle_actions;
mod character_status;
mod effects;
mod enemies;
mod event_actions;
mod events;
mod explore_actions;
mod inventory;
mod loading;
mod map;
mod menu;
mod player;
mod setup;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::battle_actions::BattleActionsPlugin;
use crate::enemies::EnemiesPlugin;
use crate::explore_actions::ExploreActionsPlugin;
use crate::inventory::InventoryPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

use crate::actions::ActionsPlugin;
use crate::effects::EffectsPlugin;
use crate::event_actions::EventActionsPlugin;
use crate::events::EventsPlugin;
use crate::setup::SetupPlugin;
use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct GamePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    InGameMap,
    InGameExplore,
    InGameBattle,
    InGameEvent,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Loading)
            .add_plugin(SetupPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(ExploreActionsPlugin)
            .add_plugin(BattleActionsPlugin)
            .add_plugin(EventActionsPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(EffectsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
