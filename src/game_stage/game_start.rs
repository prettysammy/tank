use bevy::prelude::*;

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
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(SceneCamera);    

    let button_style = TextStyle {
        font: asset_server.load("fonts/hanti.ttf"),
        font_size: 60.0,
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
            .insert(StartPage);
        });    

}

fn prepare_page_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>, With<StartPage>)>
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