mod endless;
mod hud;
mod level;
mod player;

use crate::game::hud::HudPlugin;
use crate::game::player::Player;
use crate::{GameSet, GameState, RunPhase};
use bevy::prelude::*;

/// 局内单位标记
#[derive(Component, Default, Copy, Clone)]
pub struct RunEntity;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunMode {
    Level(usize),
    Endless,
}

impl Default for RunMode {
    fn default() -> Self {
        Self::Level(0)
    }
}

/// 本局累计数据。
#[derive(Resource, Default)]
pub struct RunStats {
    pub gold: usize,
    pub kills: usize,
    pub time: f32,
}

/// 关卡房间上下文资源
#[derive(Resource)]
pub struct LevelContext {
    /// 房间序列
    pub rooms: Vec<RoomDef>,
    /// 当前所在房间索引
    pub room_index: usize,
    /// 等待选择技能
    pub awaiting_choice: bool,
}

/// 该房间设定
#[derive(Clone, Copy, Debug)]
pub struct RoomDef {
    /// 是否存在boos
    pub boss: bool,
    /// 近战敌人数
    pub melee: u32,
    /// 远程攻击敌人数
    pub ranged: u32,
    /// /// 精英敌人数
    pub elite: u32,
}

/// 无尽模式上下文资源
#[derive(Resource)]
pub struct EndlessContext {
    /// 当前波数
    pub wave: u32,
    /// 下一波时间
    pub next_wave_timer: f32,
    /// 当前波已过时间
    pub elapsed: f32,
}

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

/// 单位阵营
#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunStats>()
            .add_systems(
                OnEnter(GameState::InGame),
                setup_run.in_set(GameSet::Gameplay),
            )
            .add_systems(OnExit(GameState::InGame), teardown_run)
            .add_plugins(HudPlugin);
    }
}

fn setup_run(
    mut commands: Commands,
    mut stats: ResMut<RunStats>,
    mut next_phase: ResMut<NextState<RunPhase>>,
) {
    *stats = RunStats::default();
    // *build = PlayerBuild::default();
    // *choices = SkillChoices::default();
    next_phase.set(RunPhase::Playing);
    init_run(&mut commands);
}

fn init_run(commands: &mut Commands) {
    commands.spawn_scene(bsn! {
        Player {}
        Health::full(60.0)
    });
}

/// 清理游戏运行状态：
///
/// - 递归移除所有带有 [`RunEntity`] 标记的实体；
/// - 移除 [`LevelContext`] 和 [`EndlessContext`] 资源。
fn teardown_run(mut commands: Commands, entities: Query<Entity, With<RunEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<LevelContext>();
    commands.remove_resource::<EndlessContext>();
}
