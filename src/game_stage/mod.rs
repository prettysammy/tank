mod game_start;
mod game_over;
mod next_level;


use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;

use crate::assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets, UIImageAssets, FontAssets};

use self::{game_over::GameOverPlugin, 
           next_level::NextLevelPlugin,
           game_start::GameStartPlugin, 
          };

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum GameStage {
    Loading,
    GameStart,
    Main,
    NextLevel,
    GameOver,
}


#[derive(Resource)]
pub(crate) struct SingleLevelTimer(pub Timer, bool);
impl Default for SingleLevelTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(30.0, TimerMode::Repeating), false)
    }
}

#[derive(Resource)]
pub struct GameLevel {
    pub level: u32,
}
impl Default for GameLevel{
    fn default() -> Self {
        Self { level: 1 }
    }
}


pub struct GameStagePlugin;
impl Plugin for GameStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameStage::Loading)
            //.continue_to_state(GameStage::Main)
            .continue_to_state(GameStage::GameStart)
            .with_collection::<FontAssets>()
            .with_collection::<PlayerImageAssets>()
            .with_collection::<BackgroundImageAssets>()
            .with_collection::<BulletImageAssets>()
            .with_collection::<EnemyImageAssets>()
            .with_collection::<UIImageAssets>()
        )
        .add_state(GameStage::Loading)
        
        .init_resource::<SingleLevelTimer>()
        .init_resource::<GameLevel>()

        .add_plugin(GameStartPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(NextLevelPlugin);

    }
}