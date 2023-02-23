use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerImageAssets {
    #[asset(path = "textures/tank0.png")]
    pub tank0: Handle<Image>
}