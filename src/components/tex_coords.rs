use specs::prelude::*;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct TexCoords {
    pub model: Entity
}
