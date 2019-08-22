use specs::prelude::*;

pub struct Ticks {
    pub count: u32,
}

impl Component for Ticks {
    type Storage = VecStorage<Self>;
}
