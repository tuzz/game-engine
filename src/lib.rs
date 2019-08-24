mod game_loop;
mod resources;

mod webpage;
mod webgl_viewport;
mod webgl_shader;
mod webgl_program;
mod render;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use game_loop::GameLoop;
use webpage::Webpage;
use webgl_viewport::WebGlViewport;
use webgl_shader::WebGlShader;
use webgl_program::WebGlProgram;
use render::Render;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut webgl_viewport = WebGlViewport;
    let mut webgl_shader = WebGlShader;
    let mut webgl_program = WebGlProgram;
    let mut render = Render::default();

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut webgl_viewport, world);
        System::setup(&mut webgl_shader, world);
        System::setup(&mut webgl_program, world);
        System::setup(&mut render, world);
    });

    game_loop.run(move |_world| {
        // update
    }, move |world| {
        webgl_viewport.run_now(world);
        render.run_now(world);
    });
}
