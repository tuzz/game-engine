use specs::prelude::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ClearColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ClearColor {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0., 1.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1., 1.)
    }

    pub fn transparent() -> Self {
        Self::new(0., 0., 0., 0.)
    }
}
