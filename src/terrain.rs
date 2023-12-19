use bevy::prelude::*;
use rand::Rng;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, generate_terrain);
    }
}

#[derive(Component, Debug)]
pub struct Tile {}

fn generate_terrain(mut commands: Commands) {
    let colors = [
        Color::YELLOW_GREEN,
        Color::YELLOW,
        Color::BEIGE,
        Color::LIME_GREEN,
    ];
    let mut rng = rand::thread_rng();

    for i in -3..=3 {
        for j in -3..=3 {
            let color_index = rng.gen_range(0..=3);

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(200.0)),
                    color: colors[color_index],
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(200.0 * i as f32, 200.0 * j as f32, -10.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}
