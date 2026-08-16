use crate::GameState;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_widget::prelude::{ButtonStyle, PrimaryButton, button_with_label};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

#[derive(Component, Default, Clone)]
struct MenuRoot;

fn spawn_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        menu_button()
    });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    query.for_each(|e| commands.entity(e).despawn());
}

fn menu_button() -> impl Scene {
    bsn! {
        MenuRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(18),
        }
        BackgroundColor(Color::srgb(0.07, 0.08, 0.11))
        children()
    }
}

fn children() -> impl Scene {
    let high = 0;
    bsn! {
        Children [
            (
                Text::new("弓箭传说")
                TextFont { font_size: FontSize::Px(72.0), }
                TextColor(Color::srgb(0.95, 0.85, 0.35))
            ),
            (
                Text::new("Roguelike 弓箭射击")
                TextFont { font_size: FontSize::Px(22.0),}
                TextColor(Color::srgb(0.72, 0.75, 0.82))
            ),
            (
                button_with_label(
                    PrimaryButton::default(),
                    ButtonStyle {
                        normal: Color::srgb_u8(51, 115, 61),
                        hovered: Color::srgb_u8(65, 137, 76),
                        pressed: Color::srgb_u8(39, 91, 47),
                        ..default()
                    },
                    bsn! {
                        Text::new("关卡模式")
                        TextFont { font_size: FontSize::Px(24.0), }
                        TextColor(Color::WHITE)
                    }
                )
                Node {
                    width: px(260),
                    height: px(65),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
            ),
            (
                button_with_label(
                    PrimaryButton::default(),
                    ButtonStyle {
                        normal: Color::srgb_u8(51, 97, 148),
                        hovered: Color::srgb_u8(66, 117, 174),
                        pressed: Color::srgb_u8(39, 76, 119),
                        ..default()
                    },
                    bsn! {
                        Text::new("无尽模式")
                        TextFont { font_size: FontSize::Px(24.0), }
                        TextColor(Color::WHITE)
                    }
                )
                Node {
                    width: px(260),
                    height: px(65),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
            ),
            (
                Text(format!("无尽模式最高分{high}"))
                TextColor(Color::srgb_u8(242, 217, 89))
                TextFont { font_size: FontSize::Px(20.0)}
            ),
            (
                Text("WASD / 方向键 移动 · 自动射击 · Esc 暂停 · 升级时按 1/2/3 选择 按 F 刷新")
                TextColor(Color::srgb_u8(173, 179, 196))
                TextFont { font_size: FontSize::Px(16.0)}
            ),
            (
                button_with_label(
                    PrimaryButton::default(),
                    ButtonStyle {
                        normal: Color::srgb_u8(76, 51, 51),
                        hovered: Color::srgb_u8(101, 67, 67),
                        pressed: Color::srgb_u8(59, 39, 39),
                        ..default()
                    },
                    bsn! {
                        Text::new("退出游戏")
                        TextFont { font_size: FontSize::Px(24.0), }
                        TextColor(Color::WHITE)
                    }
                )
                Node {
                    width: px(260),
                    height: px(65),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(12)),
                }
                on(|_event:On<Activate>,mut writer: MessageWriter<AppExit>|{
                    writer.write(AppExit::Success);
                })
            ),
        ]
    }
}
