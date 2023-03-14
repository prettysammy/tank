use crate::{utils::Pool, player::PlayerStatusType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ValueChange {
    IncreaseInteger(usize),
    DecreaseInteger(usize),
    IncreaseFloat(f32),
    DecreaseFloat(f32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PassiveSkill {
    pub attribute: PlayerStatusType,
    pub describe: String,
    pub image_label: String,
    pub level: usize,
    pub cost: usize,
    pub value_change: Option<ValueChange>,
}

pub fn get_passive_skill_pool() -> Pool<PassiveSkill> {
    let config = include_str!("../../assets/pool/passive_skill.ron");
    ron::from_str(config).unwrap()
}