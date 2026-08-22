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

#[derive(Resource)]
pub struct GameMeshAssets {
    pub circle: Handle<Mesh>,
    pub square: Handle<Mesh>,
    pub mat_bg: Handle<ColorMaterial>,
    pub mat_player: Handle<ColorMaterial>,
    pub mat_melee: Handle<ColorMaterial>,
    pub mat_ranged: Handle<ColorMaterial>,
    pub mat_elite: Handle<ColorMaterial>,
    pub mat_boss: Handle<ColorMaterial>,
    pub mat_arrow: Handle<ColorMaterial>,
    pub mat_enemy_shot: Handle<ColorMaterial>,
    pub mat_coin: Handle<ColorMaterial>,
    pub mat_heart: Handle<ColorMaterial>,
}

impl FromWorld for GameMeshAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            circle: world.resource_mut::<Assets<Mesh>>().add(Circle::new(1.0)),
            square: world
                .resource_mut::<Assets<Mesh>>()
                .add(Rectangle::new(1.0, 1.0)),
            mat_bg: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.10, 0.11, 0.14)),
            mat_player: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.25, 0.85, 0.95)),
            mat_melee: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.88, 0.26, 0.28)),
            mat_ranged: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.95, 0.55, 0.20)),
            mat_elite: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.95, 0.85, 0.25)),
            mat_boss: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.62, 0.22, 0.78)),
            mat_arrow: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.95, 0.92, 0.50)),
            mat_enemy_shot: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.95, 0.30, 0.60)),
            mat_coin: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(1.0, 0.84, 0.20)),
            mat_heart: world
                .resource_mut::<Assets<ColorMaterial>>()
                .add(Color::srgb(0.95, 0.30, 0.45)),
        }
    }
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
            LoadingState::new(GameState::StartupLoading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<StartupAssets>()
                .on_failure_continue_to_state(GameState::AssetLoadingError),
        )
        .add_loading_state(
            LoadingState::new(GameState::GameLoading)
                .continue_to_state(GameState::InGame)
                .load_collection::<GameAssets>()
                .finally_init_resource::<GameMeshAssets>()
                .on_failure_continue_to_state(GameState::AssetLoadingError),
        )
        .add_systems(OnExit(GameState::InGame), unload_game_assets);
    }
}

fn unload_game_assets(mut commands: Commands) {
    commands.remove_resource::<GameAssets>();
}
