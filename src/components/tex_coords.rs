use specs::prelude::*;
use crate::resources::NameIndex;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct TexCoords {
    pub model: Entity
}

impl TexCoords {
    pub fn find(index: &NameIndex, name: &str) -> Self {
        let key = format!("texcoords_{}", name);
        let model = *index.get(&key).unwrap();

        Self { model }
    }
}
