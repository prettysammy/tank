mod enemy;

use bevy::{prelude::*, time::FixedTimestep, ecs::schedule::ShouldRun};
use serde::{Deserialize, Serialize};

use crate::{GameStage, utils::{Val, Pool}};

use self::enemy::*;

const ENEMY_MAX: u32 = 6;
const ENEMY_SPEED: f32 = 0.3;
pub const ENEMY_SIZE: (f32, f32) = (32.0, 48.0);
pub const ENEMY_TOMB_SIZE: (f32, f32) = (40.0, 40.0);

#[derive(Component)]
pub struct EnemyHpBar(pub Entity);

#[derive(Resource, Deserialize, Serialize)]
pub struct EnemyInfo {
    pub name: String,
    pub max_hp: i64,
    pub atk: i64,
    pub def: i64,
    pub gold: Val,
}
impl Default for EnemyInfo {
    fn default() -> Self {
        Self { 
            name: "zombie".to_string(),
            max_hp: 20,
            atk: 5,
            def: 1,
            gold: Val::Float(1, 3)
        }
    }
}



#[derive(Component)]
pub struct EnemyStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: Val,
}

impl Default for EnemyStatus {
    fn default() -> Self {
        Self { 
            atk: 5,
            def: 1,
            cur_hp: 20,
            max_hp: 20,
            gold: Val::Float(1, 3)
        }
    }
}

impl From<&EnemyInfo> for EnemyStatus {
    fn from(enemy_info: &EnemyInfo) -> Self {
        EnemyStatus {
            atk: enemy_info.atk,
            def: enemy_info.def,
            cur_hp: enemy_info.max_hp,
            max_hp: enemy_info.max_hp,
            gold: enemy_info.gold,            
        }
    }
}

#[derive(Resource)]
pub struct EnemyCount(pub u32);
impl Default for EnemyCount {
    fn default() -> Self {
        Self(0)
    }
}

//timer and useflag
#[derive(Resource)]
pub(crate) struct EnemySpawnTimer(Timer, bool);
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating), false)
    }
}

pub(crate) fn enemy_spawn_criteria(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) -> ShouldRun {
    //timer is not ready for use
    if enemy_spawn_timer.1 == false {
        return ShouldRun::No;
    }
    
    //timer is ready for use
    if enemy_spawn_timer.0.tick(time.delta()).just_finished(){
        return ShouldRun::Yes;
    } else {
        return ShouldRun::No;
    }    
}


#[derive(Component)]
pub struct EnemyAttackTimer(pub Timer);
impl Default for EnemyAttackTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct EnemyTombTimer(pub Timer);
impl Default for EnemyTombTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemyCount>()
        .init_resource::<EnemySpawnTimer>();

        app
            .add_system_set(
                SystemSet::on_enter(GameStage::Main)
                .with_system(init_main_system)
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    .with_run_criteria(enemy_spawn_criteria)
                    .with_system(enemy_spawn_system)
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(update_enemy_texture_atlas_system)
            )            
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    .with_system(update_enemy_status_system)
                    .with_system(enemy_move_system)
            )
            .add_system_set(
                SystemSet::on_exit(GameStage::Main)
                .with_system(exit_main_system)
            );
    }
}


pub fn get_enemy_pool() -> Pool<EnemyInfo> {
    let config = include_str!("../../assets/pool/enemy.ron");
    ron::from_str(config).unwrap()
}