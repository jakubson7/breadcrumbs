use bevy::prelude::*;

use super::movement::LinearVelocity;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                (flush_collider, flush_collision_detector),
                update_vertices,
                detect_collision_target,
                detect_collision,
                handle_static_body_collisions,
            )
                .chain(),
        );
    }
}

#[derive(Debug)]
pub struct Collision {
    pub entity: Entity,
    pub offset: Vec2,
}

impl Collision {
    pub fn new(entity: Entity, offset: Vec2) -> Self {
        Self { entity, offset }
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub size: (Vec2, Vec2),
    pub collisions: Vec<Collision>,

    // Vertices in this tuple are the top-right and bottom-left
    vertices: (Vec2, Vec2),
}

impl Collider {
    pub fn new(size: (Vec2, Vec2)) -> Self {
        Self {
            size,
            vertices: size,
            collisions: vec![],
        }
    }
    pub fn square(size: f32) -> Self {
        Self::new((Vec2::splat(size / 2.0), -Vec2::splat(size / 2.0)))
    }
    pub fn collide(&mut self, collision: Collision) {
        self.collisions.push(collision);
    }

    pub fn flush(&mut self) {
        self.collisions.clear();
    }
}

#[derive(Component, Debug)]
pub struct CollisionDetector {
    pub range: f32,
    pub possible_targets: Vec<Entity>,
}

impl CollisionDetector {
    pub fn new(range: f32) -> Self {
        Self {
            range,
            possible_targets: vec![],
        }
    }
    pub fn add_possible_target(&mut self, target_entity: Entity) {
        self.possible_targets.push(target_entity);
    }
    pub fn flush(&mut self) {
        self.possible_targets.clear();
    }
}

#[derive(Component)]
pub struct StaticBody;

fn calc_collision(a: (Vec2, Vec2), b: (Vec2, Vec2)) -> Option<Vec2> {
    if (a.0.x > b.1.x) && (a.1.x < b.0.x) && (a.0.y > b.1.y) && (a.1.y < b.0.y) {
        let delta_x = if (a.0.x - b.1.x) < (b.0.x - a.1.x) {
            b.1.x - a.0.x
        } else {
            b.0.x - a.1.x
        };
        let delta_y = if (a.0.y - a.1.y) < (b.0.y - a.1.y) {
            b.1.y - a.0.y
        } else {
            b.0.y - a.1.y
        };
        return Some(Vec2::new(delta_x, delta_y));
    }

    return None;
}
fn update_vertices(mut query: Query<(&mut Collider, &GlobalTransform)>) {
    for (mut collider, global_transform) in query.iter_mut() {
        let translation = global_transform.translation();

        collider.vertices.0.x = translation.x + collider.size.0.x;
        collider.vertices.0.y = translation.y + collider.size.0.y;
        collider.vertices.1.x = translation.x + collider.size.1.x;
        collider.vertices.1.y = translation.y + collider.size.1.y;
    }
}

// Important Note
// This implementation is still pretty naive
// Yet I have to implement spacial-caching
// This this pretty much sucks, but i guess it works or something

fn detect_collision_target(
    mut detector_query: Query<(&mut CollisionDetector, &GlobalTransform), With<CollisionDetector>>,
    target_query: Query<(&GlobalTransform, Entity), (With<Collider>, Without<CollisionDetector>)>,
) {
    for (mut detector, detector_global_transform) in detector_query.iter_mut() {
        for (target_global_transform, target_entity) in target_query.iter() {
            let distance = detector_global_transform
                .translation()
                .distance(target_global_transform.translation());

            if distance < detector.range {
                detector.add_possible_target(target_entity);
            }
        }
    }
}

fn detect_collision(
    mut detector_query: Query<(&mut Collider, &CollisionDetector, Entity), With<CollisionDetector>>,
    mut target_query: Query<&mut Collider, Without<CollisionDetector>>,
) {
    for (mut detector_collider, detector, detector_entity) in detector_query.iter_mut() {
        for &target_entity in detector.possible_targets.iter() {
            let mut target_collider = target_query.get_mut(target_entity).unwrap();

            if let Some(offset) =
                calc_collision(detector_collider.vertices, target_collider.vertices)
            {
                detector_collider.collide(Collision::new(target_entity, offset));
                target_collider.collide(Collision::new(detector_entity, offset));
            }
        }
    }
}

fn flush_collider(mut query: Query<&mut Collider>) {
    for mut collider in query.iter_mut() {
        collider.flush();
    }
}

fn flush_collision_detector(mut query: Query<&mut CollisionDetector>) {
    for mut detector in query.iter_mut() {
        detector.flush();
    }
}

fn handle_static_body_collisions(
    mut transform_query: Query<
        (&mut Transform, &mut LinearVelocity),
        (With<Collider>, Without<StaticBody>),
    >,
    static_body_query: Query<&Collider, With<StaticBody>>,
) {
    for collider in static_body_query.iter() {
        for collision in collider.collisions.iter() {
            if let Ok((mut transform, mut linear_velocity)) =
                transform_query.get_mut(collision.entity)
            {
                if collision.offset.x.abs() < collision.offset.y.abs() {
                    transform.translation.x += collision.offset.x;
                } else {
                    transform.translation.y += collision.offset.y;
                }

                linear_velocity.flush();
                linear_velocity.set(Vec2::ZERO);
            }
        }
    }
}
