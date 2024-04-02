use bevy::prelude::*;

use crate::{ammunition::{Ammunition, Damage, DamageEvent, RateOfFire}, movement::Tracking, range::{InRange, Range}, targeting::Targeting};

#[derive(Component)]
pub struct Tower;

#[derive(Bundle)]
pub struct TowerBundle {
    pub tower: Tower,
    pub model: SceneBundle,
    pub ammunition: Ammunition,
    pub rate_of_fire: RateOfFire,
    pub range: Range,
    pub in_range: InRange,
    pub targeting: Targeting,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_ze_missiles);
    }
}


fn fire_ze_missiles(
    mut commands: Commands,
    mut fireable_query: Query<(&mut RateOfFire, &Targeting, &Ammunition, &Transform), With<Tower>>,
    target_query: Query<&Transform, Without<Tower>>,
    mut event_writer: EventWriter<DamageEvent>,
    time: Res<Time>,
) {
    for (mut rate_of_fire, target, ammo, transform) in fireable_query.iter_mut() {
        if let Some(target) = target.target {
            rate_of_fire.timer.tick(time.delta());

            if rate_of_fire.timer.finished() {
                commands.spawn((
                    SceneBundle {
                        scene: ammo.scene.clone(),
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                    Tracking { target, speed: ammo.speed },
                    Damage { amount: ammo.damage.amount, damage_type: ammo.damage.damage_type },
                ));
            }
        } else {
            rate_of_fire.timer.reset();
        }
    }
}
