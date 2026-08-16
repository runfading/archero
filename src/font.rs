use bevy::prelude::*;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        // 替换全局默认字体
        app.world_mut()
            .resource_mut::<Assets<Font>>()
            .insert(
                AssetId::default(),
                Font::from_bytes(
                    include_bytes!("../assets/fonts/SourceHanSerifCN-Medium.otf").to_vec(),
                ),
            )
            .expect("替换默认字体失败");
    }
}
