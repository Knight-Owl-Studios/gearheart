use bevy::prelude::*;

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
