use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component)]
pub struct DistanceTravelled {
    pub value: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self {
            velocity: Velocity { value: Vec3::ZERO},
            acceleration: Acceleration { value: Vec3::ZERO },
        }
    }
}

#[derive(Component)]

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
