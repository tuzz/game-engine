use specs::prelude::*;
use crate::resources::NameIndex;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Texture {
    pub model: Entity,
}

impl Texture {
    pub fn find(index: &NameIndex, path: &str) -> Self {
        Self { model: *index.get(&path).unwrap() }
    }
}
