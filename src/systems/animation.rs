use specs::prelude::*;
use crate::components::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = WriteStorage<'a, Transform>;

    fn run(&mut self, mut transforms: Self::SystemData) {
        for transform in (&mut transforms).join() {
            //transform.translate_mut(0.001, 0., 0.);
            //
            transform.translate_mut(-0.2, -0.2, 0.0)
                     .z_rotate_mut(0.005)
                     .translate_mut(0.2, 0.2, 0.0);
        }
    }
}
