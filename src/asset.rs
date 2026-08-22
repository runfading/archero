use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct StartupAssets {
    #[asset(path = "fonts/SourceHanSerifCN-Medium.otf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // #[asset(path = "images/player.png")]
    // pub player: Handle<Image>,
    //
    // #[asset(path = "images/enemy.png")]
    // pub enemy: Handle<Image>,
    //
    // #[asset(path = "images/arrow.png")]
    // pub arrow: Handle<Image>,
    //
    // #[asset(path = "audio/battle.ogg")]
    // pub battle_music: Handle<AudioSource>,
    //
    // #[asset(path = "audio/hit.ogg")]
    // pub hit_sound: Handle<AudioSource>,
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::StartupLoading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<StartupAssets>()
                .on_failure_continue_to_state(GameState::AssetLoadingError),
        )
        .add_loading_state(
            LoadingState::new(GameState::GameLoading)
                .continue_to_state(GameState::InGame)
                .load_collection::<GameAssets>()
                .on_failure_continue_to_state(GameState::AssetLoadingError),
        )
        .add_systems(OnExit(GameState::InGame), unload_game_assets);
    }
}

fn unload_game_assets(mut commands: Commands) {
    commands.remove_resource::<GameAssets>();
}
