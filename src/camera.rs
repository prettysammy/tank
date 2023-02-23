use bevy::prelude::*;
use crate::GameStage;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main).with_system(setup)
        );
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

