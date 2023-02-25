mod battle;

use bevy::prelude::*;
use self::battle::BattlePlugin;



//position (x,y)
pub struct PlayerMoveEvent(pub (f32, f32));

//position (x,y)
pub struct PlayerHitEnemyEvent(pub (f32, f32));

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEnemyEvent>();
        app.add_plugin(BattlePlugin);
    }
}