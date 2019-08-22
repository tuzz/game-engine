mod render;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(render::Render, "render", &[])
        .build();

    dispatcher.setup(&mut world);
    dispatcher.dispatch(&world);

    world.maintain();
}
