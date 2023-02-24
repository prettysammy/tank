mod event;


mod player;
use player::PlayerPlugin;

mod enemy;
use enemy::EnemyPlugin;

mod utils;
pub use utils::{ velocity, marks };
use utils::velocity::MovePlugin;


mod components;
pub use components::{ SpriteSize };

mod camera;
use camera::CameraPlugin;

mod assets;
pub use assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets};

mod background;
use background::BackgroundPlugin;




use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub struct GamePlugin;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum GameStage {
    Loading,
    Main,
    GameOver,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameStage::Loading)
            .continue_to_state(GameStage::Main)
            .with_collection::<PlayerImageAssets>()
            .with_collection::<BackgroundImageAssets>()
            .with_collection::<BulletImageAssets>()
            .with_collection::<EnemyImageAssets>()
        )
        .add_state(GameStage::Loading)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                window: WindowDescriptor {
                    title: "Tank".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MovePlugin);
    }
}

pub fn app() -> App{
    let mut app = App::new();

    app.add_plugin(GamePlugin);
    
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app
}
