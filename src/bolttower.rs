use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use crate::{ammunition::{Ammunition, Damage, DamageType, RateOfFire}, minion::Minion, range::{InRange, Range}, targeting::Targeting, tower::{Tower, TowerBundle}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(-6.1691093, -1.1100681, -7.9348316);
const STARTING_TRANSLATION_2: Vec3 = Vec3::new(6.4087896, -1.1100681, -7.9131517);

#[derive(Component)]
pub struct BoltTower;

pub struct BoltTowerPlugin;

impl Plugin for BoltTowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tower)
            .add_systems(Update, track_target);
    }
}

fn track_target(
    mut tower_query: Query<(&Targeting, &mut Transform), With<BoltTower>>,
    target_query: Query<&Transform, (With<Minion>, Without<BoltTower>)>,
) {
    // return if tower has no target
    for (targeting, mut transform) in tower_query.iter_mut() {
        if targeting.target.is_none() {
            continue;
        }

        let target = target_query.get(targeting.target.unwrap()).unwrap();

        let direction_to_target = target.translation.truncate() - transform.translation.truncate(); // Target point - current position, ignore Z
        let angle_to_target = direction_to_target.y.atan2(direction_to_target.x); // Calculate the angle in radians

        // Bevy uses a left-handed coordinate system, so we negate the angle for correct orientation
        // Adjust the rotation to face the target, rotating around the Z axis
        transform.rotation = Quat::from_rotation_y(angle_to_target + PI / 2.0); // Add PI/2 because the default forward vector is -Y

    }
    
}

fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Bolt_Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            range: Range { min: 0.0, max: 10.0 },
            targeting: Targeting { target: None },
            ammunition: Ammunition {
                damage: Damage {
                    amount: 1,
                    damage_type: DamageType::Fire,
                },
                scene: asset_server.load("bolt.glb#Scene0"),
                speed: 30.0,
            },
            in_range: InRange { entities: Vec::new() },
            rate_of_fire: RateOfFire { timer: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Repeating)},
            tower: Tower,
        },
        BoltTower,
    ));
    commands.spawn((
        TowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Bolt_Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION_2),
                ..default()
            },
            range: Range { min: 0.0, max: 10.0 },
            targeting: Targeting { target: None },
            ammunition: Ammunition {
                damage: Damage {
                    amount: 1,
                    damage_type: DamageType::Fire,
                },
                scene: asset_server.load("bolt.glb#Scene0"),
                speed: 30.0,
            },
            in_range: InRange { entities: Vec::new() },
            rate_of_fire: RateOfFire { timer: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Repeating)},
            tower: Tower,
        },
        BoltTower,
    ));

}
