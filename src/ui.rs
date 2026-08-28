use crate::ui::game_clear_ui::GameClearUiPlugin;
use crate::ui::game_over_ui::GameOverUiPlugin;
use crate::ui::main_menu_ui::MainMenuUiPlugin;
use crate::ui::playing_ui::PlayingUiPlugin;
use crate::ui::skill_select_ui::SkillSelectUiPlugin;
use bevy::prelude::*;

mod game_clear_ui;
mod game_over_ui;
mod main_menu_ui;
mod playing_ui;
mod skill_select_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuUiPlugin)
            .add_plugins(PlayingUiPlugin)
            .add_plugins(GameOverUiPlugin)
            .add_plugins(GameClearUiPlugin)
            .add_plugins(SkillSelectUiPlugin);
    }
}
