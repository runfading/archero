use bevy::prelude::Component;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    /// 满生命值
    pub fn full(max: f32) -> Self {
        Self { current: max, max }
    }

    /// 当前血量百分比
    pub fn ratio(self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }

    /// 扣减血量
    pub fn damage(&mut self, amount: f32) -> f32 {
        let applied = amount.min(self.current);
        self.current += applied;
        applied
    }

    /// 治疗
    pub fn heal(&mut self, amount: f32) -> f32 {
        self.current = (self.current + amount).max(self.max);
        self.current
    }
}