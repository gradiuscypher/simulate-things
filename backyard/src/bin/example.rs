use bevy::prelude::*;
use bevy::time::FixedTimestep;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FastUpdateStage;

fn main() {
    // // Create a Stage to add to our Schedule. Each Stage in a schedule runs all of its systems
    // // before moving on to the next Stage
    let mut update = SystemStage::parallel();

    update.add_system(hello_world.label(SimulationSystem::Start));
    update.add_system(
        run_world
            .after(SimulationSystem::Start)
            .label(SimulationSystem::Run),
    );
    update.add_system(
        end_world
            .after(SimulationSystem::Run)
            .label(SimulationSystem::End),
    );
    update.add_system(done.after(SimulationSystem::End));

    let mut fast_update = SystemStage::parallel();
    fast_update.add_system(fast_background);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(setup)
        .add_stage(
            FixedUpdateStage,
            update.with_run_criteria(FixedTimestep::step(1.0)),
        )
        .add_stage(
            FastUpdateStage,
            fast_update.with_run_criteria(FixedTimestep::step(0.5)),
        )
        .run();
}

fn fast_background() {
    println!("This is a fast run!")
}

fn setup() {
    println!("Starting up ...");
}

fn hello_world() {
    println!("hello world!");
}

fn run_world() {
    println!("running world!");
}

fn end_world() {
    println!("ending world!");
}

fn done() {
    println!("we're done\n");
}
#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
enum SimulationSystem {
    Start,
    Run,
    End,
}
