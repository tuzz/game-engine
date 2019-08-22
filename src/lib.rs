mod game;
mod render;
mod resources;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    game::run(|_world| {
        // update
    }, |_world| {
        // render
    });
}
