use bevy::prelude::*;

use crate::{range::Range, targeting::Targeting};

#[derive(Component)]
pub struct Tower {}

#[derive(Bundle)]
pub struct TowerBundle {
    pub model: SceneBundle,
    // ammunitions: Ammunitions,
    // rate_of_fire: RateOfFire,
    pub range: Range,
    pub targeting: Targeting,
}
