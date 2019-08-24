use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Coloring {
    pub model: Entity,
}
