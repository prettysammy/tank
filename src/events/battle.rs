use bevy::{prelude::*, math::Vec3Swizzles, sprite::collide_aabb::collide, utils::HashSet};

use crate::{SpriteSize,  GameStage, 
            components::{Bullet, Enemy}, 
            enemy::EnemyCount
           };

use super::PlayerHitEnemyEvent;


pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(player_bullet_hit_enemy_system)
        );
    }
}

fn player_bullet_hit_enemy_system(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &SpriteSize), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    mut play_hit_enemy_event: EventWriter<PlayerHitEnemyEvent> 
) {
    
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (bullet_entity, bullet_tf, bullet_size) in bullet_query.iter() {
        if despawned_entities.contains(&bullet_entity) {
            continue;
        }
        let bullet_scale = Vec2::from(bullet_tf.scale.xy());

        for(enemy_entity, enemy_tf, enemy_size) in enemy_query.iter(){
            if despawned_entities.contains(&enemy_entity) {
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
                despawned_entities.insert(enemy_entity);

                //println!("send hit {:?},{:?}", bullet_entity, enemy_entity);
                //commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();

                enemy_count.0 -= 1;

                play_hit_enemy_event.send(PlayerHitEnemyEvent((enemy_tf.translation.x, enemy_tf.translation.y)));
            }
        
        }
    }
}