use specs::prelude::*;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Material {
    pub model: Entity
}
