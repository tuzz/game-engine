use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Geometry {
    pub model: Entity,
}
