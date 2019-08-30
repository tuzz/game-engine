use specs::prelude::*;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Normals {
    pub model: Entity,
}

impl Normals {
    pub fn new(model: Entity) -> Self {
        Self { model }
    }
}
