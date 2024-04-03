use bevy::prelude::*;

use crate::{minions::Minion, states::MyStates};

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
pub struct Tracking {
    pub target: Entity,
    pub speed: f32,
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
            velocity: Velocity { value: Vec3::ZERO },
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
            (update_velocity, update_position, move_to_target)
                .chain()
                .run_if(in_state(MyStates::Game)),
        )
        .add_event::<ArrivedEvent>();
    }
}

#[derive(Event, Debug)]
pub struct ArrivedEvent {
    pub entity: Entity,
}

impl ArrivedEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

fn move_to_target(
    mut query: Query<(&Tracking, &mut Transform, Entity), Without<Minion>>,
    target: Query<&Transform, With<Minion>>,
    time: Res<Time>,
    mut event_writer: EventWriter<ArrivedEvent>,
) {
    for (tracking, mut transform, entity) in query.iter_mut() {
        let target_transform = target.get(tracking.target).unwrap();
        let direction = target_transform.translation - transform.translation;
        let distance = direction.length();
        let direction = direction.normalize();

        if distance < 0.1 {
            event_writer.send(ArrivedEvent::new(entity));
            continue;
        }

        transform.translation += direction * tracking.speed * time.delta_seconds();
        transform.look_at(target_transform.translation, Vec3::Y);
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
