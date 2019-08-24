mod game_loop;
mod resources;

mod webpage;
mod viewport;
mod shader;
mod program;
mod render;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use game_loop::GameLoop;
use webpage::Webpage;
use viewport::Viewport;
use shader::Shader;
use program::Program;
use render::Render;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut viewport = Viewport;
    let mut shader = Shader;
    let mut program = Program;
    let mut render = Render::default();

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut viewport, world);
        System::setup(&mut shader, world);
        System::setup(&mut program, world);
        System::setup(&mut render, world);
    });

    game_loop.run(move |_world| {
        // update
    }, move |world| {
        viewport.run_now(world);
        render.run_now(world);
    });
}
