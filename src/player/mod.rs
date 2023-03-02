mod player;
use crate::GameStage;

use bevy::prelude::*;
use self::player::*;

const PLAYER_SIZE: (f32, f32) = (50.0, 60.0); 

#[derive(Resource)]
pub struct PlayerStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: i64,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self { 
            atk: 10,
            def: 1,
            cur_hp: 100,
            max_hp: 100,
            gold: 10,
        }
    }
}

#[derive(Component)]
pub enum PlayerStatusType {
    ATK,
    DEF,
    HP,
    GOLD,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {       
        app
        .init_resource::<PlayerStatus>()
        .add_system_set(
            SystemSet::on_enter(GameStage::Main)
            .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(player_keyboard_event_system)
            .with_system(update_player_status_system)
            .with_system(player_move_system)
        );
    }
}