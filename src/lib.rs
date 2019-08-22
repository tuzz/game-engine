mod game;

mod components;
mod resources;

mod render;
mod ticker;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use game::Game;

use ticker::Ticker;
use render::Render;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = Game::new();

    let mut ticker = Ticker;
    let mut render = Render::default();

    game.setup(|world| {
        System::setup(&mut ticker, world);
        System::setup(&mut render, world);
    });

    game.run(move |world| {
        ticker.run_now(world);
    }, move |world| {
        render.run_now(world);
    });
}
