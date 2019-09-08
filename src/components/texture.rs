use specs::prelude::*;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Texture {
    pub model: Entity,
}
