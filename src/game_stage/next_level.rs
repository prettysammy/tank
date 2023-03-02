use bevy::prelude::*;


use crate::{camera::SceneCamera
           };

use super::*;


pub struct NextLevelPlugin;
impl Plugin for NextLevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(count_down_to_next_level)
        )
        .add_system_set(
            SystemSet::on_enter(GameStage::NextLevel)
            .with_system(cleanup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::NextLevel)
            .with_system(next_level)
        );        
    }    
}


fn cleanup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_level: Res<GameLevel>,
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
                                value: "GO".to_string(),
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
                        value: format!("Next Level {}", game_level.level),
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

fn next_level(
    mut commands: Commands,
    mut game_stage: ResMut<State<GameStage>>,
    mut single_level_live_timer: ResMut<SingleLevelTimer>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>)>
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                
                //go to main ,resume the timer
                single_level_live_timer.0.unpause();               
                game_stage.set(GameStage::Main).unwrap();
            },
            _=> (),
        }
    }
}

pub(crate) fn count_down_to_next_level(
    mut single_level_live_timer: ResMut<SingleLevelTimer>,
    mut game_level: ResMut<GameLevel>,
    time: Res<Time>,
    mut game_stage: ResMut<State<GameStage>>
) {
    //println!("{}",single_level_live_timer.0.remaining_secs());
    
    //
    if single_level_live_timer.1 {
        if single_level_live_timer.0.tick(time.delta()).just_finished() {

            //Next level page should pasue the timer
            single_level_live_timer.0.pause();
    
            game_level.level += 1;
            game_stage.set(GameStage::NextLevel).unwrap();
        }
    }

}