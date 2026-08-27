use crate::actors::player::Player;
use crate::core::RunStats;
use crate::core::health::Health;
use crate::{GameSet, GameState, RunPhase, RunSet};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_widget::prelude::{ButtonBuilder, ButtonStyle};

/// 游戏运行ui根组件
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudRoot;

/// 生命值文本
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudHpText;

/// 生命值填充ui
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudHpFill;

/// 波次信息
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudInfoText;

/// 金币信息
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudGoldText;

/// 击杀数信息
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudKillsText;

/// 经验值信息
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudXpFill;

/// 经验值文本
#[derive(Component, Debug, Default, Copy, Clone)]
struct HudXpText;

/// 暂停菜单
#[derive(Component, Debug, Default, Copy, Clone)]
struct PauseOverlay;

pub struct PlayingUiPlugin;

/// 游玩ui（暂停ui，hud）
impl Plugin for PlayingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud.in_set(GameSet::Ui))
            .add_systems(OnExit(GameState::InGame), despawn_hud.in_set(GameSet::Ui))
            .add_systems(
                Update,
                (
                    update_hp_hud,
                    update_xp_hud,
                    update_gold_hud,
                    update_kills_hud,
                )
                    .in_set(RunSet::Playing)
                    .in_set(GameSet::Ui),
            )
            .add_systems(
                OnEnter(RunPhase::Paused),
                spawn_pause_hud.in_set(GameSet::Ui),
            )
            .add_systems(
                OnExit(RunPhase::Paused),
                despawn_pause_menu.in_set(GameSet::Ui),
            );
    }
}

fn spawn_hud(mut commands: Commands, query: Query<(&Health, &Player)>, states: Res<RunStats>) {
    let (health, player) = match query.single() {
        Ok(player) => player,
        Err(err) => {
            error!("玩家状态组件查询异常 {}", err);
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
            xp_hud(player)
        ]
    });
}

fn spawn_pause_hud(mut commands: Commands) {
    commands.spawn_scene(pause_ui());
}

/// 生命值hud
fn hp_hud(health: &Health) -> impl Scene {
    let hp_text = health_text(health);
    let hp_percent = health.ratio() * 100.0;
    let hp_color = health_color(health.ratio());

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
                Text(hp_text)
                TextFont { font_size: FontSize::Px(18.0), }
                TextColor(Color::WHITE)
            ),
            (
                Node { width: px(260), height: px(16),}
                BackgroundColor(Color::srgb(0.10, 0.10, 0.10))
                Children [
                    (
                        HudHpFill
                        Node { width: Val::Percent(hp_percent), height: Val::Percent(100.0), }
                        BackgroundColor(hp_color)
                    )
                ]
            ),
        ]
    }
}

/// 波次hud
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

/// 状态（击杀、金币、暂停键等）ui
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
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                             normal: Color::srgb(0.20, 0.25, 0.35),
                             hovered: Color::srgb(0.27, 0.33, 0.45),
                             pressed: Color::srgb(0.14, 0.18, 0.27),
                             ..default()
                        })
                        .label(
                            bsn! {
                                Text::new("暂停")
                                TextFont { font_size: FontSize::Px(18.0), }
                                TextColor(Color::WHITE)
                            }
                        )
                        .build()
                }
                Node {
                    width: px(72),
                    height: px(40),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(8)),
                }
                on(|_event: On<Activate>, mut next_state: ResMut<NextState<RunPhase>>|{
                    next_state.set(RunPhase::Paused);
                })
            )
        ]
    }
}

/// 经验值hud
fn xp_hud(player: &Player) -> impl Scene {
    let xp_percent = xp_ratio(player) * 100.0;
    let xp_text = xp_text(player);

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
                    width: Val::Percent(xp_percent),
                }
                BackgroundColor(Color::srgb(0.30, 0.55, 0.95))
            ),
            (
                HudXpText
                Text::new(xp_text)
                TextFont { font_size: FontSize::Px(15.0),}
                TextColor(Color::WHITE)
            )
        ]
    }
}

fn update_hp_hud(
    player: Query<&Health, (With<Player>, Changed<Health>)>,
    mut hp_text_query: Query<&mut Text, With<HudHpText>>,
    mut hp_fill_query: Query<(&mut Node, &mut BackgroundColor), With<HudHpFill>>,
) {
    let Ok(health) = player.single() else {
        return;
    };

    if let Ok(mut text) = hp_text_query.single_mut() {
        *text = Text::new(health_text(health));
    }

    if let Ok((mut node, mut color)) = hp_fill_query.single_mut() {
        let ratio = health.ratio();
        node.width = Val::Percent(ratio * 100.0);
        color.0 = health_color(ratio);
    }
}

