mod player;
use crate::GameStage;

use bevy::{prelude::*, ecs::schedule::ShouldRun};
use self::player::*;



const PLAYER_SIZE: (f32, f32) = (60.0, 60.0); 
const BULLET_SIZE:  (f32, f32) = (16.0, 16.0); 
const PLAY_BULLET_SPEED: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum AliveStatus {
    ALIVE,
    DIE,
}

#[derive(Resource)]
pub struct PlayerStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: i64,
    pub alive: AliveStatus,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self { 
            atk: 10,
            def: 1,
            cur_hp: 100,
            max_hp: 100,
            gold: 10,
            alive: AliveStatus::ALIVE, 
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

#[derive(Component)]
pub(crate) struct PlayerFireTimer(Timer);
impl Default for PlayerFireTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.3, TimerMode::Repeating))
    }
}

pub(crate) fn player_fire_criteria(
    mut query: Query<&mut PlayerFireTimer>,
    time: Res<Time>,
) -> ShouldRun {
    for mut player_fire_timer in query.iter_mut(){
        if player_fire_timer.0.tick(time.delta()).just_finished(){
            return ShouldRun::Yes;
        } else {
            return ShouldRun::No;
        }
    }
    return ShouldRun::No;
}



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {       
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main)
            .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(player_keyboard_event_system)
            .with_system(update_player_status_system)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
                .with_run_criteria(player_fire_criteria)
                .with_system(player_fire_system)
        )        
        ;
    }
}