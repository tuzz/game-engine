mod game;
mod render;
mod resources;

use wasm_bindgen::prelude::*;
use game::Game;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = Game::new();

    game.setup(|_world| {
        // setup
    });

    game.run(move |_world| {
        // update
    }, move |_world| {
        // render
    });
}
