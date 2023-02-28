mod status_ui;
use status_ui::StatusUilPlguin;

use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StatusUilPlguin);
    }
}