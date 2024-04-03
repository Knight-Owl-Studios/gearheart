use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct MapAssets {
    #[asset(path = "map/map.glb#Scene0")]
    pub level: Handle<Scene>,
}
