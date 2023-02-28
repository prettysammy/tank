use bevy::prelude::*;

use crate::{camera::SceneCamera, 
            events::GameOverEvent
           };

use super::*;



pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::GameOver)
            .with_system(cleanup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::GameOver)
            .with_system(replay_game)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(to_game_over)
        );        
    }    
}


fn cleanup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    } 

    //game over scene
    commands
        .spawn(Camera2dBundle::default())
        .insert(SceneCamera);    

    let text_style = TextStyle {
        font: asset_server.load("fonts/hanti.ttf"),
        font_size: 120.0,
        color: Color::RED,
    };

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
                                value: "Play Again".to_string(),
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
            });
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Game Over".to_string(),
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

}

fn replay_game(
    mut commands: Commands,
    mut game_stage: ResMut<State<GameStage>>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>)>
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                game_stage.set(GameStage::Main).unwrap();
            },
            _=> (),
        }
    }
}

fn to_game_over(
    mut game_over_event: EventReader<GameOverEvent>,
    mut game_stage: ResMut<State<GameStage>>
) {
    for _ in game_over_event.iter() {
        println!("enter game over");
        game_stage.set(GameStage::GameOver).unwrap();
    }
}