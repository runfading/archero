use crate::skill::property::PropertyType;
use bevy::prelude::*;
use crate::skill::ability::{AbilityType, SkillAbilityPlugin};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillId {
    RapidFire,
    PowerShot,
    Multishot,
    Pierce,
    Vitality,
    FleetFoot,
    CriticalFocus,
}

impl SkillId {
    const ALL: [Self; 7] = [
        Self::RapidFire,
        Self::PowerShot,
        Self::Multishot,
        Self::Pierce,
        Self::Vitality,
        Self::FleetFoot,
        Self::CriticalFocus,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::RapidFire => "快速装填",
            Self::PowerShot => "强力射击",
            Self::Multishot => "多重箭",
            Self::Pierce => "贯穿",
            Self::Vitality => "活力",
            Self::FleetFoot => "轻盈步伐",
            Self::CriticalFocus => "致命专注",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::RapidFire => "攻击间隔 -12%",
            Self::PowerShot => "武器伤害 +20%",
            Self::Multishot => "投射物 +1，增加扩散角",
            Self::Pierce => "箭矢额外贯穿 1 个敌人",
            Self::Vitality => "最大生命 +18%，并回复增加值",
            Self::FleetFoot => "移动速度 +10%",
            Self::CriticalFocus => "暴击率 +8%，暴伤 +10%",
        }
    }

    pub fn property(&self) -> Vec<(PropertyType, f32)> {
        match self {
            SkillId::RapidFire => {
                vec![(PropertyType::CooldownTime, -0.01)]
            }
            SkillId::PowerShot => {
                vec![(PropertyType::DamageMultiply, 0.02)]
            }
            _=> vec![],
        }
    }

    pub fn ability(&self)->Vec<(AbilityType, u32)>{
        match self {
            SkillId::RapidFire => {}
            SkillId::PowerShot => {}
            SkillId::Multishot => {}
            SkillId::Pierce => {}
            SkillId::Vitality => {}
            SkillId::FleetFoot => {}
            SkillId::CriticalFocus => {}
        }
    }
}
