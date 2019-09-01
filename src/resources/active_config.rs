use std::ops::Deref;
use super::*;

#[derive(Default)]
pub struct ActiveConfig {
    pub config: ShaderConfig
}

impl Deref for ActiveConfig {
    type Target = ShaderConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
