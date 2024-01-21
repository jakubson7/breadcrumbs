use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    physics::{
        collision::{Collider, CollisionDetector, StaticBody},
        movement::{LinearForceFactor, LinearFriction, LinearMaxVelocity, LinearVelocity},
    },
};

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Missile;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player).add_systems(
            Update,
            (
                control_player_movement,
                control_player_rotation,
                show_player_collisions,
            ),
        );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::ORANGE_RED,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        LinearVelocity::ZERO,
        LinearMaxVelocity::new(500.0),
        LinearForceFactor::splat(Vec2::new(5.0, 15.0)),
        LinearFriction::new(Vec2::splat(500.0)),
        Collider::square(50.0),
        CollisionDetector::new(500.0),
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::BLUE,
                ..default()
            },
            ..default()
        },
        Collider::square(50.0),
        StaticBody,
    ));
}

fn control_player_movement(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    keyborad_input: Res<Input<KeyCode>>,
) {
    let mut velocity = query.single_mut();

    let mut direction = Vec2::ZERO;

    if keyborad_input.pressed(KeyCode::Up) || keyborad_input.pressed(KeyCode::W) {
        direction += Vec2::Y;
    }
    if keyborad_input.pressed(KeyCode::Down) || keyborad_input.pressed(KeyCode::S) {
        direction += Vec2::NEG_Y;
    }
    if keyborad_input.pressed(KeyCode::Right) || keyborad_input.pressed(KeyCode::D) {
        direction += Vec2::X;
    }
    if keyborad_input.pressed(KeyCode::Left) || keyborad_input.pressed(KeyCode::A) {
        direction += Vec2::NEG_X;
    }

    velocity.apply_force(direction.normalize_or_zero() * 300.0);
}

fn control_player_rotation(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut player_transform = player_query.single_mut();
    let window = window_query.single();
    let (camera, camera_global_transform) = camera_query.single();

    let player_viewport_position = camera
        .world_to_viewport(camera_global_transform, player_transform.translation)
        .unwrap_or_default()
        .extend(0.0);
    let relative_cursor_position =
        window.cursor_position().unwrap_or(Vec2::ZERO) - player_viewport_position.truncate();
    let angle = relative_cursor_position.angle_between(Vec2::X);

    player_transform.rotation = Quat::from_rotation_z(angle);
}
fn show_player_collisions(query: Query<&Collider, With<Player>>) {
    let collider = query.single();

    if collider.collisions.len() != 0 {
        //info!("{:?}", collider.collisions);
    }
}
