mod game_over;



use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets, UIImageAssets, FontAssets};

use self::game_over::GameOverPlugin;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum GameStage {
    Loading,
    Main,
    GameOver,
}

pub struct GameStagePlugin;
impl Plugin for GameStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameStage::Loading)
            .continue_to_state(GameStage::Main)
            .with_collection::<FontAssets>()
            .with_collection::<PlayerImageAssets>()
            .with_collection::<BackgroundImageAssets>()
            .with_collection::<BulletImageAssets>()
            .with_collection::<EnemyImageAssets>()
            .with_collection::<UIImageAssets>()
        )
        .add_state(GameStage::Loading)
        
        .add_plugin(GameOverPlugin);
    }
}