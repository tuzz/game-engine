use specs::prelude::*;
use crate::components::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        ReadStorage<'a, Camera>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, (cameras, mut transforms): Self::SystemData) {
        for (_, transform) in (!&cameras, &mut transforms).join() {
            transform.z_rotate_mut(0.007);
        }
    }
}
