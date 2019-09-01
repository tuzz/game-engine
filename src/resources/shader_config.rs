#[derive(Eq, PartialEq, Hash)]
pub struct ShaderConfig {
    pub point_lights: u32,
    pub directional_lights: u32,
    pub spot_lights: u32,
}

impl ShaderConfig {
    pub fn combinations(&self) -> Vec<Self> {
        (0..=self.point_lights).flat_map(|p| {
            (0..=self.directional_lights).flat_map(move |d| {
                (0..=self.spot_lights).map(move |s| {
                    Self { point_lights: p, directional_lights: d, spot_lights: s }
                })
            })
        }).collect()
    }
}

impl Default for ShaderConfig {
    fn default() -> Self {
        ShaderConfig { point_lights: 3, directional_lights: 1, spot_lights: 1 }
    }
}
