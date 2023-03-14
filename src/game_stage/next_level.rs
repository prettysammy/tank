use bevy::{prelude::*, ui::FocusPolicy};


use crate::{
            camera::SceneCamera,
            skills::{PassiveSkills, PassiveSkill, ValueChange},
            player::{PlayerStatusType, PlayerStatus}
           };

use super::*;

struct ShopItemAttribute {
    position: UiRect,
    shop_item: ShopItems, 
}



#[derive(Component, Debug)]
struct PassiveSkillComponent{
    attribute: PlayerStatusType,
    cost: usize,
    value_change: Option<ValueChange>,    
}

impl From<&PassiveSkill> for PassiveSkillComponent{
    fn from(passive_skill: &PassiveSkill) -> Self {
        PassiveSkillComponent {
            attribute: passive_skill.attribute,
            cost: passive_skill.cost,
            value_change: passive_skill.value_change,
        }
    }
}


#[derive(Component)]
struct ShopArea;

#[derive(Component)]
struct ItemArea;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum ShopItems {
    ITEM1,
    ITEM2,
    ITEM3,
    ITEM4,
    ITEM5,
}

#[derive(Component)]
struct NextLevelButton;

#[derive(Component)]
struct BuyItemButton;

pub struct NextLevelPlugin;
impl Plugin for NextLevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(GameStage::Main)
            .with_system(timer_count_down_to_next_level)
        )
        .add_system_set(
            SystemSet::on_enter(GameStage::NextLevel)
            .with_system(next_level_page)
        )
        .add_system_set(
            SystemSet::on_update(GameStage::NextLevel)
            .with_system(to_next_level)
            .with_system(buy_item_system)
        );        
    }    
}


fn next_level_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_level: Res<GameLevel>,
    ui_image_assets: Res<UIImageAssets>,
    passive_skills: Res<PassiveSkills>,
    query: Query<Entity>
) {
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

    let item_text_style = TextStyle {
        font: asset_server.load("fonts/hanti.ttf"),
        font_size: 20.0,
        color: Color::NAVY,
    };

    //spawn shop item course 
    let spawn_shop_item = 
        |parent:&mut ChildBuilder, item_attribute: ShopItemAttribute, passive_skill: &PassiveSkill| -> () 
    {
        parent.spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                position: item_attribute.position,

                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(item_attribute.shop_item)
        .insert(ItemArea)

        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::from(asset_server.load(passive_skill.image_label.clone()).clone_weak()),
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect { 
                                    left: Val::Percent(25.0),
                                    right: Val::Percent(25.0),
                                    top: Val::Percent(0.0),
                                    bottom:Val::Percent(50.0),
                                    },
                    ..Default::default()
                },
                ..Default::default()
            });
            //.insert(item_attribute.shop_item);                                            
        }) 
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    position: UiRect { 
                                        left: Val::Percent(10.0),
                                        right: Val::Percent(10.0),
                                        top: Val::Percent(50.0),
                                        bottom:Val::Percent(15.0),
                                    },

                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {                                                
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{}\r\ncost {} gold", passive_skill.describe, passive_skill.cost),
                            style: item_text_style.clone(),
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
                //.insert(item_attribute.shop_item);
            });
        })                                  
        .with_children(|parent| {                           
            parent.spawn(ButtonBundle {
                image: UiImage::from(ui_image_assets.button0.clone_weak()),
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect { 
                                    left: Val::Percent(30.0),
                                    right: Val::Percent(30.0),
                                    top: Val::Percent(85.0),
                                    bottom:Val::Percent(0.0),
                                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(item_attribute.shop_item)
            .insert(BuyItemButton)
            .insert(PassiveSkillComponent::from(passive_skill));
        });
    };



    commands
        .spawn(NodeBundle {
            style: Style {
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

            //text info
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        position: UiRect { 
                                        left: Val::Percent(0.0),
                                        right: Val::Percent(0.0),
                                        top: Val::Percent(0.0),
                                        bottom:Val::Percent(90.0),
                                        },

                        ..Default::default()
                    },
                    background_color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Next Level {}, Now you can enhance your tank", game_level.level),
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
            })

            //shop area
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        position: UiRect { 
                                        left: Val::Percent(0.0),
                                        right: Val::Percent(0.0),
                                        top: Val::Percent(15.0),
                                        bottom:Val::Percent(50.0),
                                        },

                        ..Default::default()
                    },
                    background_color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(ShopArea)
                
                //shop item1
                .with_children(|parent| {
                    let item_attribute =  UiRect { 
                                                    left: Val::Percent(0.0),
                                                    right: Val::Percent(80.0),
                                                    top: Val::Percent(0.0),
                                                    bottom:Val::Percent(0.0),
                                                    };

                    let shop_item_attribute = ShopItemAttribute {
                        position: item_attribute,
                        shop_item: ShopItems::ITEM1,
                    };
                    let passive_skill = passive_skills.passive_skill_pool.get_random_item_from_pool();
                    spawn_shop_item(parent, shop_item_attribute, passive_skill);
                })
                            
                //shop item2
                .with_children(|parent| {
                    let item_attribute =  UiRect { 
                                                    left: Val::Percent(20.0),
                                                    right: Val::Percent(60.0),
                                                    top: Val::Percent(0.0),
                                                    bottom:Val::Percent(0.0),
                                                    };

                    let shop_item_attribute = ShopItemAttribute {
                        position: item_attribute,
                        shop_item: ShopItems::ITEM2,
                    };
                    let passive_skill = passive_skills.passive_skill_pool.get_random_item_from_pool();
                    spawn_shop_item(parent, shop_item_attribute, passive_skill);
                })           
            
                //shop item3
                .with_children(|parent| {
                    let item_attribute =  UiRect { 
                                                    left: Val::Percent(40.0),
                                                    right: Val::Percent(40.0),
                                                    top: Val::Percent(0.0),
                                                    bottom:Val::Percent(0.0),
                                                    };

                    let shop_item_attribute = ShopItemAttribute {
                        position: item_attribute,
                        shop_item: ShopItems::ITEM3,
                    };
                    let passive_skill = passive_skills.passive_skill_pool.get_random_item_from_pool();
                    spawn_shop_item(parent, shop_item_attribute, passive_skill);
                })

                //shop item4
                .with_children(|parent| {
                    let item_attribute =  UiRect { 
                                                    left: Val::Percent(60.0),
                                                    right: Val::Percent(20.0),
                                                    top: Val::Percent(0.0),
                                                    bottom:Val::Percent(0.0),
                                                    };

                    let shop_item_attribute = ShopItemAttribute {
                        position: item_attribute,
                        shop_item: ShopItems::ITEM4,
                    };
                    let passive_skill = passive_skills.passive_skill_pool.get_random_item_from_pool();
                    spawn_shop_item(parent, shop_item_attribute, passive_skill);
                })

                //shop item5
                .with_children(|parent| {
                    let item_attribute =  UiRect { 
                                                    left: Val::Percent(80.0),
                                                    right: Val::Percent(0.0),
                                                    top: Val::Percent(0.0),
                                                    bottom:Val::Percent(0.0),
                                                    };

                    let shop_item_attribute = ShopItemAttribute {
                        position: item_attribute,
                        shop_item: ShopItems::ITEM5,
                    };
                    let passive_skill = passive_skills.passive_skill_pool.get_random_item_from_pool();
                    spawn_shop_item(parent, shop_item_attribute, passive_skill);
                });

            })
            
            //next button
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
                .insert(NextLevelButton)                
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

