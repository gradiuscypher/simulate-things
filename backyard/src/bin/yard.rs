use backyard::components::plants::{count_trees, spawn_plants};
use bevy::prelude::*;
use bevy::time::FixedTimestep;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    let mut update = SystemStage::parallel();
    update.add_system(count_trees);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(spawn_plants)
        .add_stage(
            FixedUpdateStage,
            update.with_run_criteria(FixedTimestep::step(5.0)),
        )
        .run();
}
