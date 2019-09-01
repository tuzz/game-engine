#[derive(Eq, PartialEq, Hash, Clone)]
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

    pub fn total_lights(&self) -> u32 {
        self.point_lights + self.directional_lights + self.spot_lights
    }
}

impl Default for ShaderConfig {
    fn default() -> Self {
        ShaderConfig { point_lights: 0, directional_lights: 3, spot_lights: 0 }
    }
}
