use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BoltTowerAssets {
    #[asset(path = "towers/bolt_tower.glb#Scene0")]
    pub bolt_tower: Handle<Scene>,

    #[asset(path = "ammunition/bolt.glb#Scene0")]
    pub bolt: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct WizardTowerAssets {
    #[asset(path = "towers/wizard_tower.glb#Scene0")]
    pub wizard_tower: Handle<Scene>,

    #[asset(path = "ammunition/fireball.glb#Scene0")]
    pub fireball: Handle<Scene>,
}
