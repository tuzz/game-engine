use specs::prelude::*;
use crate::resources::NameIndex;

#[derive(Component, Clone, Eq, PartialEq, Debug)]
#[storage(VecStorage)]
pub struct Normals {
    pub model: Entity,
}

impl Normals {
    pub fn new(model: Entity) -> Self {
        Self { model }
    }
}

impl Normals {
    pub fn find(index: &NameIndex, name: &str) -> Self {
        let key = format!("normals_{}", name);
        let model = *index.get(&key).unwrap();

        Self { model }
    }
}
