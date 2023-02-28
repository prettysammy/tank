use bevy::prelude::*;
use crate::{GameStage, 
            events::PlayerMoveEvent, 
            };

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main)
            .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(camera_with_player_view_system)
        );        
    }
}


#[derive(Component)]
pub struct SceneCamera;

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default())
    .insert(SceneCamera);
}

fn camera_with_player_view_system(
    mut player_move_event: EventReader<PlayerMoveEvent>,
    mut camera_query: Query<&mut Transform, With<SceneCamera>>,
) {
    for PlayerMoveEvent(player_pos) in player_move_event.iter(){
        if let Ok(mut camera_tf) = camera_query.get_single_mut() {
            camera_tf.translation.x = player_pos.0;
            camera_tf.translation.y = player_pos.1;
        }
    }

}