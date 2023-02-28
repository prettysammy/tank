mod enemy;

use bevy::{prelude::*, time::FixedTimestep, ecs::schedule::ShouldRun};

use crate::GameStage;

use self::enemy::*;

const ENEMY_MAX: u32 = 6;
pub const ENEMY_SIZE: (f32, f32) = (80.0, 80.0);
pub const ENEMY_TOMB_SIZE: (f32, f32) = (40.0, 40.0);

#[derive(Component)]
pub struct EnemyHpBar(pub Entity);

#[derive(Component)]
pub struct EnemyStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
}

impl Default for EnemyStatus {
    fn default() -> Self {
        Self { 
            atk: 5,
            def: 1,
            cur_hp: 20,
            max_hp: 20,
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

#[derive(Resource)]
pub(crate) struct EnemySpawnTimer(Timer);
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

pub(crate) fn enemy_spawn_criteria(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) -> ShouldRun {
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
            )
            .add_system_set(
                SystemSet::on_exit(GameStage::Main)
                .with_system(exit_main_system)
            );
    }
}