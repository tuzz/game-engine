use specs::prelude::*;
use crate::components::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        ReadStorage<'a, Camera>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, (cameras, mut transforms): Self::SystemData) {
        for ((), transform) in (!&cameras, &mut transforms).join() {
            transform.x_rotate_mut(0.02).y_rotate_mut(0.01);
        }
    }
}
