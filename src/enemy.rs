use bevy::{prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

use crate::{EnemyImageAssets, SpriteSize,
            components::Enemy, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT}, 
            velocity::Velocity
        };

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyCount>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(enemy_spawn_system)
            );
    }
}


#[derive(Resource)]
pub struct EnemyCount(pub u32);
impl Default for EnemyCount {
    fn default() -> Self {
        Self(0)
    }
}


const ENEMY_MAX: u32 = 6;
pub const ENEMY_SIZE: (f32, f32) = (60.0, 60.0);

fn enemy_spawn_system(
    mut commands: Commands,
    enemy_image_assets: Res<EnemyImageAssets>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    if enemy_count.0 < ENEMY_MAX {  

        let mut rng = thread_rng();

        let x = rng.gen_range( -(X_DIRECTION_LIMIT - ENEMY_SIZE.0 / 2.0)..(X_DIRECTION_LIMIT - ENEMY_SIZE.0 / 2.0) );
        let y = rng.gen_range( -(Y_DIRECTION_LIMIT - ENEMY_SIZE.1 / 2.0)..(Y_DIRECTION_LIMIT - ENEMY_SIZE.1 / 2.0));

        commands.spawn(
            SpriteBundle {
                texture: enemy_image_assets.enemy0.clone_weak(),
                transform: Transform{
                    translation:Vec3::new(x, y, 10.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(SpriteSize::from(ENEMY_SIZE).0),
                    ..Default::default()
                },                
                ..Default::default()
            }       
        )
        .insert(Enemy)
        .insert(Velocity::default())
        .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}