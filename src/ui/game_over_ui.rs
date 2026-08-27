use crate::GameState;
use crate::core::RunStats;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_widget::prelude::{ButtonBuilder, ButtonStyle};

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_ui);
    }
}

/// 游戏结束界面的根节点，用于离开结算状态时统一清理。
#[derive(Component, Debug, Default, Copy, Clone)]
struct GameOverRoot;

fn spawn_game_over_ui(mut commands: Commands, stats: Res<RunStats>) {
    commands.spawn_scene(game_over_scene(&stats));
}

fn game_over_scene(stats: &RunStats) -> impl Scene {
    bsn! {
        GameOverRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(24)),
        }
        BackgroundColor(Color::srgba(0.025, 0.03, 0.045, 0.96))
        Children [
            result_card(stats)
        ]
    }
}

fn result_card(stats: &RunStats) -> impl Scene {
    let elapsed = format_time(stats.time);
    let kills = stats.kills.to_string();
    let reward = format!("◆  {} 金币", stats.gold);

    bsn! {
        Node {
            width: Val::Percent(100.0),
            max_width: px(520),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::new(px(42), px(42), px(36), px(34)),
            row_gap: px(10),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(22)),
        }
        BackgroundColor(Color::srgb_u8(27, 30, 42))
        BorderColor::all(Color::srgb_u8(73, 78, 99))
        Children [
            (
                Text::new("闯关结束")
                TextFont { font_size: FontSize::Px(54.0), }
                TextColor(Color::srgb_u8(239, 86, 91))
            ),
            (
                Text::new("胜败乃兵家常事，整装再战吧")
                TextFont { font_size: FontSize::Px(18.0), }
                TextColor(Color::srgb_u8(166, 171, 190))
                Node { margin: UiRect::bottom(px(18)) }
            ),
            summary_panel(elapsed, kills),
            (
                Text::new("本局获得")
                TextFont { font_size: FontSize::Px(16.0), }
                TextColor(Color::srgb_u8(135, 141, 160))
                Node { margin: UiRect::top(px(10)) }
            ),
            (
                Text::new(reward)
                TextFont { font_size: FontSize::Px(25.0), }
                TextColor(Color::srgb_u8(247, 205, 75))
                Node { margin: UiRect::bottom(px(14)) }
            ),
            action_buttons(),
            (
                Text::new("再试一次，下一局会走得更远")
                TextFont { font_size: FontSize::Px(14.0), }
                TextColor(Color::srgb_u8(104, 110, 130))
                Node { margin: UiRect::top(px(8)) }
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
        BackgroundColor(Color::srgb_u8(19, 21, 31))
        Children [
            stat_item("存活时间", elapsed, Color::srgb_u8(102, 190, 255)),
            (
                Node {
                    width: px(1),
                    height: px(48),
                }
                BackgroundColor(Color::srgb_u8(62, 66, 82))
            ),
            stat_item(
                "击败敌人",
                kills,
                Color::srgb_u8(243, 133, 84),
            )
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
                TextColor(Color::srgb_u8(142, 148, 167))
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
                            normal: Color::srgb_u8(51, 115, 61),
                            hovered: Color::srgb_u8(65, 137, 76),
                            pressed: Color::srgb_u8(39, 91, 47),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("重新挑战")
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
                            normal: Color::srgb_u8(48, 53, 69),
                            hovered: Color::srgb_u8(62, 68, 88),
                            pressed: Color::srgb_u8(38, 42, 56),
                            ..default()
                        })
                        .label(bsn! {
                            Text::new("返回主菜单")
                            TextFont { font_size: FontSize::Px(21.0), }
                            TextColor(Color::srgb_u8(221, 224, 234))
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

fn despawn_game_over_ui(mut commands: Commands, roots: Query<Entity, With<GameOverRoot>>) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::format_time;

    #[test]
    fn formats_elapsed_time_as_minutes_and_seconds() {
        assert_eq!(format_time(0.0), "00:00");
        assert_eq!(format_time(65.9), "01:05");
    }
}
