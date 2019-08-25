use specs::prelude::*;
use crate::components::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = WriteStorage<'a, Transform>;

    fn run(&mut self, mut transforms: Self::SystemData) {
        for transform in (&mut transforms).join() {
            transform.x_rotate_mut(0.01).y_rotate_mut(0.02);
        }
    }
}
