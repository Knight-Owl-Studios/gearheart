use bevy::prelude::*;

use crate::minions::dragon::Dragon;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform), With<Dragon>>) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform) in query.iter() {
        // info!(
        //     "Entity {:?} is at position {:?},",
        //     entity, transform.translation
        // );
    }
}
