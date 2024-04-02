use bevy::prelude::*;

use crate::movement::ArrivedEvent;

#[derive(Component, Debug)]
pub struct RateOfFire {
  pub timer: Timer,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DamageType {
  Physical,
  Fire,
}

#[derive(Component, Debug, Clone)]
pub struct Damage {
  pub amount: i32,
  pub damage_type: DamageType,
}

#[derive(Event, Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
}

impl DamageEvent {
    pub fn new(target: Entity, amount: i32) -> Self {
        Self { target, amount }
    }
}

impl Default for Damage {
  fn default() -> Self {
    Self {
      amount: 1,
      damage_type: DamageType::Physical,
    }
  }
}

#[derive(Component, Debug)]
pub struct Ammunition {
  pub damage: Damage,
  pub scene: Handle<Scene>,
  pub speed: f32
}
#[derive(Component, Debug)]
pub struct Ammo; // Marker

pub struct AmmunitionPlugin;

impl Plugin for AmmunitionPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_event::<DamageEvent>()
          .add_systems(Update, despawn_ammunition);
    }
}


fn despawn_ammunition(
    mut commands: Commands,
    query: Query<(Entity), With<Ammo>>,
    mut event_reader: EventReader<ArrivedEvent>,
) {
    for &ArrivedEvent { entity } in event_reader.read() {
      // despawn entitiy when it arrives at its target. Make sure its ammunition by passing the query
      if query.get(entity).is_ok() {
        commands.entity(entity).despawn_recursive();
      }
    }
}
