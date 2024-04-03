use bevy::prelude::*;

use crate::map::Path;
use crate::minions::health::Health;
use crate::minions::movement::{MovementBundle, Velocity};
use crate::states::MyStates;

#[derive(Component, Debug)]
pub struct Minion {
    pub minion_type: TargetType,
    pub path_target: Option<Vec3>,
    pub speed: f32,
}

impl Default for Minion {
    fn default() -> Self {
        Self {
            minion_type: TargetType::Average,
            path_target: None,
            speed: 5.0,
        }
    }
}

#[derive(Debug)]
pub enum TargetType {
    Swarm,
    Tank,
    Average,
}

#[derive(Bundle)]
pub struct MinionBundle {
    pub minion: Minion,
    pub movement: MovementBundle,
    pub model: SceneBundle,
    pub health: Health,
}

impl Default for MinionBundle {
    fn default() -> Self {
        Self {
            minion: Minion::default(),
            movement: MovementBundle::default(),
            model: SceneBundle {
                ..Default::default()
            },
            health: Health {
                max: 100,
                current: 100,
            },
        }
    }
}

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (choose_path_target, move_minion).run_if(in_state(MyStates::Game)),
        );
    }
}

fn choose_path_target(path: Option<Res<Path>>, mut minion_query: Query<(&mut Minion, &Transform)>) {
    let path = match path {
        Some(path) => path,
        None => return,
    };

    // loop through minions setting path target to the next point in the path
    for (mut minion, transform) in minion_query.iter_mut() {
        if minion.path_target.is_none() {
            minion.path_target = Some(path.points[0]);
            continue;
        }

        // if minion arrive at point, set next point as target
        if transform.translation.distance(minion.path_target.unwrap()) < 0.1 {
            // find current point index and set point at index + 1 as path_target
            let current_point_index = path
                .points
                .iter()
                .position(|&point| point == minion.path_target.unwrap());
            let next_point = path.points.get(current_point_index.unwrap() + 1);

            if let Some(next_point) = next_point {
                minion.path_target = Some(*next_point);
            } else {
                minion.path_target = None;
                // Minion has reach the end of the path
                info!("Minion has reach the end of the path")
            }
        }
    }
}

fn move_minion(mut minion_query: Query<(&Minion, &mut Transform, &mut Velocity)>) {
    for (minion, mut transform, mut velocity) in minion_query.iter_mut() {
        // move minion closer to path target
        if let Some(path_target) = minion.path_target {
            let direction = path_target - transform.translation;
            let distance = direction.length();
            let direction = direction.normalize();
            if distance > 0.1 {
                velocity.value = direction * minion.speed;
            } else {
                velocity.value = Vec3::ZERO;
            }

            // ensure minion faces the right way
            if direction != Vec3::ZERO {
                transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
            }
        }
    }
}
