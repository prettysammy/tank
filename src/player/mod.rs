mod player;


use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{GameStage, utils::Pool};
use self::player::*;

const PLAYER_SIZE: (f32, f32) = (60.0, 60.0); 


#[derive(Resource, Deserialize, Serialize)]
pub struct PlayerInfo {
    pub name: String,
    pub role: PlayerRole,
    pub max_hp: i64,
    pub atk: i64,
    pub def: i64,
}
impl Default for PlayerInfo {
    fn default() -> Self {
        Self { 
            role: PlayerRole::TANK0,
            name: "tank0".to_string(),
            max_hp: 100,
            atk: 10,
            def: 2,
        }
    }
}

#[derive(Resource)]
pub struct PlayerStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: i64,
    pub level: usize,
    pub is_init: bool,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self { 
            atk: 10,
            def: 2,
            cur_hp: 100,
            max_hp: 100,
            gold: 10,
            level: 1,
            is_init: true,
        }
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum PlayerRole {
    TANK0,
    TANK1,
    TANK2,
    TANK3,
}

#[derive(Debug, Component, Clone, Copy,Deserialize, Serialize)]
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
        .init_resource::<PlayerInfo>()
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

pub fn get_player_pool() -> Pool<PlayerInfo> {
    let config = include_str!("../../assets/pool/player.ron");
    ron::from_str(config).unwrap()
}