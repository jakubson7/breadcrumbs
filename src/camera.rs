use bevy::prelude::*;

use crate::{physics::movement::LinearVelocity, player::Player};

const MIN_MARGIN: f32 = 100.0;
const PERCENT_MARGIN: f32 = 0.35;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, control_camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle { ..default() },
        LinearVelocity::ZERO,
        MainCamera,
    ));
}

fn control_camera_movement(
    mut camera_query: Query<
        (&mut LinearVelocity, &Camera, &GlobalTransform),
        (With<MainCamera>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    window: Query<&Window>,
) {
    let (mut velocity, camera, camera_global_transform) = camera_query.single_mut();
    let player_transform = player_query.single();
    let window = window.single();

    let width = window.width();
    let height = window.height();
    let mut force = Vec2::ZERO;
    let player_viewport_position = camera
        .world_to_viewport(camera_global_transform, player_transform.translation)
        .unwrap_or_default()
        .extend(0.0);

    let margin_x = MIN_MARGIN.max(width * PERCENT_MARGIN);
    let margin_y = MIN_MARGIN.max(height * PERCENT_MARGIN);

    if player_viewport_position.x > width - margin_x {
        force.x += player_viewport_position.x - width + margin_x;
    }
    if player_viewport_position.x < margin_x {
        force.x += player_viewport_position.x - margin_x;
    }
    if player_viewport_position.y < margin_y {
        force.y -= player_viewport_position.y - margin_y;
    }
    if player_viewport_position.y > height - margin_y {
        force.y -= player_viewport_position.y - height + margin_y;
    }

    velocity.set(force * 3.0);
}
