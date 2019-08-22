use specs::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use super::resources::Timing;

pub fn run<U, R>(update: U, render: R)
    where U: Fn(&mut World) + 'static, R: Fn(&mut World) + 'static
{
    let mut world = World::new();

    world.insert(Timing::default());

    frame(world, current_time(), update, render);
}

fn frame<U, R>(mut world: World, previous: f64, update: U, render: R)
    where U: Fn(&mut World) + 'static, R: Fn(&mut World) + 'static
{
    let mut t = timing_resource(&mut world);

    let current = current_time();
    let elapsed = current - previous;

    t.time_since_update += elapsed;

    if t.time_since_update > t.pause_updates_after {
        t.time_since_update = t.pause_updates_after;
    }

    while t.time_since_update >= t.fixed_update_time {
        update(&mut world);

        t = timing_resource(&mut world);
        t.time_since_update -= t.fixed_update_time;
    }
    render(&mut world);

    request_animation_frame(move || {
        frame(world, current, update, render);
    });
}

fn timing_resource(world: &mut World) -> &mut Timing {
    world.get_mut::<Timing>().unwrap()
}

fn request_animation_frame<F: FnOnce() + 'static>(callback: F) {
    let window = web_sys::window().unwrap();
    let closure = Closure::once_into_js(callback);
    let js_func = closure.as_ref().unchecked_ref();

    window.request_animation_frame(js_func).unwrap();
}

fn current_time() -> f64 {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();

    performance.now() / 1000.0
}
