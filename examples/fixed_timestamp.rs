use bevy::{
    core::{FixedTimestep, FixedTimesteps},
    prelude::*,
};

const LABEL: &str = "my_fixed_timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

// ゲームの状態
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Title,
    Main,
}
impl Default for GameState {
    fn default() -> Self {
        GameState::Title
    }
}

fn main() {
    App::build()
        .init_resource::<GameState>()
        .add_state(GameState::default())
        .add_plugins(DefaultPlugins)
        // this system will run once every update (it should match your screen's refresh rate)
        .add_system_set(
            SystemSet::on_update(GameState::Title)
                .with_system(update.system())
        )
        // .add_system(update.system())
        // add a new stage that runs every two seconds
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(2.0)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label(LABEL),
                )
                .with_system(fixed_update.system()),
        )
        .run();
}

fn update(mut last_time: Local<f64>, time: Res<Time>) {
    println!("update: {}", time.seconds_since_startup() - *last_time);
    *last_time = time.seconds_since_startup();
}

fn fixed_update(mut last_time: Local<f64>, time: Res<Time>, fixed_timesteps: Res<FixedTimesteps>) {
    println!(
        "fixed_update: {}",
        time.seconds_since_startup() - *last_time,
    );

    let fixed_timestep = fixed_timesteps.get(LABEL).unwrap();
    println!(
        "  overstep_percentage: {}",
        fixed_timestep.overstep_percentage()
    );

    *last_time = time.seconds_since_startup();
}