use specs::prelude::*;
use crate::resources::GameTiming;
use crate::utilities::*;

pub struct GameLoop {
    pub world: World,
}

impl GameLoop {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(GameTiming::default());

        Self { world }
    }

    pub fn before<C: FnMut(&mut World)>(&mut self, mut callback: C) {
        callback(&mut self.world);
    }

    pub fn run<U, R>(self, update: U, render: R)
        where U: FnMut(&mut World) + 'static, R: FnMut(&mut World) + 'static,
    {
        frame(self.world, current_time(), update, render);
    }
}


fn frame<U, R>(mut world: World, previous: f64, mut update: U, mut render: R)
    where U: FnMut(&mut World) + 'static, R: FnMut(&mut World) + 'static
{
    let mut t = game_timing(&mut world);

    let current = current_time();
    let elapsed = current - previous;

    t.time_since_update += elapsed;

    if t.time_since_update > t.pause_updates_after {
        t.time_since_update = t.pause_updates_after;
    }

    while t.time_since_update >= t.fixed_update_time() {
        update(&mut world);

        t = game_timing(&mut world);
        t.time_since_update -= t.fixed_update_time();
    }
    render(&mut world);

    let next_frame = move |_| frame(world, current, update, render);

    single_use_handler(next_frame, |c| {
        web_sys::window().unwrap().request_animation_frame(c).unwrap();
    });
}

fn game_timing(world: &mut World) -> &mut GameTiming {
    world.get_mut::<GameTiming>().unwrap()
}

fn current_time() -> f64 {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();

    performance.now() / 1000.0
}
