use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                (apply_linear_friction, apply_angular_friction),
                (apply_linear_force_factor, apply_angular_force_factor),
                (apply_linear_velocity, apply_angular_velocity),
                (apply_linear_max_velocity, apply_angular_max_velocity),
                (update_position, update_rotation),
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
pub struct LinearForceFactor {
    positive: Vec2,
    negative: Vec2,
}

impl LinearForceFactor {
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

#[derive(Component, Debug)]
pub struct AngularVelocity {
    value: f32,
    force: f32,
}

impl AngularVelocity {
    pub const ZERO: Self = Self {
        value: 0.0,
        force: 0.0,
    };
    pub fn new(value: f32, force: f32) -> Self {
        Self { value, force }
    }
    pub fn set(&mut self, value: f32) {
        self.value = value;
    }
    pub fn apply_force(&mut self, force: f32) {
        self.force += force;
    }
    pub fn flush(&mut self) {
        self.force = 0.0;
    }
}

#[derive(Component, Debug)]
pub struct AngularMaxVelocity {
    value: f32,
}

impl AngularMaxVelocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn set(&mut self, value: f32) {
        self.value = value;
    }
}

#[derive(Component, Debug)]
pub struct AngularForceFactor {
    positive: f32,
    negative: f32,
}

impl AngularForceFactor {
    pub fn new(positive: f32, negative: f32) -> Self {
        Self { positive, negative }
    }
    pub fn splat(value: f32) -> Self {
        Self::new(value, value)
    }
    pub fn set_positive(&mut self, positive: f32) {
        self.positive = positive;
    }
    pub fn set_negative(&mut self, negative: f32) {
        self.negative = negative;
    }
}

#[derive(Component, Debug)]
pub struct AngularFriction {
    value: f32,
}

impl AngularFriction {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn set(&mut self, value: f32) {
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

fn apply_linear_force_factor(mut query: Query<(&mut LinearVelocity, &LinearForceFactor)>) {
    for (mut velocity, force_factor) in query.iter_mut() {
        if velocity.value.x.signum() == velocity.force.x.signum() {
            velocity.force.x *= force_factor.positive.x;
        } else {
            velocity.force.x *= force_factor.negative.x;
        }

        if velocity.value.y.signum() == velocity.force.y.signum() {
            velocity.force.y *= force_factor.positive.y;
        } else {
            velocity.force.y *= force_factor.negative.y;
        }
    }
}

fn apply_linear_friction(mut query: Query<(&mut LinearVelocity, &LinearFriction)>) {
    for (mut velocity, friction) in query.iter_mut() {
        // This check is here in order to reduce wobble
        // This method reduces precision for really small velocity values
        if velocity.value.length_squared() < 1.0 {
            velocity.value = Vec2::ZERO;
        } else {
            let force = -velocity.value.signum() * velocity.value.abs().min(friction.value);
            velocity.apply_force(force);
        }
    }
}

fn update_rotation(mut query: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.rotate_z(velocity.value * time.delta_seconds());
    }
}

fn apply_angular_velocity(mut query: Query<&mut AngularVelocity>, time: Res<Time>) {
    for mut velocity in query.iter_mut() {
        velocity.value = velocity.value + velocity.force * time.delta_seconds();
        velocity.flush();
    }
}

fn apply_angular_max_velocity(mut query: Query<(&mut AngularVelocity, &AngularMaxVelocity)>) {
    for (mut velocity, max_velocity) in query.iter_mut() {
        velocity.value = velocity.value.signum() * velocity.value.abs().min(max_velocity.value);
    }
}

fn apply_angular_force_factor(mut query: Query<(&mut AngularVelocity, &AngularForceFactor)>) {
    for (mut velocity, force_factor) in query.iter_mut() {
        if velocity.value.signum() == velocity.force.signum() {
            velocity.force *= force_factor.positive;
        } else {
            velocity.force *= force_factor.negative;
        }
    }
}

fn apply_angular_friction(mut query: Query<(&mut AngularVelocity, &AngularFriction)>) {
    for (mut velocity, friction) in query.iter_mut() {
        // This check is here in order to reduce wobble
        // This method reduces precision for really small velocity values
        if velocity.value.abs() < 0.01 {
            velocity.value = 0.0;
        } else {
            let force = -velocity.value.signum() * velocity.value.abs().min(friction.value);
            velocity.apply_force(force);
        }
    }
}
