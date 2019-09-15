use specs::prelude::*;
use crate::resources::NameIndex;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Material {
    pub model: Entity
}

impl Material {
    pub fn find(index: &NameIndex, name: &str) -> Self {
        let key = format!("material_{}", name);
        let model = *index.get(&key).unwrap();

        Self { model }
    }
}
