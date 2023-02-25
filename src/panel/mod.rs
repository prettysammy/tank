mod main_panel;
use main_panel::MainPanelPlguin;

use bevy::prelude::*;

pub struct PanelPlugin;
impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainPanelPlguin);
    }
}