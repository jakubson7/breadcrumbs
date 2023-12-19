use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                apply_linear_friction,
                apply_linear_force_scaler,
                apply_linear_velocity,
                apply_linear_max_velocity,
                update_position,
            )
                .chain(),
        );
    }
}

#[derive(Component, Debug)]
pub struct LinearVelocity {
    value: Vec2,
    force: Vec2,
}

impl LinearVelocity {
    pub const ZERO: Self = Self {
        value: Vec2::ZERO,
        force: Vec2::ZERO,
    };
    pub fn new(value: Vec2, force: Vec2) -> Self {
        Self { value, force }
    }
    pub fn set(&mut self, value: Vec2) {
        self.value = value;
    }
    pub fn apply_force(&mut self, force: Vec2) {
        self.force += force;
    }
    pub fn flush(&mut self) {
        self.force = Vec2::ZERO;
    }
}

#[derive(Component, Debug)]
pub struct LinearMaxVelocity {
    value: f32,
}

impl LinearMaxVelocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn set(&mut self, value: f32) {
        self.value = value;
    }
}

#[derive(Component, Debug)]
pub struct LinearForceScaler {
    positive: Vec2,
    negative: Vec2,
}

impl LinearForceScaler {
    pub fn new(positive: Vec2, negative: Vec2) -> Self {
        Self { positive, negative }
    }
    pub fn splat(value: Vec2) -> Self {
        Self::new(value, value)
    }
    pub fn set_positive(&mut self, positive: Vec2) {
        self.positive = positive;
    }
    pub fn set_negative(&mut self, negative: Vec2) {
        self.negative = negative;
    }
}

#[derive(Component, Debug)]
pub struct LinearFriction {
    value: Vec2,
}

impl LinearFriction {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
    pub fn set(&mut self, value: Vec2) {
        self.value = value;
    }
}

fn update_position(mut query: Query<(&mut Transform, &LinearVelocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value.extend(0.0) * time.delta_seconds();
    }
}

fn apply_linear_velocity(mut query: Query<&mut LinearVelocity>, time: Res<Time>) {
    for mut velocity in query.iter_mut() {
        velocity.value = velocity.value + velocity.force * time.delta_seconds();
        velocity.flush();
    }
}

fn apply_linear_max_velocity(mut query: Query<(&mut LinearVelocity, &LinearMaxVelocity)>) {
    for (mut velocity, max_velocity) in query.iter_mut() {
        velocity.value =
            velocity.value.signum() * velocity.value.abs().clamp_length_max(max_velocity.value);
    }
}

fn apply_linear_force_scaler(mut query: Query<(&mut LinearVelocity, &LinearForceScaler)>) {
    for (mut velocity, scaler) in query.iter_mut() {
        if velocity.value.x.signum() == velocity.force.x.signum() {
            velocity.force.x *= scaler.positive.x;
        } else {
            velocity.force.x *= scaler.negative.x;
        }

        if velocity.value.y.signum() == velocity.force.y.signum() {
            velocity.force.y *= scaler.positive.y;
        } else {
            velocity.force.y *= scaler.negative.y;
        }
    }
}

fn apply_linear_friction(mut query: Query<(&mut LinearVelocity, &LinearFriction)>) {
    for (mut velocity, friction) in query.iter_mut() {
        let force = -velocity.value.signum() * velocity.value.abs().min(friction.value);
        velocity.apply_force(force);
    }
}
