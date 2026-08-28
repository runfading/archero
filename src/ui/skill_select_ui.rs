use crate::actors::player::Player;
use crate::core::weapon::WeaponId;
use crate::skill::PendingLevelUps;
use crate::skill::ability::AbilityChangeMessage;
use crate::skill::property::PropertyChangeMessage;
use crate::skill::skill_list::SkillId;
use crate::{GameSet, RunPhase};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_widget::prelude::{ButtonBuilder, ButtonStyle};

const CHOICE_COUNT: usize = 3;

#[derive(Component, Debug, Default, Copy, Clone)]
struct SkillSelectRoot;

#[derive(Resource, Debug, Clone, Copy)]
struct SkillChoices([SkillId; CHOICE_COUNT]);

impl Default for SkillChoices {
    fn default() -> Self {
        Self([SkillId::RapidFire, SkillId::PowerShot, SkillId::Multishot])
    }
}

#[derive(SystemParam)]
struct SkillSelectionParams<'w, 's> {
    commands: Commands<'w, 's>,
    choices: ResMut<'w, SkillChoices>,
    pending: ResMut<'w, PendingLevelUps>,
    next_phase: ResMut<'w, NextState<RunPhase>>,
    players: Query<'w, 's, Entity, With<Player>>,
    weapons: Query<'w, 's, (Entity, &'static ChildOf), With<WeaponId>>,
    roots: Query<'w, 's, Entity, With<SkillSelectRoot>>,
    property_writer: MessageWriter<'w, PropertyChangeMessage>,
    ability_writer: MessageWriter<'w, AbilityChangeMessage>,
}

pub struct SkillSelectUiPlugin;

impl Plugin for SkillSelectUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillChoices>()
            .add_systems(
                OnEnter(RunPhase::LevelUp),
                spawn_skill_select_ui.in_set(GameSet::Ui),
            )
            .add_systems(
                OnExit(RunPhase::LevelUp),
                despawn_skill_select_ui.in_set(GameSet::Ui),
            )
            .add_systems(
                Update,
                skill_select_keyboard
                    .run_if(in_state(RunPhase::LevelUp))
                    .in_set(GameSet::Ui),
            );
    }
}

fn spawn_skill_select_ui(
    mut commands: Commands,
    mut choices: ResMut<SkillChoices>,
    mut pending: ResMut<PendingLevelUps>,
) {
    // 允许调试或其他玩法直接切入 LevelUp，也能正常完成一次选择。
    pending.0 = pending.0.max(1);
    choices.0 = SkillId::random_choices();
    spawn_choices(&mut commands, choices.0);
}

fn despawn_skill_select_ui(mut commands: Commands, roots: Query<Entity, With<SkillSelectRoot>>) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

fn skill_select_keyboard(keys: Res<ButtonInput<KeyCode>>, mut params: SkillSelectionParams) {
    if keys.just_pressed(KeyCode::KeyF) {
        refresh_choices(&mut params.commands, &mut params.choices, &params.roots);
        return;
    }

    let choice_index = if keys.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else {
        None
    };

    if let Some(index) = choice_index {
        let skill = params.choices.0[index];
        select_skill(skill, &mut params);
    }
}

fn select_skill(skill: SkillId, params: &mut SkillSelectionParams) {
    if params.pending.0 == 0 {
        return;
    }

    let Ok(player) = params.players.single() else {
        error!("选择技能时找不到唯一玩家实体");
        return;
    };

    let properties = skill.properties();
    let abilities = skill.abilities();
    let needs_weapon = skill.properties_affect_weapon() || !abilities.is_empty();
    let weapon = needs_weapon.then(|| {
        params
            .weapons
            .iter()
            .find_map(|(entity, parent)| (parent.parent() == player).then_some(entity))
    });

    let weapon = match weapon {
        Some(Some(weapon)) => Some(weapon),
        Some(None) => {
            error!("技能 {} 需要武器，但玩家没有武器实体", skill.name());
            return;
        }
        None => None,
    };

    if !properties.is_empty() {
        params.property_writer.write(PropertyChangeMessage {
            source: player,
            effect_entity: weapon
                .filter(|_| skill.properties_affect_weapon())
                .unwrap_or(player),
            effect_property: properties,
        });
    }

    if !abilities.is_empty() {
        params.ability_writer.write(AbilityChangeMessage {
            source: player,
            effect_entity: weapon.expect("已校验需要武器的技能"),
            effect_property: abilities,
        });
    }

    info!("玩家选择技能：{}", skill.name());
    params.pending.0 = params.pending.0.saturating_sub(1);
    if params.pending.0 == 0 {
        params.next_phase.set(RunPhase::Playing);
    } else {
        refresh_choices(&mut params.commands, &mut params.choices, &params.roots);
    }
}

