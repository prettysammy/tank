mod player;
use player::PlayerPlugin;


mod components;
pub use components::{ Velocity, SpriteSize };

mod camera;
pub use camera::CameraPlugin;

mod assets;
pub use assets::PlayerImageAssets;






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
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin);
    }
}

pub fn app() -> App{
    let mut app = App::new();

    app.add_plugin(GamePlugin);
    
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app
}
