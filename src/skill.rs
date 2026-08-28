use crate::GameState;
use crate::skill::ability::SkillAbilityPlugin;
use crate::skill::property::SkillPropertyPlugin;
use bevy::prelude::*;

pub(crate) mod ability;
pub(crate) mod property;
pub(crate) mod skill_list;

/// 尚未消费的升级选择次数，支持一次获得多级时连续选择。
#[derive(Resource, Debug, Default)]
pub struct PendingLevelUps(pub u32);

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
        app.init_resource::<PendingLevelUps>()
            .configure_sets(
                FixedUpdate,
                (
                    SkillSet::Select.run_if(in_state(GameState::InGame)),
                    SkillSet::Effective
                        .run_if(in_state(GameState::InGame))
                        .after(SkillSet::Select),
                ),
            )
            .add_plugins(SkillPropertyPlugin)
            .add_plugins(SkillAbilityPlugin)
            .add_systems(OnEnter(GameState::InGame), reset_pending_level_ups);
    }
}

fn reset_pending_level_ups(mut pending: ResMut<PendingLevelUps>) {
    pending.0 = 0;
}
