use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

// #[derive(Component)]
// pub struct Visibility{
//     pub is_visible: bool,
// }
// impl Default for Visibility {
//     fn default() -> Self {
//         Self { is_visible: false }
//     }
// }

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1)) 
    }
}