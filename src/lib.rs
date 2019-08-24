mod game;
mod resources;

mod webpage;
mod viewport;
mod render;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use game::Game;

use webpage::Webpage;
use viewport::Viewport;
use render::Render;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = Game::new();

    let mut webpage = Webpage;
    let mut viewport = Viewport;
    let mut render = Render::default();

    game.setup(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut viewport, world);
        System::setup(&mut render, world);
    });

    game.run(move |_world| {
        // update
    }, move |world| {
        webpage.run_now(world);
        viewport.run_now(world);
        render.run_now(world);
    });
}
