use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::states::MyStates;

use crate::map::asset::MapAssets;
use crate::minions::dragon::assets::DragonAssets;
use crate::towers::assets::{BoltTowerAssets, WizardTowerAssets};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Game)
                .load_collection::<BoltTowerAssets>()
                .load_collection::<WizardTowerAssets>()
                .load_collection::<DragonAssets>()
                .load_collection::<MapAssets>(),
        );
    }
}
