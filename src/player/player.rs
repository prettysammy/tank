use bevy::{prelude::*};

use crate::{ SpriteSize, PlayerImageAssets,BulletImageAssets,
            velocity::Velocity,             
            components::{ Player, Bullet},  
            };

use super::{PlayerStatus, PlayerStatusType, PLAYER_SIZE, BULLET_SIZE, PLAY_BULLET_SPEED, PlayerFireTimer};


pub(crate) fn setup(
    mut commands: Commands,
    player_image_assets: Res<PlayerImageAssets>
) {
    commands.insert_resource(PlayerStatus::default());

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
    .insert(Velocity::default())
    .insert(SpriteSize::from(PLAYER_SIZE))
    .insert(PlayerFireTimer::default());
}


pub(crate) fn player_fire_system(
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


pub(crate) fn update_player_status_system(
    player_status: Res<PlayerStatus>,
    mut query_text: Query<(&mut Text, &PlayerStatusType)>
) {
    if player_status.is_changed() {
        for (mut text, status_type) in query_text.iter_mut() {
            text.sections[0].value =match status_type {
                PlayerStatusType::ATK => format!("{}", player_status.atk),
                PlayerStatusType::DEF => format!("{}", player_status.def),
                PlayerStatusType::HP => format!("{}/{}", player_status.cur_hp, player_status.max_hp),
                PlayerStatusType::GOLD => format!("{}", player_status.gold),
            };
        }
    }
}


pub(crate) fn player_keyboard_event_system(
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