use bevy::prelude::*;
use bevy::time::FixedTimestep;

#[derive(Component)]
struct Leaf {
    energy: f32,
}

#[derive(Bundle)]
struct Tree {
    leaf: Leaf,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    let mut update = SystemStage::parallel();
    update.add_system(count_trees);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(startup)
        .add_stage(
            FixedUpdateStage,
            update.with_run_criteria(FixedTimestep::step(1.0)),
        )
        .run();
}

fn startup(mut commands: Commands) {
    println!("Starting up ...");
    let tree = commands
        .spawn_bundle(Tree {
            leaf: Leaf { energy: 50.0 },
        })
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(1.0, 1.0, 0.0),
            ..Default::default()
        });
    let tree = commands
        .spawn_bundle(Tree {
            leaf: Leaf { energy: 33.0 },
        })
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(2.0, 2.0, 0.0),
            ..Default::default()
        });
}

fn count_trees(query: Query<(&Leaf, &Transform)>) {
    for (leaf, transform) in query.iter() {
        println!(
            "leaf at {} has {} energy",
            transform.translation, leaf.energy,
        );
    }
}