fn update_xp_hud(
    player_query: Query<&Player, Changed<Player>>,
    mut xp_text_query: Query<&mut Text, With<HudXpText>>,
    mut xp_fill_query: Query<&mut Node, With<HudXpFill>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    if let Ok(mut text) = xp_text_query.single_mut() {
        *text = Text::new(xp_text(player));
    }

    if let Ok(mut node) = xp_fill_query.single_mut() {
        node.width = Val::Percent(xp_ratio(player) * 100.0);
    }
}

fn update_gold_hud(stats: Res<RunStats>, mut query: Query<&mut Text, With<HudGoldText>>) {
    if !stats.is_changed() {
        return;
    }

    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("金币 {}", stats.gold));
    }
}

fn update_kills_hud(stats: Res<RunStats>, mut query: Query<&mut Text, With<HudKillsText>>) {
    if !stats.is_changed() {
        return;
    }

    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("击杀 {}", stats.kills));
    }
}

fn health_text(health: &Health) -> String {
    format!(
        "HP {:.0}/{:.0}",
        health.current.max(0.0),
        health.max.max(0.0)
    )
}

fn health_color(ratio: f32) -> Color {
    if ratio <= 0.25 {
        Color::srgb(0.90, 0.22, 0.24)
    } else if ratio <= 0.5 {
        Color::srgb(0.95, 0.66, 0.20)
    } else {
        Color::srgb(0.30, 0.85, 0.40)
    }
}

fn xp_ratio(player: &Player) -> f32 {
    if player.xp_to_next == 0 {
        return 0.0;
    }

    (player.xp as f32 / player.xp_to_next as f32).clamp(0.0, 1.0)
}

fn xp_text(player: &Player) -> String {
    format!(
        "LV {}  {}/{} XP",
        player.level, player.xp, player.xp_to_next
    )
}

/// 游戏暂停ui
fn pause_ui() -> impl Scene {
    bsn! {
        PauseOverlay
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(14),
        }
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6))
        Children [
            (
                Text::new("已暂停")
                TextFont { font_size: FontSize::Px(46.0),}
                TextColor(Color::WHITE)
            ),
            (
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                            normal: Color::srgb(0.20, 0.45, 0.24),
                            hovered: Color::srgb(0.27, 0.56, 0.32),
                            pressed: Color::srgb(0.14, 0.34, 0.18),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("继续游戏")
                            TextFont { font_size: FontSize::Px(22.0), }
                            TextColor(Color::WHITE)
                        })
                        .build()
                }
                Node {
                    width: px(240),
                    height: px(56),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event:On<Activate>, mut next_state: ResMut<NextState<RunPhase>>|{
                    next_state.set(RunPhase::Playing);
                })
            ),
            (
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                            normal: Color::srgb(0.20, 0.34, 0.58),
                            hovered: Color::srgb(0.27, 0.44, 0.70),
                            pressed: Color::srgb(0.14, 0.25, 0.46),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("重新开始")
                            TextFont { font_size: FontSize::Px(22.0),}
                            TextColor(Color::WHITE)
                        })
                        .build()
                }
                Node {
                    width: px(240),
                    height: px(56),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event:On<Activate>, _next_state: ResMut<NextState<RunPhase>>|{
                    info!("重新开始")
                })
            ),
            (
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                            normal: Color::srgb(0.35, 0.30, 0.26),
                            hovered: Color::srgb(0.46, 0.39, 0.33),
                            pressed: Color::srgb(0.26, 0.22, 0.19),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("返回主菜单")
                            TextFont { font_size: FontSize::Px(22.0), }
                            TextColor(Color::WHITE)
                        })
                        .build()
                }
                Node {
                    width: px(240),
                    height: px(56),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event:On<Activate>, mut next_state: ResMut<NextState<GameState>>|{
                    next_state.set(GameState::MainMenu);
                })
            )
        ]
    }
}

fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HudRoot>>) {
    for entity in hud_query {
        commands.entity(entity).despawn();
    }
}

fn despawn_pause_menu(mut commands: Commands, pause_ui_query: Query<Entity, With<PauseOverlay>>) {
    for entity in pause_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
