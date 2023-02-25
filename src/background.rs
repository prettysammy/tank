use bevy::{prelude::*, sprite::Anchor};

use crate::{ BackgroundImageAssets, GameStage };


const TILE_NUM_EVERY_X_REGION: u32 = 3;
const TILE_NUM_EVERY_Y_REGION: u32 = 2;
const TILE_SIZE: f32 = 256.0;
const START_POS: (f32, f32) = (- ((TILE_SIZE as u32* TILE_NUM_EVERY_X_REGION) as f32), (TILE_SIZE as u32 * TILE_NUM_EVERY_Y_REGION) as f32);

pub const X_DIRECTION_LIMIT: f32 = (TILE_SIZE as u32* TILE_NUM_EVERY_X_REGION) as f32;
pub const Y_DIRECTION_LIMIT: f32 = (TILE_SIZE as u32* TILE_NUM_EVERY_Y_REGION) as f32;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main).with_system(setup)
        );
    }
}



fn setup(
    mut commands: Commands,
    backgroun_image_assets: Res<BackgroundImageAssets>
) {
    //let backgroud_tile_size = SpriteSize::from((TILE_SIZE, TILE_SIZE));
    
    for x_direction in 0..TILE_NUM_EVERY_X_REGION * 2 {
        for y_direction in 0..TILE_NUM_EVERY_Y_REGION * 2 {
            let x_pos: f32 = START_POS.0 + x_direction as f32 * TILE_SIZE;
            let y_pos: f32 = START_POS.1 - y_direction as f32 * TILE_SIZE;

            commands.spawn(
                SpriteBundle {
                    texture: backgroun_image_assets.background0.clone_weak(),
                    transform: Transform {
                        translation: Vec3{x: x_pos, y: y_pos, z:0.0},
                        ..Default::default()
                    },
                    sprite: Sprite {
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );                
        }
    }


}