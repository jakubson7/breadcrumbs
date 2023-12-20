use bevy::prelude::*;

struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug)]
struct Collider {}
