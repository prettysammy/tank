mod events;
use events::EventsPlugin;

mod render;
use render::RenderPlugin;

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
pub use assets::{ PlayerImageAssets, BackgroundImageAssets, BulletImageAssets, EnemyImageAssets, UIImageAssets};

mod background;
use background::BackgroundPlugin;

mod panel;
use panel::PanelPlugin;


use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy::utils::HashMap;
use bevy_ninepatch::NinePatchPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;




#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum GameStage {
    Loading,
    Main,
    GameOver,
}

#[derive(Default, Resource)]
pub struct EelEntityMap(pub HashMap<u64, Entity>);


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameStage::Loading)
            .continue_to_state(GameStage::Main)
            .with_collection::<PlayerImageAssets>()
            .with_collection::<BackgroundImageAssets>()
            .with_collection::<BulletImageAssets>()
            .with_collection::<EnemyImageAssets>()
            .with_collection::<UIImageAssets>()
        )
        .add_state(GameStage::Loading)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                window: WindowDescriptor {
                    title: "Tank".to_string(),
                    width: 1280.,
                    height: 800.,
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MovePlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PanelPlugin);
    }
}

pub fn app() -> App{
    let mut app = App::new();

    app.add_plugin(GamePlugin);
    
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app
}
