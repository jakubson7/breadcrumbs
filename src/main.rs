use bevy::prelude::*;

mod camera;
mod movement;
mod player;
mod terrain;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb_u8(100, 140, 70)))
        .add_plugins((
            camera::CameraPlugin,
            movement::MovementPlugin,
            player::PlayerPlugin,
            terrain::TerrainPlugin,
        ))
        .run();
}
