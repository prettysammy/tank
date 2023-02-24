use bevy::prelude::*;

use crate::{GameStage, 
            event::PlayerMoveEvent, 
            components::{ Player, Bullet }, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT}, 
            SpriteSize,
            };

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

pub struct MovePlugin;
impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMoveEvent>();
        
        app.add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(player_move_system)
            .with_system(bullet_move_system)
        );
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 100.0;


// fn move_system(
//     mut query: Query<(&mut Transform, &RoleType, &Velocity)>,
//     mut player_move_event: EventWriter<PlayerMoveEvent>,
// ){
//     for(mut transform, role_type,velocity) in query.iter_mut(){
//         let translation = &mut transform.translation;
//         translation.x += velocity.x * TIME_STEP * BASE_SPEED;
//         translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    
//         match role_type {
//             RoleType::Player => {
//                 player_move_event.send(PlayerMoveEvent((translation.x, translation.y)));
//             },
//             _ => ()
//         }
//     }
// }

fn player_move_system(
    mut query: Query<(&mut Transform, &Velocity, &SpriteSize), With<Player>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
){
    for(mut transform, velocity, play_size) in query.iter_mut(){
        let translation = &mut transform.translation;
        let play_size = play_size.0 / 2.0;

        if translation.x + velocity.x * TIME_STEP * BASE_SPEED + play_size.x >= X_DIRECTION_LIMIT {
            translation.x = X_DIRECTION_LIMIT - play_size.x;
        } else if translation.x + velocity.x * TIME_STEP * BASE_SPEED - play_size.x < -X_DIRECTION_LIMIT{
            translation.x = -X_DIRECTION_LIMIT + play_size.x;
        } else {
            translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        }


        if translation.y + velocity.y * TIME_STEP * BASE_SPEED + play_size.y > Y_DIRECTION_LIMIT {
            translation.y = Y_DIRECTION_LIMIT - play_size.y;
        } else if translation.y + velocity.y * TIME_STEP * BASE_SPEED - play_size.y < -Y_DIRECTION_LIMIT {
            translation.y = -Y_DIRECTION_LIMIT + play_size.y;
        } else {
            translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        }

        player_move_event.send(PlayerMoveEvent((translation.x, translation.y)));

    }
}


fn bullet_move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity, &SpriteSize), With<Bullet>>,
){
    for(entity, mut transform, velocity, bullet_size) in query.iter_mut(){
        let translation = &mut transform.translation;
        let bullet_size = bullet_size.0 / 2.0;

        if translation.x + velocity.x * TIME_STEP * BASE_SPEED + bullet_size.x >= X_DIRECTION_LIMIT 
            || translation.x + velocity.x * TIME_STEP * BASE_SPEED - bullet_size.x < -X_DIRECTION_LIMIT
            || translation.y + velocity.y * TIME_STEP * BASE_SPEED + bullet_size.y > Y_DIRECTION_LIMIT 
            || translation.y + velocity.y * TIME_STEP * BASE_SPEED - bullet_size.y < -Y_DIRECTION_LIMIT 
        {
            commands.entity(entity).despawn();
        } else {
            translation.x += velocity.x * TIME_STEP * BASE_SPEED;
            translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        }


    }
}