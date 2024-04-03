use crate::minions::Minion;
use crate::states::MyStates;
use crate::towers::Tower;
use bevy::prelude::*;

#[derive(Component)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct InRange {
    pub entities: Vec<Entity>,
}

pub struct RangePlugin;

impl Plugin for RangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, in_range.run_if(in_state(MyStates::Game)));
    }
}

fn in_range(
    minions_query: Query<(&Transform, Entity), With<Minion>>,
    mut towers_query: Query<(&Transform, &mut InRange, &Range), With<Tower>>,
) {
    // loop through all minions and and check if they are in range of any towers
    for (minion_transform, entity) in minions_query.iter() {
        for (tower_transform, mut in_range, range) in towers_query.iter_mut() {
            in_range.entities.clear();
            let distance = minion_transform
                .translation
                .distance(tower_transform.translation);
            if distance >= range.min
                && distance <= range.max
                && !in_range.entities.contains(&entity)
            {
                in_range.entities.push(entity);
            }
        }
    }
}
