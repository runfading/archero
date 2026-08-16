use crate::game::player::Player;
use crate::game::{Health, RunStats};
use crate::{GameSet, GameState, RunPhase};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

#[derive(Component, Debug, Default, Copy, Clone)]
struct HudRoot;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudHpText;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudHpFill;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudInfoText;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudGoldText;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudKillsText;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudXpFill;
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudXpText;
#[derive(Component, Debug, Default, Copy, Clone)]
struct PauseOverlay;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud.in_set(GameSet::Ui))
            .add_systems(OnExit(GameState::InGame), despawn_hud);
    }
}

fn spawn_hud(mut commands: Commands, query: Query<&Health, With<Player>>, states: Res<RunStats>) {
    let health = match query.single() {
        Ok(health) => health,
        Err(err) => {
            error!("玩家生命组件查询异常 {}", err);
            return;
        }
    };

    commands.spawn_scene(bsn! {
        #主hud
        HudRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        }
        Children [
            hp_hud(health),
            wav_hud(),
            state_hud(&states),
            ex_hud()
        ]
    });
}

fn hp_hud(health: &Health) -> impl Scene {
    bsn! {
        #生命值hud
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(12),
            width: px(280),
            flex_direction: FlexDirection::Column,
            row_gap: px(4)
        }
        Children [
            (
                HudHpText
                Text(format!("HP {}/{}",health.current, health.max))
                TextFont { font_size: FontSize::Px(18.0), }
                TextColor(Color::WHITE)
            ),
            (
                Node { width: px(260), height: px(16),}
                BackgroundColor(Color::srgb(0.10, 0.10, 0.10))
                Children [
                    (
                        HudHpFill
                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), }
                        BackgroundColor(Color::srgb(0.30, 0.85, 0.40))
                    )
                ]
            ),
        ]
    }
}

fn wav_hud() -> impl Scene {
    bsn! {
         Node {
            position_type: PositionType::Absolute,
            top: px(12),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center
        }
        Children [
            (
                HudInfoText
                Text("")
                TextFont { font_size: FontSize::Px(22.0), }
                TextColor(Color::WHITE)
            )
        ]
    }
}

fn state_hud(state: &RunStats) -> impl Scene {
    let gold_text = format!("金币 {}", state.gold);
    let kills_text = format!("击杀 {}", state.kills);

    bsn! {
        #金币击杀hud
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(16),
            align_items: AlignItems::Center,
            column_gap: px(12),
        }
        Children [
            (
                HudGoldText
                Text::new(gold_text)
                TextFont { font_size: FontSize::Px(18.0),}
                TextColor(Color::srgb(1.0, 0.84, 0.30))
            ),
            (
                HudKillsText
                Text::new(kills_text)
                TextFont { font_size: FontSize::Px(18.0), }
                TextColor(Color::srgb(0.80, 0.82, 0.88))
            ),
            (
                Button
                Node {
                    width: px(72),
                    height: px(40),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(8)),
                }
                BackgroundColor(Color::srgb(0.20, 0.25, 0.35))
                Children [
                    (
                        Text::new("暂停")
                        TextFont { font_size: FontSize::Px(18.0), }
                        TextColor(Color::WHITE)
                    )
                ]
                on(|_event: On<Activate>, mut next_state: ResMut<NextState<RunPhase>>|{
                    next_state.set(RunPhase::Paused);
                })
            )
        ]
    }
}

fn ex_hud() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(16),
            right: px(16),
            height: px(22),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(Color::srgb(0.10, 0.10, 0.10))
        Children [
            (
                HudXpFill
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    bottom: px(0),
                    width: Val::Percent(0.0),
                }
                BackgroundColor(Color::srgb(0.30, 0.55, 0.95))
            ),
            (
                HudXpText
                Text::new("LV 1")
                TextFont { font_size: FontSize::Px(15.0),}
                TextColor(Color::WHITE)
            )
        ]
    }
}

fn despawn_hud(mut commands: Commands) {}

fn pause_menu(mut commands: Commands) {}

fn despawn_pause_menu(mut commands: Commands) {}
