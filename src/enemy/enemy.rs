use bevy::{prelude::*, sprite::collide_aabb::collide, math::Vec3Swizzles};
use rand::{thread_rng, Rng};


use crate::{EnemyImageAssets, SpriteSize, FontAssets, TIME_STEP, BASE_SPEED,
            components::{Enemy, Player}, 
            background::{X_DIRECTION_LIMIT, Y_DIRECTION_LIMIT},
            events::EnemyHitPlayerEvent, utils::cal_vector_by_two_points, 
        };

use super::{EnemyCount, ENEMY_MAX, ENEMY_SIZE, EnemyStatus, EnemyAttackTimer, EnemyHpBar, EnemySpawnTimer, ENEMY_SPEED, EnemyInfo, get_enemy_pool};

pub(crate) fn init_main_system(
    mut commands: Commands,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, 
) {
    commands.insert_resource(EnemyCount::default());

    //enable enemy_spawn_timer
    enemy_spawn_timer.1 = true;
    enemy_spawn_timer.0.unpause();
}

pub(crate) fn exit_main_system(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, 
) {
    //disable enemy_spawn_timer
    enemy_spawn_timer.1 = false;
    enemy_spawn_timer.0.pause();
}

pub(crate) fn enemy_spawn_system(
    mut commands: Commands,
    enemy_image_assets: Res<EnemyImageAssets>,
    font_assets: Res<FontAssets>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    //println!("enemy_spawn_system");
    if enemy_count.0 < ENEMY_MAX {  
        let hp_style = TextStyle {
            font: font_assets.bold.clone_weak(),
            font_size: 16.0,
            color: Color::BLACK,
        };
        let mut rng = thread_rng();

        let mut gen_enemy = || {           
            let x = rng.gen_range( -(X_DIRECTION_LIMIT - ENEMY_SIZE.0 / 2.0)..(X_DIRECTION_LIMIT - ENEMY_SIZE.0 / 2.0) );
            let y = rng.gen_range( -(Y_DIRECTION_LIMIT - ENEMY_SIZE.1 / 2.0)..(Y_DIRECTION_LIMIT - ENEMY_SIZE.1 / 2.0));
    
            let enemy_pool =  get_enemy_pool().0;
            let enemy_info: &EnemyInfo = enemy_pool.get(rng.gen_range(0..2)).unwrap();
            let enemy_atlas = match &enemy_info.name as &str{
                "zombie" => enemy_image_assets.zombie.clone_weak(),
                "skeleton" => enemy_image_assets.skeleton.clone_weak(),
                _ => enemy_image_assets.zombie.clone_weak(),
            };
            let enemy_status = EnemyStatus::from(enemy_info);            
            
            let mut entity_command = 
            commands.spawn(
                SpriteSheetBundle {
                    texture_atlas: enemy_atlas,
                    transform: Transform{
                        translation:Vec3::new(x, y, 10.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        custom_size: Some(SpriteSize::from(ENEMY_SIZE).0),
                        ..Default::default()
                    },                
                    ..Default::default()
                }       
            );
                    
            entity_command
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE))
            .insert(enemy_status)
            .insert(EnemyAttackTimer::default());
            
            let entity = entity_command.id();
    
            entity_command
                .with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        transform: Transform{
                            translation:Vec3::new(0.0,40.0, 9.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(60.0, 20.0)),
                            color: Color::RED,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(EnemyHpBar(entity));
                })
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        transform: Transform{
                            translation:Vec3::new(0.0,40.0, 10.0),
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: "HP".to_string(),
                                style: hp_style.clone(),
                                ..Default::default()
                            }],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(EnemyHpBar(entity));
                }); 
      
            enemy_count.0 += 1;
        };

        gen_enemy();
    }
}

pub(crate) fn enemy_move_system (
    mut enemy_query: Query<(&mut Transform, &mut EnemyAttackTimer, &EnemyStatus, &SpriteSize), With<Enemy>>,
    player_query: Query<(&Transform, &SpriteSize), (With<Player>, Without<Enemy>)>,
    mut enemy_hit_player_event: EventWriter<EnemyHitPlayerEvent>,
    time: Res<Time>
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());
        for (mut enemy_tf, mut enemy_attcak_timer, enemy_status, enemy_size) in enemy_query.iter_mut() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());
            
            let collison = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collison {               
                if enemy_attcak_timer.0.tick(time.delta()).just_finished() {
                    enemy_hit_player_event.send(EnemyHitPlayerEvent(enemy_status.atk))
                }                
            } else {
                let src = enemy_tf.translation.truncate();
                let dst = player_tf.translation.truncate();
                let vector = cal_vector_by_two_points(src, dst) * ENEMY_SPEED;

                enemy_tf.translation.x +=  vector.x * TIME_STEP * BASE_SPEED;
                enemy_tf.translation.y +=  vector.y * TIME_STEP * BASE_SPEED ;
            }

        }
    }
}


pub(crate) fn update_enemy_status_system(
    mut text_query: Query<(&mut Text, &EnemyHpBar)>,
    mut hp_bar_query: Query<(&mut Sprite, &EnemyHpBar)>,
    enemy_status_query: Query<(Entity, &EnemyStatus)>
) {
    for (mut text, EnemyHpBar(enemy_entity)) in text_query.iter_mut() {
        for (entity, enemy_status) in enemy_status_query.iter(){
            if *enemy_entity == entity {
                text.sections[0].value = format!("{}/{}", enemy_status.cur_hp, enemy_status.max_hp);
            }
            
        }        
    }
    for (mut sprite, EnemyHpBar(enemy_entity)) in hp_bar_query.iter_mut() {
        for (entity, enemy_status) in enemy_status_query.iter(){
            let hp_scale = (enemy_status.cur_hp as f32 / enemy_status.max_hp as f32) as f32 * 60.0;
            if *enemy_entity == entity {
                sprite.custom_size = Some(Vec2::new(hp_scale, 20.0));
            }
            
        }        
    }

}

pub(crate) fn update_enemy_texture_atlas_system(
    mut query: Query<&mut TextureAtlasSprite, With<Enemy>>    
) {
    for mut sprite in query.iter_mut() { 
        sprite.index =  (sprite.index +1) % 3;
    }   
}
