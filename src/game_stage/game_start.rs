use bevy::{prelude::*, ui::FocusPolicy};

use crate::{Visibility,
            camera::SceneCamera,
            player::{PlayerRole, PlayerInfo, get_player_pool}
           };

use super::*;

#[derive(Component)]
struct StartPage;

#[derive(Component)]
struct PreparePage;

#[derive(Component)]
struct ChooseFrame;


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
            .with_system(choose_player)
            .with_system(start_game)
        );    
    }    
}

fn start_page_setup(
    mut commands: Commands,
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
                });                    
            });
        });    
          
}

fn prepare_page_setup(
    mut commands: Commands,
    ui_image_assets: Res<UIImageAssets>,
    player_image_assets: Res<PlayerImageAssets>,
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
 
            let text_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 60.0,
                color: Color::BLUE,
            };
            commands
                .spawn(NodeBundle {
                    style: Style {
                        // justify_content: JustifyContent::Center,
                        // align_content: AlignContent::Center,
                        // align_items: AlignItems::Center,
                        // flex_direction: FlexDirection::Column,
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
                        image: UiImage(ui_image_assets.prepare_page.clone_weak()),
                        ..default()
                    })


                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            style : Style { 
                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(35.0),
                                                right: Val::Percent(35.0),
                                                top: Val::Percent(0.0),
                                                bottom:Val::Percent(90.0),
                                                 },                                                              
                                ..Default::default()
                            }, 
                            text: Text {
                                sections: vec![TextSection {
                                    value: format!("choose your tank"),
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
                    })

                    //tank0
                    .with_children(|parent| {
                        parent.spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(0.0),
                                                right: Val::Percent(75.0),
                                                top: Val::Percent(10.0),
                                                bottom:Val::Percent(60.0),
                                                },

                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {                           
                            parent.spawn(ButtonBundle {
                                image: UiImage::from(player_image_assets.tanks.get(0).unwrap().clone_weak()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Percent(0.0)),
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PlayerRole::TANK0)
                        
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    focus_policy: FocusPolicy::Pass,
                                    image: UiImage::from(ui_image_assets.choose_frame.clone_weak()),
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(0.0)),
                                        size: Size {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                        },
                                        ..Default::default()
                                    },
                                    //visibility: Visibility { is_visible: false },
                                    ..Default::default()
                                })
                                .insert(PlayerRole::TANK0)
                                .insert(ChooseFrame);
                            });                                                
                        });
                    })


                    //tank1
                    .with_children(|parent| {
                        parent.spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(25.0),
                                                right: Val::Percent(50.0),
                                                top: Val::Percent(10.0),
                                                bottom:Val::Percent(60.0),
                                                },

                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {                           
                            parent.spawn(ButtonBundle {
                                image: UiImage::from(player_image_assets.tanks.get(1).unwrap().clone_weak()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Percent(0.0)),
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PlayerRole::TANK1)
                        
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    focus_policy: FocusPolicy::Pass,
                                    image: UiImage::from(ui_image_assets.choose_frame.clone_weak()),
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(0.0)),
                                        size: Size {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                        },
                                        ..Default::default()
                                    },
                                    visibility: Visibility { is_visible: false },
                                    ..Default::default()
                                })
                                .insert(PlayerRole::TANK1)
                                .insert(ChooseFrame);
                            });                                                
                        });
                    })

                    //tank2
                    .with_children(|parent| {
                        parent.spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(50.0),
                                                right: Val::Percent(25.0),
                                                top: Val::Percent(10.0),
                                                bottom:Val::Percent(60.0),
                                                },

                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {                           
                            parent.spawn(ButtonBundle {
                                image: UiImage::from(player_image_assets.tanks.get(2).unwrap().clone_weak()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Percent(0.0)),
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PlayerRole::TANK2)
                        
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    focus_policy: FocusPolicy::Pass,
                                    image: UiImage::from(ui_image_assets.choose_frame.clone_weak()),
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(0.0)),
                                        size: Size {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                        },
                                        ..Default::default()
                                    },
                                    visibility: Visibility { is_visible: false },
                                    ..Default::default()
                                })
                                .insert(PlayerRole::TANK2)
                                .insert(ChooseFrame);
                            });                                                
                        });
                    })

                    //tank3
                    .with_children(|parent| {
                        parent.spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(75.0),
                                                right: Val::Percent(0.0),
                                                top: Val::Percent(10.0),
                                                bottom:Val::Percent(60.0),
                                                },

                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {                           
                            parent.spawn(ButtonBundle {
                                image: UiImage::from(player_image_assets.tanks.get(3).unwrap().clone_weak()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Percent(0.0)),
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PlayerRole::TANK3)
                        
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    focus_policy: FocusPolicy::Pass,
                                    image: UiImage::from(ui_image_assets.choose_frame.clone_weak()),
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(0.0)),
                                        size: Size {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                        },
                                        ..Default::default()
                                    },
                                    visibility: Visibility { is_visible: false },
                                    ..Default::default()
                                })
                                .insert(PlayerRole::TANK3)
                                .insert(ChooseFrame);
                            });                                                
                        });
                    })


                    //button
                    .with_children(|parent| {
                        parent.spawn(ButtonBundle {
                            image: UiImage::from(ui_image_assets.button0.clone_weak()),
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect { 
                                                left: Val::Percent(80.0),
                                                right: Val::Percent(1.0),
                                                top: Val::Percent(90.0),
                                                bottom:Val::Percent(1.0),
                                                },
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(PreparePage)
                        
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
                        });                    
                    });

                        
                });
            },
            _=> (),
        }
    }
}

fn choose_player(
    //mut commands: Commands,
    //ui_image_assets: Res<UIImageAssets>,
    mut player_info: ResMut<PlayerInfo>,
    interaction_query: Query<(&PlayerRole, &Interaction), (With<Interaction>, With<Button>)>,
    mut choose_frame_query: Query<(&mut Visibility, &PlayerRole), With<ChooseFrame>>     
) {   
    for (button_role,interaction) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                //println!("{:?}click", button_entity);
                
                for (mut visible, frame_role) in choose_frame_query.iter_mut() {
                    if button_role == frame_role {
                        visible.is_visible = true;
                        //player_info.role = button_role.clone();

                        let player_pool = get_player_pool();
                        for player in player_pool.0.iter() {
                            if player.role == *button_role {
                                player_info.role = player.role.clone();
                                player_info.atk = player.atk;
                                player_info.def = player.def;
                                player_info.max_hp = player.max_hp;
                                player_info.name = player.name.clone();

                            }
                        }
                        

                    }
                    else {
                        visible.is_visible = false;
                    }
                }
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
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>, With<PreparePage>)>
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