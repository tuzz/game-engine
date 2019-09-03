#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ShaderConfig {
    pub point_lights: u32,
    pub directional_lights: u32,
}

impl ShaderConfig {
    pub fn combinations(&self) -> Vec<Self> {
        (0..=self.point_lights).flat_map(|p| {
            (0..=self.directional_lights).map(move |d| {
                Self { point_lights: p, directional_lights: d }
            })
        }).collect()
    }

    pub fn total_lights(&self) -> u32 {
        self.point_lights + self.directional_lights
    }
}

impl ShaderConfig {
    pub fn no_lights() -> Self {
        ShaderConfig { point_lights: 0, directional_lights: 0 }
    }

    pub fn one_of_each_light() -> Self {
        ShaderConfig { point_lights: 1, directional_lights: 1 }
    }

    pub fn a_few_lights() -> Self {
        ShaderConfig { point_lights: 3, directional_lights: 2 }
    }

    pub fn lots_of_lights() -> Self {
        ShaderConfig { point_lights: 8, directional_lights: 3 }
    }
}

impl Default for ShaderConfig {
    fn default() -> Self {
        Self::a_few_lights()
    }
}
