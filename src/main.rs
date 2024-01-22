use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

mod camera;
mod physics;
mod player;
mod terrain;

fn main() {
    let mut rules: HashMap<u16, HashSet<u16>> = HashMap::new();
    let neighbors: Vec<u16> = vec![1, 1, 1, 2, 3, 3];

    rules.insert(1, vec![1, 2].into_iter().collect());
    rules.insert(2, vec![1, 2, 3].into_iter().collect());
    rules.insert(3, vec![3, 2].into_iter().collect());

    let res = terrain::wave_function_collapse::collapse_cell(&rules, &neighbors);
    println!("{:?}", res);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb_u8(100, 140, 70)))
        .add_plugins((
            camera::CameraPlugin,
            physics::movement::MovementPlugin,
            physics::collision::CollisionPlugin,
            player::PlayerPlugin,
        ))
        .run();
}
