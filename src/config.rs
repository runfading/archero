use serde::Deserialize;

#[derive(Deserialize)]
pub struct StartUpConfig {
    pub title: String,
    pub min_window_width: f32,
    pub min_window_height: f32,
    pub resolution: (u32, u32),
}

impl StartUpConfig {
    /// 加载启动配置
    pub fn load() -> Self {
        ron::from_str(include_str!("../assets/config/start_config.ron"))
            .expect("启动配置读取错误！")
    }
}
