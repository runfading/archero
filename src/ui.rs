use crate::ui::main_menu_ui::MainMenuUiPlugin;
use crate::ui::playing_ui::PlayingUiPlugin;
use bevy::prelude::*;

mod main_menu_ui;
mod playing_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuUiPlugin)
            .add_plugins(PlayingUiPlugin);
    }
}