fn refresh_choices(
    commands: &mut Commands,
    choices: &mut SkillChoices,
    roots: &Query<Entity, With<SkillSelectRoot>>,
) {
    for entity in roots {
        commands.entity(entity).despawn();
    }

    choices.0 = SkillId::random_choices();
    spawn_choices(commands, choices.0);
}

fn spawn_choices(commands: &mut Commands, choices: [SkillId; CHOICE_COUNT]) {
    commands.spawn_scene(skill_select_scene(choices));
}

fn skill_select_scene(choices: [SkillId; CHOICE_COUNT]) -> impl Scene {
    bsn! {
        SkillSelectRoot
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            right: px(0),
            top: px(0),
            bottom: px(0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(24),
        }
        ZIndex(100)
        BackgroundColor(Color::srgba(0.015, 0.02, 0.04, 0.88))
        Children [
            (
                Text::new("选择一项升级")
                TextFont { font_size: FontSize::Px(46.0), }
                TextColor(Color::srgb(1.0, 0.86, 0.35))
            ),
            (
                Text::new("按 1 / 2 / 3 或点击卡片选择")
                TextFont { font_size: FontSize::Px(17.0), }
                TextColor(Color::srgb(0.72, 0.76, 0.84))
            ),
            (
                Node {
                    width: Val::Percent(92.0),
                    max_width: px(1050),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Stretch,
                    column_gap: px(22),
                }
                Children [
                    skill_card(choices[0], 0),
                    skill_card(choices[1], 1),
                    skill_card(choices[2], 2),
                ]
            ),
            refresh_button(),
        ]
    }
}

fn skill_card(skill: SkillId, index: usize) -> impl Scene {
    let title = format!("{}. {}", index + 1, skill.name());

    bsn! {
        {
            ButtonBuilder::builder()
                .button_style(ButtonStyle {
                    normal: Color::srgb(0.12, 0.17, 0.28),
                    hovered: Color::srgb(0.18, 0.27, 0.43),
                    pressed: Color::srgb(0.08, 0.12, 0.21),
                    ..default()
                })
                .label(bsn! {
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: px(18),
                        padding: UiRect::all(px(18)),
                    }
                    Children [
                        (
                            Text::new(title)
                            TextFont { font_size: FontSize::Px(27.0), }
                            TextColor(Color::WHITE)
                        ),
                        (
                            Text::new(skill.description())
                            TextFont { font_size: FontSize::Px(18.0), }
                            TextColor(Color::srgb(0.76, 0.84, 0.98))
                        ),
                    ]
                })
                .build()
        }
        Node {
            width: px(310),
            height: px(220),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(16)),
        }
        on(move |_event: On<Activate>, mut params: SkillSelectionParams| {
            select_skill(skill, &mut params);
        })
    }
}

fn refresh_button() -> impl Scene {
    bsn! {
        {
            ButtonBuilder::builder()
                .button_style(ButtonStyle {
                    normal: Color::srgb(0.25, 0.29, 0.38),
                    hovered: Color::srgb(0.34, 0.39, 0.50),
                    pressed: Color::srgb(0.18, 0.21, 0.29),
                    ..default()
                })
                .label(bsn! {
                    Text::new("刷新选项  [F]")
                    TextFont { font_size: FontSize::Px(19.0), }
                    TextColor(Color::WHITE)
                })
                .build()
        }
        Node {
            width: px(210),
            height: px(50),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(10)),
        }
        on(|_event: On<Activate>,
            mut commands: Commands,
            mut choices: ResMut<SkillChoices>,
            roots: Query<Entity, With<SkillSelectRoot>>| {
            refresh_choices(&mut commands, &mut choices, &roots);
        })
    }
}
