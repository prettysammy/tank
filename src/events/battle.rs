use bevy::{prelude::*, math::Vec3Swizzles, sprite::collide_aabb::collide};

use crate::{GameStage,
            player::PlayerStatus, 
           };

use super::{EnemyHitPlayerEvent, GameOverEvent};


pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(enemy_hit_player_system)
        );
    }
}

fn enemy_hit_player_system (
    mut player_status: ResMut<PlayerStatus>,
    mut enemy_hit_player_event: EventReader<EnemyHitPlayerEvent>,
    mut game_over_event: EventWriter<GameOverEvent>
) {
    for EnemyHitPlayerEvent(enemy_atk) in enemy_hit_player_event.iter() {
        player_status.cur_hp -= (*enemy_atk - player_status.def).max(0);

        if player_status.cur_hp <= 0 {
            //  match player_status.alive {
            //     crate::player::AliveStatus::ALIVE => {
            //         player_status.alive = crate::player::AliveStatus::DIE;
            //         game_over_event.send(GameOverEvent);
            //     },
            //     crate::player::AliveStatus::DIE => {},
            // };
            game_over_event.send(GameOverEvent);
        }
    }
}