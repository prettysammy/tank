use bevy::{prelude::*, math::Vec3Swizzles, sprite::collide_aabb::collide};

use crate::{ SpriteSize, PlayerImageAssets, TIME_STEP, BASE_SPEED,          
            components::{ Player, Velocity, Enemy},
            events::PlayerMoveEvent, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT},  
            };

use super::{PlayerStatus, PlayerStatusType, PLAYER_SIZE, PlayerInfo};


pub(crate) fn setup(
    mut commands: Commands,
    player_info: Res<PlayerInfo>,
    mut player_status: ResMut<PlayerStatus>,
    player_image_assets: Res<PlayerImageAssets>
) {
    //game over时刷新
    //commands.insert_resource(PlayerStatus::default());

    if player_status.is_init {
        player_status.atk = player_info.atk;
        player_status.def = player_info.def;
        player_status.cur_hp = player_info.max_hp;
        player_status.max_hp = player_info.max_hp;
        player_status.is_init = false;
    }


    let index:usize = match  player_info.role {
        super::PlayerRole::TANK0 => 0,
        super::PlayerRole::TANK1 => 1,
        super::PlayerRole::TANK2 => 2,
        super::PlayerRole::TANK3 => 3,
    };
    commands.spawn(
        SpriteBundle{
            texture: player_image_assets.tanks.get(index).unwrap().clone(),
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
    .insert(SpriteSize::from(PLAYER_SIZE));
}


pub(crate) fn player_move_system(
    mut player_query: Query<(&mut Transform, &Velocity, &SpriteSize), With<Player>>,
    enemy_query: Query<(&Transform, &SpriteSize), (With<Enemy>, Without<Player>)>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
){
    
    if let Ok((mut player_tf, velocity, player_size)) =  player_query.get_single_mut() {                       
        
        //collsion detect
        let player_scale = Vec2::from(player_tf.scale.xy());
        let mut collison_flag: bool = false;

        let mut dst_pos = player_tf.translation.clone();
        dst_pos.x += velocity.x * TIME_STEP * BASE_SPEED;
        dst_pos.y += velocity.y * TIME_STEP * BASE_SPEED;

        
        for(enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());
            //add a magrin to 0.9
            let collison = collide(
                dst_pos,
                player_size.0 * player_scale * 0.9,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale * 0.9
            );

            if let Some(_) = collison {
                //collsion, player can not move
                collison_flag = true;
                break;
            }
        }

        if !collison_flag {
            let translation = &mut player_tf.translation;
            let player_size = player_size.0 / 2.0;
            if translation.x + velocity.x * TIME_STEP * BASE_SPEED + player_size.x >= X_DIRECTION_LIMIT {
                translation.x = X_DIRECTION_LIMIT - player_size.x;
            } else if translation.x + velocity.x * TIME_STEP * BASE_SPEED - player_size.x < -X_DIRECTION_LIMIT{
                translation.x = -X_DIRECTION_LIMIT + player_size.x;
            } else {
                translation.x += velocity.x * TIME_STEP * BASE_SPEED;
            }
    
    
            if translation.y + velocity.y * TIME_STEP * BASE_SPEED + player_size.y > Y_DIRECTION_LIMIT {
                translation.y = Y_DIRECTION_LIMIT - player_size.y;
            } else if translation.y + velocity.y * TIME_STEP * BASE_SPEED - player_size.y < -Y_DIRECTION_LIMIT {
                translation.y = -Y_DIRECTION_LIMIT + player_size.y;
            } else {
                translation.y += velocity.y * TIME_STEP * BASE_SPEED;
            }
    
            //for camera to location
            player_move_event.send(PlayerMoveEvent((translation.x, translation.y)));
        }

    }
   
}









// pub(crate) fn player_move_system(
//     mut player_query: Query<(&mut Transform, &Velocity, &SpriteSize), With<Player>>,
//     enemy_query: Query<(&Transform, &SpriteSize), (With<Enemy>, Without<Player>)>,
//     mut player_move_event: EventWriter<PlayerMoveEvent>,
// ){
//     let mut collison_flag: bool = false;

//     let judge_collison = || {
//         if let Ok((mut player_tf, velocity, player_size)) =  player_query.get_single_mut() {                       
//             let player_scale = Vec2::from(player_tf.scale.xy());
            
//             for(enemy_tf, enemy_size) in enemy_query.iter() {
//                 let enemy_scale = Vec2::from(enemy_tf.scale.xy());
//                 let collison = collide(
//                     player_tf.translation,
//                     player_size.0 * player_scale,
//                     enemy_tf.translation,
//                     enemy_size.0 * enemy_scale
//                 );
    
//                 if let Some(_) = collison {
//                     //collsion, player can not move
//                     collison_flag = true;
//                 } else {
    
//                 }
//             }
//         }
//     };

//     judge_collison();

//     if !collison_flag {
//         let translation = &mut player_tf.translation;
//         let player_size = player_size.0 / 2.0;
//         if translation.x + velocity.x * TIME_STEP * BASE_SPEED + player_size.x >= X_DIRECTION_LIMIT {
//             translation.x = X_DIRECTION_LIMIT - player_size.x;
//         } else if translation.x + velocity.x * TIME_STEP * BASE_SPEED - player_size.x < -X_DIRECTION_LIMIT{
//             translation.x = -X_DIRECTION_LIMIT + player_size.x;
//         } else {
//             translation.x += velocity.x * TIME_STEP * BASE_SPEED;
//         }


//         if translation.y + velocity.y * TIME_STEP * BASE_SPEED + player_size.y > Y_DIRECTION_LIMIT {
//             translation.y = Y_DIRECTION_LIMIT - player_size.y;
//         } else if translation.y + velocity.y * TIME_STEP * BASE_SPEED - player_size.y < -Y_DIRECTION_LIMIT {
//             translation.y = -Y_DIRECTION_LIMIT + player_size.y;
//         } else {
//             translation.y += velocity.y * TIME_STEP * BASE_SPEED;
//         }

//         //for camera to location
//         player_move_event.send(PlayerMoveEvent((translation.x, translation.y)));
//     }
   
// }

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
        velocity.x = if kb.pressed(KeyCode::A) {
            -1.0
        } else if kb.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };

        velocity.y = if kb.pressed(KeyCode::S) {
            -1.0
        } else if kb.pressed(KeyCode::W) {
            1.0
        } else {
            0.0
        }       
    }
}