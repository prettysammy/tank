mod game_start;
mod game_over;
mod next_level;


use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;

use crate::assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets, UIImageAssets, FontAssets, SkillsImageAssets};

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


const SINGLE_LEVEL_TIME: f32 = 30.0;

#[derive(Resource)]
pub(crate) struct SingleLevelTimer(pub Timer, bool);
impl Default for SingleLevelTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SINGLE_LEVEL_TIME, TimerMode::Repeating), false)
    }
}

#[derive(Resource, Clone, Copy)]
pub struct GameLevel {
    pub level: u32,
    
    pub enemy_hp_enhance: f32,
    pub enemy_atk_enhance: f32,
    pub enemy_def_enhance: f32,
    pub enemy_number_enhance: f32,

}
const LEVEL_INCREASE_RATE: f32 = 1.1;

impl Default for GameLevel{
    fn default() -> Self {
        Self { 
                level: 1,
                enemy_hp_enhance: 1.0,
                enemy_atk_enhance: 1.0,
                enemy_def_enhance: 1.0,
                enemy_number_enhance: 1.0,
             }
    }
}

impl GameLevel {
    pub fn increase_for_next_level(&mut self) {
        
        self.level += 1;
        self.enemy_hp_enhance  *= LEVEL_INCREASE_RATE;
        self.enemy_atk_enhance *= LEVEL_INCREASE_RATE;
        self.enemy_def_enhance *= LEVEL_INCREASE_RATE;
        self.enemy_number_enhance *= LEVEL_INCREASE_RATE;

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
            .with_collection::<SkillsImageAssets>()
        )
        .add_state(GameStage::Loading)
        
        .init_resource::<SingleLevelTimer>()
        .init_resource::<GameLevel>()

        .add_plugin(GameStartPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(NextLevelPlugin);

    }
}