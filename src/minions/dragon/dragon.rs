use bevy::prelude::*;

use crate::minions::dragon::assets::DragonAssets;
use crate::minions::health::Health;
use crate::minions::{Minion, MinionBundle, TargetType};
use crate::states::MyStates;

#[derive(Resource)]
pub struct DragonAnimations {
    pub fly: Handle<AnimationClip>,
    pub die: Handle<AnimationClip>,
}

#[derive(Component)]
pub struct Dragon;

pub struct DragonPlugin;

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Game), spawn_dragon)
            .add_systems(
                Update,
                setup_scene_once_loaded.run_if(in_state(MyStates::Game)),
            );
    }
}

fn spawn_dragon(mut commands: Commands, dragon_assets: Res<DragonAssets>) {
    commands.spawn((
        Dragon,
        MinionBundle {
            minion: Minion {
                minion_type: TargetType::Average,
                ..Default::default()
            },
            model: SceneBundle {
                scene: dragon_assets.dragon.clone(),
                transform: Transform::from_xyz(3.092422, -1.6510189, -1.001564),
                ..default()
            },
            health: Health {
                max: 100,
                current: 100,
            },
            ..Default::default()
        },
    ));
}

fn setup_scene_once_loaded(
    dragon_assets: Res<DragonAssets>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player
            .play(dragon_assets.dragon_animation_move.clone_weak())
            .repeat();
    }
}
