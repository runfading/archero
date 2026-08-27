use crate::GameState;
use crate::core::RunStats;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_widget::prelude::{ButtonBuilder, ButtonStyle};

pub struct GameClearUiPlugin;

impl Plugin for GameClearUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameClear), spawn_game_clear_ui)
            .add_systems(OnExit(GameState::GameClear), despawn_game_clear_ui);
    }
}

/// 通关结算界面的根节点。
#[derive(Component, Debug, Default, Copy, Clone)]
struct GameClearRoot;

fn spawn_game_clear_ui(mut commands: Commands, stats: Res<RunStats>) {
    commands.spawn_scene(game_clear_scene(&stats));
}

fn game_clear_scene(stats: &RunStats) -> impl Scene {
    bsn! {
        GameClearRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(24)),
        }
        BackgroundColor(Color::srgba(0.025, 0.035, 0.035, 0.97))
        Children [
            clear_card(stats)
        ]
    }
}

fn clear_card(stats: &RunStats) -> impl Scene {
    let elapsed = format_time(stats.time);
    let kills = stats.kills.to_string();
    let reward = format!("◆  {} 金币", stats.gold);

    bsn! {
        Node {
            width: Val::Percent(100.0),
            max_width: px(540),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::new(px(44), px(44), px(32), px(32)),
            row_gap: px(9),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(22)),
        }
        BackgroundColor(Color::srgb_u8(27, 34, 38))
        BorderColor::all(Color::srgb_u8(177, 145, 52))
        Children [
            (
                Text::new("★  ★  ★")
                TextFont { font_size: FontSize::Px(29.0), }
                TextColor(Color::srgb_u8(250, 207, 73))
            ),
            (
                Text::new("恭喜通关")
                TextFont { font_size: FontSize::Px(54.0), }
                TextColor(Color::srgb_u8(255, 220, 104))
            ),
            (
                Text::new("所有敌人已被击败，胜利属于勇者")
                TextFont { font_size: FontSize::Px(18.0), }
                TextColor(Color::srgb_u8(174, 189, 184))
                Node { margin: UiRect::bottom(px(16)) }
            ),
            summary_panel(elapsed, kills),
            (
                Text::new("通关奖励")
                TextFont { font_size: FontSize::Px(16.0), }
                TextColor(Color::srgb_u8(139, 154, 149))
                Node { margin: UiRect::top(px(10)) }
            ),
            (
                Text::new(reward)
                TextFont { font_size: FontSize::Px(27.0), }
                TextColor(Color::srgb_u8(250, 207, 73))
                Node { margin: UiRect::bottom(px(13)) }
            ),
            action_buttons(),
            (
                Text::new("本关已完成，可以再次挑战刷新纪录")
                TextFont { font_size: FontSize::Px(14.0), }
                TextColor(Color::srgb_u8(105, 123, 117))
                Node { margin: UiRect::top(px(7)) }
            )
        ]
    }
}

fn summary_panel(elapsed: String, kills: String) -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            padding: UiRect::axes(px(12), px(18)),
            border_radius: BorderRadius::all(px(14)),
        }
        BackgroundColor(Color::srgb_u8(18, 25, 27))
        Children [
            stat_item("通关用时", elapsed, Color::srgb_u8(103, 211, 171)),
            (
                Node { width: px(1), height: px(48) }
                BackgroundColor(Color::srgb_u8(65, 77, 76))
            ),
            stat_item("消灭敌人", kills, Color::srgb_u8(246, 172, 77))
        ]
    }
}

fn stat_item(label: &'static str, value: String, color: Color) -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(42.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(4),
        }
        Children [
            (
                Text::new(value)
                TextFont { font_size: FontSize::Px(30.0), }
                TextColor(color)
            ),
            (
                Text::new(label)
                TextFont { font_size: FontSize::Px(15.0), }
                TextColor(Color::srgb_u8(148, 163, 158))
            )
        ]
    }
}

fn action_buttons() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(12),
        }
        Children [
            (
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                            normal: Color::srgb_u8(51, 125, 82),
                            hovered: Color::srgb_u8(66, 148, 99),
                            pressed: Color::srgb_u8(39, 98, 64),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("再次挑战")
                            TextFont { font_size: FontSize::Px(23.0), }
                            TextColor(Color::WHITE)
                        })
                        .build()
                }
                Node {
                    width: Val::Percent(100.0),
                    height: px(58),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::GameLoading);
                })
            ),
            (
                {
                    ButtonBuilder::builder()
                        .button_style(ButtonStyle {
                            normal: Color::srgb_u8(51, 59, 62),
                            hovered: Color::srgb_u8(65, 75, 78),
                            pressed: Color::srgb_u8(40, 47, 49),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("返回主菜单")
                            TextFont { font_size: FontSize::Px(21.0), }
                            TextColor(Color::srgb_u8(224, 231, 228))
                        })
                        .build()
                }
                Node {
                    width: Val::Percent(100.0),
                    height: px(52),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::MainMenu);
                })
            )
        ]
    }
}

fn format_time(seconds: f32) -> String {
    let total_seconds = seconds.max(0.0).floor() as u64;
    format!("{:02}:{:02}", total_seconds / 60, total_seconds % 60)
}

fn despawn_game_clear_ui(mut commands: Commands, roots: Query<Entity, With<GameClearRoot>>) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::format_time;

    #[test]
    fn formats_clear_time_as_minutes_and_seconds() {
        assert_eq!(format_time(0.0), "00:00");
        assert_eq!(format_time(125.8), "02:05");
    }
}
