use bevy::{prelude::*, ecs::schedule::ShouldRun};

use crate::{ SpriteSize, PlayerImageAssets, GameStage, BulletImageAssets,
            marks::RoleType,
            velocity::Velocity,             
            components::{ Player, Bullet},  
            };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {       
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main).with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(player_keyboard_event_system)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(player_fire_criteria)
                .with_system(player_fire_system)
        )        
        ;
    }
}


const PLAYER_SIZE: (f32, f32) = (60.0, 60.0); 
const BULLET_SIZE:  (f32, f32) = (16.0, 16.0); 
const PLAY_BULLET_SPEED: f32 = 2.0;

fn setup(
    mut commands: Commands,
    player_image_assets: Res<PlayerImageAssets>
) {
    commands.spawn(
        SpriteBundle{
            texture: player_image_assets.tank0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0), 
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(SpriteSize::from(PLAYER_SIZE).0),
                ..Default::default()
            },
            ..Default::default()
        }       
    )
    .insert(Player)
    .insert(RoleType::Player)
    .insert(Velocity::default())
    .insert(SpriteSize::from(PLAYER_SIZE))
    .insert(PlayerFireTimer::default());
}

#[derive(Component)]
struct PlayerFireTimer(Timer);
impl Default for PlayerFireTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}



fn player_fire_criteria(
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



fn player_fire_system(
    mut commands: Commands,
    bullet_image_assets: Res<BulletImageAssets>,
    query: Query<(&Transform, &Velocity), With<Player>>
) {
    if let Ok((player_tf, player_velocity)) = query.get_single(){
        let (pos_x, pos_y) = (player_tf.translation.x ,player_tf.translation.y);
        
        let velocity_x = player_velocity.x * PLAY_BULLET_SPEED;
        let mut velocity_y = player_velocity.y * PLAY_BULLET_SPEED;


        //TODO静止状态下根据前一次状态确定方向，先无脑向上
        if velocity_x == 0.0 && velocity_y == 0.0 {
            velocity_y = 2.0;
        }


        let mut spawn_bullet = || {
            commands.spawn(
                SpriteBundle {
                    texture: bullet_image_assets.player_bullet0.clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(SpriteSize::from(BULLET_SIZE).0),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, 5.0),
                        //rotation: Quat::from_rotation_x(30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bullet)
                .insert(Velocity {x: velocity_x, y: velocity_y})
                .insert(SpriteSize::from(BULLET_SIZE));
        };

        spawn_bullet();

    }
 
}



fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut(){
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.0
        } else if kb.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        velocity.y = if kb.pressed(KeyCode::Down) {
            -1.0
        } else if kb.pressed(KeyCode::Up) {
            1.0
        } else {
            0.0
        }       
    }
}