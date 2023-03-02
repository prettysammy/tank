use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

use crate::{assets::UIImageAssets,  
            player::{PlayerStatusType, PlayerStatus}, 
            GameStage,
            game_stage::{SingleLevelTimer, GameLevel},
           };


pub struct StatusUilPlguin;

impl Plugin for StatusUilPlguin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameStage::Main)
            .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(update_count_down_time)
        );    
    }
}

fn setup(
    mut commands: Commands,
    player_status: Res<PlayerStatus>,
    asset_server: Res<AssetServer>,
    ui_image_assets: Res<UIImageAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,    
) {

    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(10, 10, 20, 20));

    let main_entity = commands
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        //HP 
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                            margin: UiRect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image_assets.icon_hp.clone_weak()),
                        ..default()
                    });

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(24.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(24.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::RED.into(),
                                        ..default()
                                    },
                                    value: format!("{}/{}", player_status.cur_hp, player_status.max_hp),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::HP);
                });                          
        })

        //ATK
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                            margin: UiRect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image_assets.icon_atk.clone_weak()),
                        ..default()
                    });

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(24.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(24.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::BLUE.into(),
                                        ..default()
                                    },
                                    value: format!("{}", player_status.atk),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::ATK);
                });                         
        })

        //DEF
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                            margin: UiRect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image_assets.icon_def.clone_weak()),
                        ..default()
                    });

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(24.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(24.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::BLUE.into(),
                                        ..default()
                                    },
                                    value: format!("{}", player_status.def),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::DEF);
                });                         
        })


        //GOLD
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                            margin: UiRect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image_assets.icon_gold.clone_weak()),
                        ..default()
                    });

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(24.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(24.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::GOLD.into(),
                                        ..default()
                                    },
                                    value: format!("{}", player_status.gold),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::GOLD);
                });                          
        })

        //timer
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(32.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(32.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::BLACK.into(),
                                        ..default()
                                    },
                                    value: format!("left 100s"),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CountDownTimeMark);
                });                          
        })       


        .id();

        commands.spawn(NinePatchBundle {
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(180.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(5.0),
                    top: Val::Px(5.0),
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


#[derive(Component)]
struct CountDownTimeMark;

fn update_count_down_time(
    single_level_live_timer: Res<SingleLevelTimer>,
    game_level: Res<GameLevel>,
    mut query: Query<&mut Text, With<CountDownTimeMark>>   
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Level {}\r\nTime left {:.0}s",
                                        game_level.level, single_level_live_timer.0.remaining_secs()
                                        //(single_level_live_timer.0.remaining_secs() as u32) + 1
                                        );
    }
}