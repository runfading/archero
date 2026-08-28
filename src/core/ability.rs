use bevy::ecs::query::QueryData;
use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
pub enum AbilityType {
    /// 正向弹药
    Forward,
    /// 斜向弹药
    Oblique,
    /// 多重施法
    Multiple,
    /// 攻击弹射
    Ejection,
    /// 穿透
    Pierce,
}

/// 正向数量
#[derive(Component, Clone, Copy)]
pub struct Forward(pub u32);

impl Default for Forward {
    fn default() -> Self {
        Forward(1)
    }
}

impl Forward {
    pub fn change(&mut self, amount: u32) {
        self.0 += amount;
    }
}

/// 斜向数量
#[derive(Component, Default, Clone, Copy)]
pub struct Oblique(pub u32);

impl Oblique {
    pub fn change(&mut self, amount: u32) {
        self.0 += amount;
    }
}

/// 多重施法
#[derive(Component, Default, Clone, Copy)]
pub struct Multiple(pub u32);

impl Multiple {
    pub fn change(&mut self, amount: u32) {
        self.0 += amount;
    }
}

/// 穿透数量
#[derive(Component, Default, Clone, Copy)]
pub struct Pierce(pub u32);

impl Pierce {
    pub fn change(&mut self, amount: u32) {
        self.0 += amount;
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Ejection(pub u32);

impl Ejection {
    pub fn change(&mut self, amount: u32) {
        self.0 += amount;
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AbilityQuery<'a> {
    pub forward: Option<&'a mut Forward>,
    pub oblique: Option<&'a mut Oblique>,
    pub multiple: Option<&'a mut Multiple>,
    pub pierce: Option<&'a mut Pierce>,
    pub ejection: Option<&'a mut Ejection>,
}
