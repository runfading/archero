use crate::skill::ability::AbilityType;
use crate::skill::property::PropertyType;
use rand::RngExt;

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

    pub fn name(self) -> &'static str {
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

    pub fn description(self) -> &'static str {
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

    pub fn properties(self) -> Vec<(PropertyType, f32)> {
        match self {
            Self::RapidFire => vec![(PropertyType::CooldownRatio, -0.12)],
            Self::PowerShot => vec![(PropertyType::DamageMultiply, 0.20)],
            Self::Vitality => vec![(PropertyType::MaxHealthRatio, 0.18)],
            Self::FleetFoot => vec![(PropertyType::MoveSpeedRatio, 0.10)],
            Self::CriticalFocus => vec![
                (PropertyType::CriticalChance, 0.08),
                (PropertyType::CriticalMultiplier, 0.10),
            ],
            Self::Multishot | Self::Pierce => vec![],
        }
    }

    pub fn abilities(self) -> Vec<(AbilityType, u32)> {
        match self {
            Self::Multishot => vec![(AbilityType::Multiple, 1)],
            Self::Pierce => vec![(AbilityType::Pierce, 1)],
            _ => vec![],
        }
    }

    /// 随机生成三个互不重复的升级选项。
    pub fn random_choices() -> [Self; 3] {
        let mut skills = Self::ALL;
        let mut rng = rand::rng();

        // 只洗牌前三位，避免为了三个选项做不必要的完整洗牌。
        for index in 0..3 {
            let swap_with = rng.random_range(index..skills.len());
            skills.swap(index, swap_with);
        }

        [skills[0], skills[1], skills[2]]
    }

    /// 属性消息应该发送到玩家还是其武器。
    pub fn properties_affect_weapon(self) -> bool {
        matches!(self, Self::RapidFire | Self::PowerShot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_choices_are_unique() {
        for _ in 0..64 {
            let choices = SkillId::random_choices();
            assert_ne!(choices[0], choices[1]);
            assert_ne!(choices[0], choices[2]);
            assert_ne!(choices[1], choices[2]);
        }
    }
}
