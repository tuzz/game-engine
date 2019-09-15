use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Shininess(pub f32);

impl Default for Shininess {
    fn default() -> Self {
        Self(80.)
    }
}

impl Deref for Shininess {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
