use bevy::prelude::*;

use crate::ammunition::DamageEvent;

#[derive(Component, Debug)]
pub struct Health {
    pub value: i32,
    pub max: i32,
}


pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_damage);
    }
}

fn apply_damage(
  mut health_query: Query<&mut Health>,
  mut damage_event_reader: EventReader<DamageEvent>,
) {
    for event in damage_event_reader.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.value -= event.amount;
            if health.value <= 0 {
                info!("Entity {:?} has died!", event.target);
                health.value = 0;
            }
        }
    }
}
