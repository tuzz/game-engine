use specs::prelude::*;
use crate::components::*;

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, SceneParent>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, (cameras, scene_parents, mut transforms): Self::SystemData) {
        for (_, _, transform) in (!&cameras, !&scene_parents, &mut transforms).join() {
            transform.x_rotate_mut(0.02).y_rotate_mut(0.01);
        }
    }
}
