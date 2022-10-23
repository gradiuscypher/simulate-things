use bevy::prelude::*;

#[derive(Component)]
pub struct Leaf {
    pub energy: f32,
}

#[derive(Bundle)]
pub struct Tree {
    pub leaf: Leaf,
}
