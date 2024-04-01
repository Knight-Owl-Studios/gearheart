use bevy::prelude::*;
use rand::seq::SliceRandom; // 0.7.2
use crate::range::InRange;

#[derive(Component)]
pub struct Targeting {
    pub target: Option<Entity>
}

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (untarget).before(target));
    }
}


fn untarget(
  mut tower_query: Query<(&mut Targeting, &InRange)>,
) {
    // remove_target if it's no longer in in_range
    for (mut targeting, in_range) in tower_query.iter_mut() {
        if let Some(target) = targeting.target {
            if !in_range.entities.contains(&target) {
                info!("Untargeting entity: {:?}", target);
                targeting.target = None;
            }
        }
    }
}

// assign new target if none exists. Currently just randomly picks from
// available in range entities
fn target(
    mut tower_query: Query<(&mut Targeting, &InRange)>
) {
    for (mut targeting, in_range) in tower_query.iter_mut() {
        if targeting.target.is_some() || in_range.entities.is_empty() {
            continue;
        }

        let target = in_range.entities.choose(&mut rand::thread_rng()).cloned();
        info!("Targeting entity: {:?}", target);
        // assign new target at random
        targeting.target = target;
    }
}
