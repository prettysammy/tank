mod battle_render;


use bevy::prelude::*;

use self::battle_render::BattleRenderPlugin;


pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BattleRenderPlugin);
    }
}