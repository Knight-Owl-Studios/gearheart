use bevy::prelude::*;
use crate::minion::{Minion, MinionBundle, TargetType};

#[derive(Resource)]
pub struct DragonAnimations {
    pub fly: Handle<AnimationClip>,
}

#[derive(Component)]
pub struct Dragon;

pub struct DragonPlugin;

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dragon)
          .add_systems(Update, setup_scene_once_loaded);
    }
}

fn spawn_dragon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Dragon,
        MinionBundle {
            minion: Minion {
              minion_type: TargetType::Average,
              ..Default::default()
            },
            model: SceneBundle {
              scene: asset_server.load("dragon.glb#Scene0"),
              transform: Transform::from_xyz(3.092422, -1.6510189, -1.001564),
              ..default()
          },
          ..Default::default()
        },
    ));

    commands.insert_resource(DragonAnimations {
        fly: asset_server.load("dragon.glb#Animation1"),
    });
}

fn setup_scene_once_loaded(
  animations: Res<DragonAnimations>,
  mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
  for mut player in &mut players {
      player.play(animations.fly.clone_weak()).repeat();    
  }
}
