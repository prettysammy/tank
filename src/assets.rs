use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/hanti.ttf")]
    pub hanti: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub bold: Handle<Font>
}


#[derive(AssetCollection, Resource)]
pub struct PlayerImageAssets {
    #[asset(path = "textures/tank0.png")]
    pub tank0: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct BackgroundImageAssets {
    #[asset(path = "textures/background0.png")]
    pub background0: Handle<Image>,

    #[asset(path = "textures/background", collection(typed))]
    pub background_tiles: Vec<Handle<Image>>,     
}

#[derive(AssetCollection, Resource)]
pub struct BulletImageAssets {
    #[asset(path = "textures/player_bullet0.png")]
    pub player_bullet0: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct EnemyImageAssets {
    #[asset(texture_atlas(tile_size_x = 155.0, tile_size_y = 175.0, columns = 2, rows = 1))]
    #[asset(path = "textures/kun-altes.png")]
    pub enemy1: Handle<TextureAtlas>,

    #[asset(path = "textures/shit.png")]
    pub tomb: Handle<Image>,    
}

#[derive(AssetCollection, Resource)]
pub struct UIImageAssets {
    #[asset(path = "textures/ui/status_ui.png")]
    pub main_panel: Handle<Image>,

    #[asset(path = "textures/ui/hp.png")]
    pub icon_hp: Handle<Image>,

    #[asset(path = "textures/ui/atk.png")]
    pub icon_atk: Handle<Image>,

    #[asset(path = "textures/ui/def.png")]
    pub icon_def: Handle<Image>,  

    #[asset(path = "textures/ui/gold.png")]
    pub icon_gold: Handle<Image>,        
}