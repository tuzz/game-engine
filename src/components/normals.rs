use specs::prelude::*;

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
