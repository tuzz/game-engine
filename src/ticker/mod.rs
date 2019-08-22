use specs::prelude::*;
use super::components::Ticks;

pub struct Ticker;

impl<'a> System<'a> for Ticker {
    type SystemData = WriteStorage<'a, Ticks>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        world.create_entity().with(Ticks { count: 0 }).build();
        world.maintain();
    }

    fn run(&mut self, mut ticks: Self::SystemData) {
        for ticks in (&mut ticks).join() {
            ticks.count += 1;
        }
    }
}
