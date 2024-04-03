use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct DragonAssets {
    #[asset(path = "minions/dragon.glb#Scene0")]
    pub dragon: Handle<Scene>,

    #[asset(path = "minions/dragon.glb#Animation1")]
    pub dragon_animation_move: Handle<AnimationClip>,
    #[asset(path = "minions/dragon.glb#Animation0")]
    pub dragon_animation_die: Handle<AnimationClip>,
}
