use bevy::prelude::*;
use bevy::time::FixedTimestep;

#[derive(Component)]
struct Tree {
    energy: f32,
    leaves: f32,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(startup)
        .run();
}

fn startup() {
    println!("Starting up ...");
}