fn buy_item_system(
    mut player_status: ResMut<PlayerStatus>,
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &ShopItems, &PassiveSkillComponent), (With<Interaction>, With<Button>, With<BuyItemButton>)>,
    item_query: Query<(Entity, &ShopItems), With<ItemArea>>    
) {
    for (interaction, shop_item, skill) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for (entity, item) in item_query.iter() {
                    if *item == *shop_item {
                        
                        if player_status.gold >= skill.cost as i64 {
                            //just only plus
                            let vlaue = match skill.value_change.unwrap() {
                                ValueChange::IncreaseInteger(val) => val as i64,
                                ValueChange::DecreaseInteger(val) => val as i64,
                                _ => 0
                            };                       
                                                
                            match skill.attribute {
                                PlayerStatusType::ATK => {
                                    player_status.atk += vlaue;
                                },
                                PlayerStatusType::DEF => {
                                    player_status.def += vlaue;
                                },
                                PlayerStatusType::HP => {
                                    player_status.cur_hp += vlaue;
                                    player_status.max_hp += vlaue;
                                },
                                _ => (),
                            }
                            player_status.gold -= skill.cost as i64;

                            commands.entity(entity).despawn_recursive();
                        } else {
                        
                        }
                        
                    }
                }
            },
            _=> (),
        }
    }    
}


fn to_next_level(
    mut commands: Commands,
    mut game_stage: ResMut<State<GameStage>>,
    mut single_level_live_timer: ResMut<SingleLevelTimer>,
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>, With<NextLevelButton>)>
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

pub(crate) fn timer_count_down_to_next_level(
    mut single_level_live_timer: ResMut<SingleLevelTimer>,
    mut game_level: ResMut<GameLevel>,
    time: Res<Time>,
    mut game_stage: ResMut<State<GameStage>>
) {
    if single_level_live_timer.1 {
        if single_level_live_timer.0.tick(time.delta()).just_finished() {

            //Next level page should pasue the timer
            single_level_live_timer.0.pause();
    
            //game_level.level += 1;
            game_level.increase_for_next_level();
                        
            game_stage.set(GameStage::NextLevel).unwrap();
        }
    }

}