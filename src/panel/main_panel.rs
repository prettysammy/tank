use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

use crate::{assets::UIImageAssets, GameStage};


pub struct MainPanelPlguin;

impl Plugin for MainPanelPlguin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Main).with_system(setup)
        );    
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_image_assets: Res<UIImageAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,    
) {

    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(10, 10, 20, 20));

    let main_entity = commands
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(5.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    // parent.spawn(ImageBundle {
                    //     style: Style {
                    //         size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                    //         margin: UiRect {
                    //             right: Val::Px(4.0),
                    //             ..default()
                    //         },
                    //         ..default()
                    //     },
                    //     image: UiImage(ui_image.icon_hp.clone_weak()),
                    //     ..default()
                    // });
                    // hp
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(16.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::GREEN.into(),
                                        ..default()
                                    },
                                    value: "HP: 100/100".to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        });
                });                  
        
        })
        .id();

        commands.spawn(NinePatchBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(160.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            nine_patch_data: NinePatchData::with_single_content(
                ui_image_assets.main_panel.clone_weak(),
                nine_patch_handle,
                main_entity,
            ),
            ..Default::default()
        }); 
       
}