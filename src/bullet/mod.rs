mod bullet;



use bevy::{prelude::*, ecs::schedule::ShouldRun};
use crate::GameStage;
use self::bullet::*;

const BULLET_SIZE:  (f32, f32) = (16.0, 16.0); 
const PLAY_BULLET_SPEED: f32 = 1.0;


#[derive(Resource)]
pub(crate) struct BulletSpawnTimer(Timer);
impl Default for BulletSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

pub(crate) fn bullet_spawn_criteria(
    mut bullet_spawn_timer: ResMut<BulletSpawnTimer>,
    time: Res<Time>,
) -> ShouldRun {
    if bullet_spawn_timer.0.tick(time.delta()).just_finished(){
        return ShouldRun::Yes;
    } else {
        return ShouldRun::No;
    } 
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<BulletSpawnTimer>()
        .init_resource::<WorldMouse>();

        app
        .add_system_set(
            SystemSet::on_enter(GameStage::Main)
            .with_system(init_main_system)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(bullet_move_system)
            .with_system(current_world_mouse_system)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
                .with_run_criteria(bullet_spawn_criteria)
                .with_system(bullet_spawn_system)
        )
        .add_system_set(
            SystemSet::on_exit(GameStage::Main)
            .with_system(exit_main_system)
        );
    }
}