use bevy::{prelude::*, sprite::collide_aabb::collide, math::Vec3Swizzles};

use crate::{GameStage, 
            events::PlayerMoveEvent, 
            components::{ Player, Bullet, Enemy }, 
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
            .with_system(enemy_move_system)
        );
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 100.0;


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

fn enemy_move_system (
    mut enemy_query: Query<(&mut Transform , &SpriteSize), With<Enemy>>,
    player_query: Query<(&Transform, &SpriteSize), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());
        for (mut enemy_tf, enemy_size) in enemy_query.iter_mut() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());
            let x_direcion = player_tf.translation.x - enemy_tf.translation.x;
            let y_direcion = player_tf.translation.y - enemy_tf.translation.y;

            let bevel = (x_direcion * x_direcion + y_direcion * y_direcion).sqrt();

            let collison = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collison {

            } else {
                enemy_tf.translation.x +=  x_direcion / bevel * TIME_STEP * BASE_SPEED;
                enemy_tf.translation.y +=  y_direcion / bevel * TIME_STEP * BASE_SPEED;
            }

        }
    }
}