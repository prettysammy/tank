use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;


#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1)) 
    }
}