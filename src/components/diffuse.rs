use specs::prelude::*;
use std::ops::Deref;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Diffuse(pub Vector3f);

impl Default for Diffuse {
    fn default() -> Self {
        Self(Vector3f::new(1.0, 1.0, 1.0))
    }
}

impl Deref for Diffuse {
    type Target = Vector3f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
