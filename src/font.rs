use crate::GameState;
use crate::asset::StartupAssets;
use bevy::prelude::*;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        // 替换全局默认字体
        app.add_systems(OnEnter(GameState::MainMenu), install_default_font);
    }
}

fn install_default_font(startup_assets: Res<StartupAssets>, mut fonts: ResMut<Assets<Font>>) {
    let font = fonts
        .get(&startup_assets.font)
        .cloned()
        .expect("启动字体已加载，但 Assets<Font> 中找不到该字体");

    fonts
        .insert(AssetId::default(), font)
        .expect("替换全局默认字体失败");
}
