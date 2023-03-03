use bevy::{prelude::*, ui::FocusPolicy};

use crate::{camera::SceneCamera};

use super::*;

#[derive(Component)]
struct StartPage;

#[derive(Component)]
struct PreparetPage;


pub struct GameStartPlugin;
impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::GameStart)
            .with_system(start_page_setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::GameStart)
            .with_system(prepare_page_setup)
            .with_system(start_game)
        );    
    }    
}

fn start_page_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_image_assets: Res<UIImageAssets>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(SceneCamera);    

    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })

        //background picture
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    margin: UiRect {
                        ..default()
                    },
                    ..default()
                },
                image: UiImage(ui_image_assets.start_page.clone_weak()),
                ..default()
            })
            
            //Title
            .with_children(|parent| { 
                parent.spawn(ImageBundle {
                    style : Style { 
                        position_type: PositionType::Absolute,
                        position: UiRect { 
                                        left: Val::Percent(20.0),
                                        right: Val::Percent(20.0),
                                        top: Val::Percent(15.0),
                                        bottom:Val::Percent(50.0),
                                         },                                                              
                        ..Default::default()
                    }, 
                    image: UiImage(ui_image_assets.title.clone_weak()),                 
                    ..Default::default()
                });                       
            })

            //button
            .with_children(|parent| {
                parent.spawn(ButtonBundle {
                    image: UiImage::from(ui_image_assets.button0.clone_weak()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect { 
                                        left: Val::Percent(35.0),
                                        right: Val::Percent(35.0),
                                        top: Val::Percent(60.0),
                                        bottom:Val::Percent(30.0),
                                         },
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(StartPage)
                
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        focus_policy: FocusPolicy::Pass,
                        style: Style {
                            margin: UiRect {
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image_assets.text_play.clone_weak()),
                        ..default()
                    });
                })
                .insert(StartPage);                    
            });
        });    
          
}

fn prepare_page_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<StartPage>)>
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
            
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }                        

            commands
            .spawn(Camera2dBundle::default())
            .insert(SceneCamera); 

            let button_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 60.0,
                color: Color::RED,
            };  
            let text_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 120.0,
                color: Color::RED,
            };
            commands
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(20.0)),
                            ..Default::default()
                        },
                        background_color: Color::GREEN.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            style : Style {
                                ..Default::default()
                            },
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "START GAME".to_string(),
                                        style: button_style.clone(),
                                        ..Default::default()
                                    }
                                ],
                                alignment: TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,                            
                                },
                                ..Default::default()
                            },                    
                            ..Default::default()
                        });
                    })
                    .insert(PreparetPage);
        
                    parent.spawn(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("准备界面"),
                                style: text_style.clone(),
                                ..Default::default()
                            }],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });            
        
                }); 

            },
            _=> (),
        }
    }
}


fn start_game(
    mut commands: Commands,
    mut game_stage: ResMut<State<GameStage>>,
    mut single_level_live_timer: ResMut<SingleLevelTimer>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>, With<PreparetPage>)>
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }

                //enable the timer and reset it
                single_level_live_timer.1 = true;
                single_level_live_timer.0.reset();

                game_stage.set(GameStage::Main).unwrap();
            },
            _=> (),
        }
    }
}