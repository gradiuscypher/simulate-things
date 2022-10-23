use bevy::prelude::*;
use bevy::time::FixedTimestep;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Default, Debug)]
struct FrameCount {
    value: u32,
}

fn main() {
    let mut update = SystemStage::parallel();
    update.add_system(count_frames);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(startup)
        .add_stage(
            FixedUpdateStage,
            update.with_run_criteria(FixedTimestep::step(1.0)),
        )
        .insert_resource(FrameCount { value: 0 })
        .add_system(increment_count)
        .run();
}

fn startup() {
    println!("Starting up ...");
}

fn count_frames(frame_count: Res<FrameCount>) {
    println!("frames: {:?}", frame_count.value);
}

fn increment_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.value += 1;
}
