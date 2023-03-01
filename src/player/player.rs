use bevy::{prelude::*};

use crate::{ SpriteSize, PlayerImageAssets, TIME_STEP, BASE_SPEED,          
            components::{ Player, Velocity},
            events::PlayerMoveEvent, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT},  
            };

use super::{PlayerStatus, PlayerStatusType, PLAYER_SIZE};


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
    .insert(SpriteSize::from(PLAYER_SIZE));
}

pub(crate) fn player_move_system(
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

        //for camera to location
        player_move_event.send(PlayerMoveEvent((translation.x, translation.y)));

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