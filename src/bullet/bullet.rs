use bevy::{prelude::*, utils::HashSet, sprite::collide_aabb::collide, math::Vec3Swizzles};

use crate::{TIME_STEP, BASE_SPEED, Velocity, SpriteSize, BulletImageAssets, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT},
            events::PlayerHitEnemyEvent,
            components::{Bullet, Enemy, Player}
           };

use super::{PLAY_BULLET_SPEED, BULLET_SIZE, BulletSpawnTimer};


pub(crate) fn init_main_system(
    mut bullet_spawn_timer: ResMut<BulletSpawnTimer>, 
) {
    bullet_spawn_timer.0.unpause();
}

pub(crate) fn exit_main_system(
    mut bullet_spawn_timer: ResMut<BulletSpawnTimer>, 
) {
    bullet_spawn_timer.0.pause();
}


pub(crate) fn bullet_spawn_system(
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



pub(crate) fn bullet_move_system(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform, &Velocity, &SpriteSize), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &SpriteSize), (With<Enemy>, Without<Bullet>)>,
    mut play_hit_enemy_event: EventWriter<PlayerHitEnemyEvent>
){
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    
    for(bullet_entity, mut bullet_tf, velocity, bullet_size) in bullet_query.iter_mut(){
        if despawned_entities.contains(&bullet_entity) {
            continue;
        }        
                
        let bullet_scale = Vec2::from(bullet_tf.scale.xy());
        for(enemy_entity, enemy_tf, enemy_size) in enemy_query.iter_mut(){
            if despawned_entities.contains(&bullet_entity) {
                continue;
            }  

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collison = collide(
                bullet_tf.translation,
                bullet_size.0 * bullet_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collison {
                despawned_entities.insert(bullet_entity);
                commands.entity(bullet_entity).despawn();
                play_hit_enemy_event.send(PlayerHitEnemyEvent(enemy_entity));                
            } else {
                let translation = &mut bullet_tf.translation;
                let bullet_size = bullet_size.0 / 2.0;

                if translation.x + velocity.x * TIME_STEP * BASE_SPEED + bullet_size.x >= X_DIRECTION_LIMIT 
                    || translation.x + velocity.x * TIME_STEP * BASE_SPEED - bullet_size.x < -X_DIRECTION_LIMIT
                    || translation.y + velocity.y * TIME_STEP * BASE_SPEED + bullet_size.y > Y_DIRECTION_LIMIT 
                    || translation.y + velocity.y * TIME_STEP * BASE_SPEED - bullet_size.y < -Y_DIRECTION_LIMIT 
                {
                    despawned_entities.insert(bullet_entity);
                    commands.entity(bullet_entity).despawn();
                } else {
                    translation.x += velocity.x * TIME_STEP * BASE_SPEED;
                    translation.y += velocity.y * TIME_STEP * BASE_SPEED;
                }                
            }            
       
        }

    }
}