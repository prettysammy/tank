use bevy::prelude::*;
use crate::{ SpriteSize, PlayerImageAssets, GameStage };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main).with_system(setup)
        );
    }
}


const PLAYER_SIZE: (f32, f32) = (50.0, 60.0); 

fn setup(
    mut commands: Commands,
    player_image_assets: Res<PlayerImageAssets>
) {
    commands.spawn(
        SpriteBundle{
            texture: player_image_assets.tank0.clone(),
            sprite: Sprite {
                custom_size: Some(SpriteSize::from(PLAYER_SIZE).0),
                ..Default::default()
            },
            ..Default::default()
        }
    );

}
