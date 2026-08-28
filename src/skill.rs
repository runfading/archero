use crate::GameState;
use crate::skill::ability::SkillAbilityPlugin;
use crate::skill::property::SkillPropertyPlugin;
use bevy::prelude::*;

mod ability;
mod property;
mod skill_list;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillSet {
    /// 选择（ui方面的）
    Select,
    /// 技能适用
    Effective,
}

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                SkillSet::Select.run_if(in_state(GameState::InGame)),
                SkillSet::Effective
                    .run_if(in_state(GameState::InGame))
                    .after(SkillSet::Select),
            ),
        )
        .add_plugins(SkillPropertyPlugin)
        .add_plugins(SkillAbilityPlugin);
    }
}
