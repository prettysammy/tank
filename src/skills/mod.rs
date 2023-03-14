mod skills_pool;
pub use self::skills_pool::*;

use bevy::prelude::*;

use crate::utils::Pool;

use self::skills_pool::get_passive_skill_pool;

#[derive(Resource)]
pub struct PassiveSkills {
    pub passive_skill_pool: Pool<PassiveSkill>,
}

impl Default for PassiveSkills {    
    fn default() -> Self {
        let passive_skill_pool = get_passive_skill_pool();
        PassiveSkills{
            passive_skill_pool
        }
    }
}


pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PassiveSkills>();
    }
}