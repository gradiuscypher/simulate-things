use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Leaf {
    pub energy: f32,
}

#[derive(Bundle)]
pub struct Tree {
    pub leaf: Leaf,
}

pub fn spawn_plants(mut commands: Commands) {
    println!("Starting plant spawn...");
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let x: f32 = rng.gen_range(0..100) as f32;
        let y: f32 = rng.gen_range(0..100) as f32;
        let energy: f32 = rng.gen_range(0..100) as f32;

        commands
            .spawn_bundle(Tree {
                leaf: Leaf { energy },
            })
            .insert_bundle(TransformBundle {
                local: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            });
    }
    println!("Plant spawn complete!");
}

pub fn count_trees(query: Query<(&Leaf, &Transform)>) {
    for (leaf, transform) in query.iter() {
        println!(
            "leaf at {} has {} energy",
            transform.translation, leaf.energy,
        );
    }
    println!("-----------------------\n")
}
