use bevy::prelude::*;

use crate::{range::{Range, InRange}, targeting::Targeting};

#[derive(Component)]
pub struct Tower;

#[derive(Bundle)]
pub struct TowerBundle {
    pub tower: Tower,
    pub model: SceneBundle,
    // ammunitions: Ammunitions,
    // rate_of_fire: RateOfFire,
    pub range: Range,
    pub in_range: InRange,
    pub targeting: Targeting,
}

impl Default for TowerBundle {
    fn default() -> Self {
        Self {
            tower: Tower,
            model: SceneBundle {
                ..Default::default()
            },
            range: Range { min: 0.0, max: 30.0 },
            in_range: InRange { entities: Vec::new() },
            targeting: Targeting { target: None },
        }
    }
}
