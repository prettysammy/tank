//plugins
mod events;
use events::EventsPlugin;

mod render;
use render::RenderPlugin;

mod player;
use player::PlayerPlugin;

mod enemy;
use enemy::EnemyPlugin;

mod bullet;
use bullet::BulletPlugin;

mod camera;
use camera::CameraPlugin;

mod background;
use background::BackgroundPlugin;

mod ui;
use ui::UiPlugin;

mod skills;
use skills::SkillsPlugin;


//pub functions
mod utils;


mod components;
pub use components::{ SpriteSize, Velocity};

mod game_stage;
pub use game_stage::{GameStage, GameStagePlugin};

mod assets;
pub use assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets, UIImageAssets, FontAssets};


//consts
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 100.0;


//lib functions
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ninepatch::NinePatchPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;



#[derive(Default, Resource)]
pub struct EelEntityMap(pub HashMap<u64, Entity>);


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(GameStagePlugin)

        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                window: WindowDescriptor {
                    title: "Tank  VS  Zombie".to_string(),
                    width: 1280.,
                    height: 800.,
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SkillsPlugin);
    }
}

pub fn app() -> App{
    let mut app = App::new();

    app.add_plugin(GamePlugin);
    
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app
}
