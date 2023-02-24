use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerImageAssets {
    #[asset(path = "textures/tank0.png")]
    pub tank0: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct BackgroundImageAssets {
    #[asset(path = "textures/background0.png")]
    pub background0: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct BulletImageAssets {
    #[asset(path = "textures/player_bullet0.png")]
    pub player_bullet0: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct EnemyImageAssets {
    #[asset(path = "textures/enemy0.png")]
    pub enemy0: Handle<Image>
}