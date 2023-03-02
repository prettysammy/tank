mod battle;

use bevy::prelude::*;
use self::battle::BattlePlugin;



//position (x,y)
pub struct PlayerMoveEvent(pub (f32, f32));

//position (x,y)
pub struct PlayerHitEnemyEvent(pub Entity);

//enemy_atk
pub struct EnemyHitPlayerEvent(pub i64);

pub struct GameOverEvent;


pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerMoveEvent>()
        .add_event::<PlayerHitEnemyEvent>()
        .add_event::<EnemyHitPlayerEvent>()
        .add_event::<GameOverEvent>()
        .add_plugin(BattlePlugin);
    }
}