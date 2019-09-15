use specs::prelude::*;
use std::ops::Deref;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Ambient(pub Vector3f);

impl Default for Ambient {
    fn default() -> Self {
        Self(Vector3f::new(0.1, 0.1, 0.1))
    }
}

impl Deref for Ambient {
    type Target = Vector3f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
