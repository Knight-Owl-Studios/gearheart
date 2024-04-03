use crate::{states::MyStates, towers::range::InRange};
use bevy::prelude::*;
use rand::seq::SliceRandom; // 0.7.2

#[derive(Component)]
pub struct Targeting {
    pub target: Option<Entity>,
}

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, target.run_if(in_state(MyStates::Game)));
    }
}
// assign new target if none exists. Currently just randomly picks from
// available in range entities
fn target(mut tower_query: Query<(&mut Targeting, &InRange)>) {
    for (mut targeting, in_range) in tower_query.iter_mut() {
        if targeting.target.is_some() && !in_range.entities.contains(&targeting.target.unwrap()) {
            info!("Untargeting entity: {:?}", targeting.target);
            targeting.target = None;
        }

        if targeting.target.is_some() || in_range.entities.is_empty() {
            continue;
        }

        let target = in_range.entities.choose(&mut rand::thread_rng()).cloned();
        info!("Targeting entity: {:?}", target);
        // assign new target at random
        targeting.target = target;
    }
}
