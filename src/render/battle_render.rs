use bevy::prelude::*;

use crate::{GameStage, EnemyImageAssets, SpriteSize,
            events::PlayerHitEnemyEvent, 
            enemy::ENEMY_SIZE, 
           };

pub struct BattleRenderPlugin;
impl Plugin for BattleRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(render_player_hit_enemy)
        );
    }
}

fn render_player_hit_enemy(
    mut commands: Commands,
    mut player_hit_enemy_event: EventReader<PlayerHitEnemyEvent>,
    enemy_image_assets: Res<EnemyImageAssets>
) {
    for PlayerHitEnemyEvent(enemy_pos) in player_hit_enemy_event.iter() {
        commands.spawn(SpriteBundle {
            texture: enemy_image_assets.tomb.clone_weak(),
            transform: Transform {
                translation: Vec3::new(enemy_pos.0, enemy_pos.1, 5.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(SpriteSize::from(ENEMY_SIZE).0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
