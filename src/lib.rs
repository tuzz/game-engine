#[macro_use]
extern crate specs_derive;

mod game_loop;
mod resources;
mod components;

mod webpage;
mod webgl_viewport;
mod webgl_shader;
mod webgl_program;
mod webgl_buffer;
mod render;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use game_loop::GameLoop;
use webpage::Webpage;
use webgl_viewport::WebGlViewport;
use webgl_shader::WebGlShader;
use webgl_program::WebGlProgram;
use webgl_buffer::WebGlBuffer;
use render::Render;

use components::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut webgl_viewport = WebGlViewport;
    let mut webgl_shader = WebGlShader;
    let mut webgl_program = WebGlProgram;
    let mut webgl_buffer = WebGlBuffer;
    let mut render = Render::default();

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut webgl_viewport, world);
        System::setup(&mut webgl_shader, world);
        System::setup(&mut webgl_program, world);
        System::setup(&mut webgl_buffer, world);
        System::setup(&mut render, world);

        let geometry_model = world.create_entity().with(BufferData(vec![
            -1.0, 0.0,
            -0.5, 0.0,
            -1.0, 0.5,
        ])).build();

        let coloring_model = world.create_entity().with(BufferData(vec![
            1.0, 0.5, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 1.0, 1.0,
        ])).build();

        world.create_entity()
            .with(Geometry { model: geometry_model })
            .with(Coloring { model: coloring_model })
            .build();
    });

    game_loop.run(move |_world| {
        // update
    }, move |world| {
        webgl_viewport.run_now(world);
        webgl_buffer.run_now(world);
        render.run_now(world);
    });
}
