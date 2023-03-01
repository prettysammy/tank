use bevy::prelude::*;

use crate::{GameStage, EnemyImageAssets, SpriteSize,
            events::PlayerHitEnemyEvent, 
            enemy::{ENEMY_TOMB_SIZE, EnemyStatus, EnemyCount, EnemyTombTimer}, 
            player::PlayerStatus, 
           };

pub struct BattleRenderPlugin;
impl Plugin for BattleRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(render_bullet_hit_enemy)
            .with_system(despawn_enemy_tomb_system)
        );
    }
}

fn render_bullet_hit_enemy(
    mut commands: Commands,
    mut query: Query<&mut EnemyStatus, &Transform>,
    mut player_hit_enemy_event: EventReader<PlayerHitEnemyEvent>,
    mut player_status: ResMut<PlayerStatus>,
    mut enemy_count: ResMut<EnemyCount>,
    enemy_image_assets: Res<EnemyImageAssets>
) {
    for PlayerHitEnemyEvent(enemy_entity) in player_hit_enemy_event.iter() {
        //println!("enemy {:?} is hitted", enemy_entity);
        if let Ok(mut enemy_status) = query.get_component_mut::<EnemyStatus>(*enemy_entity) {
            enemy_status.cur_hp -= player_status.atk - enemy_status.def;
            //println!("enemy {:?} hp is {}", enemy_entity, enemy_status.cur_hp);
            if enemy_status.cur_hp <= 0 {
                
                player_status.gold += enemy_status.gold.to_i64();

                if let Ok(enemy_tf) = query.get_component::<Transform>(*enemy_entity) {
                    //println!("should despawn enemy {:?}", enemy_entity);
                    commands.entity(*enemy_entity).despawn_recursive();
                    enemy_count.0 -= 1;                    
                    
                    commands.spawn(SpriteBundle {
                        texture: enemy_image_assets.tomb.clone_weak(),
                        transform: Transform {
                            translation: Vec3::new(enemy_tf.translation.x, enemy_tf.translation.y, 10.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            custom_size: Some(SpriteSize::from(ENEMY_TOMB_SIZE).0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(EnemyTombTimer::default());
                }
            }
        }
    }
}

fn despawn_enemy_tomb_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut EnemyTombTimer)>
) {
    for (entity, mut enemy_tomb_timer) in query.iter_mut() {
        if enemy_tomb_timer.0.tick(time.delta()).just_finished(){
            commands.entity(entity).despawn();
        }
    }
}
