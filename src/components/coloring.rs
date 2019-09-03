use specs::prelude::*;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Coloring {
    pub model: Entity,
}
