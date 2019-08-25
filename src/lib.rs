#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate impl_ops;

#[macro_use] #[cfg(test)]
extern crate assert_approx_eq;

mod components;
mod resources;
mod systems;
mod utilities;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use utilities::GameLoop;
use utilities::Matrix4f;
use components::*;
use systems::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut webgl_viewport = WebGlViewport;
    let mut webgl_shader = WebGlShader;
    let mut webgl_program = WebGlProgram;
    let mut webgl_buffer = WebGlBuffer;
    let mut webgl_render = WebGlRender;
    let mut animation = Animation;

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut webgl_viewport, world);
        System::setup(&mut webgl_shader, world);
        System::setup(&mut webgl_program, world);
        System::setup(&mut webgl_buffer, world);
        System::setup(&mut webgl_render, world);
        System::setup(&mut animation, world);

        let geometry_model = world.create_entity().with(BufferData(vec![
            // Front
            -1., -1., -1.,
             1., -1., -1.,
             1.,  1., -1.,

             1.,  1., -1.,
            -1.,  1., -1.,
            -1., -1., -1.,

            // Back
            -1., -1.,  1.,
             1.,  1.,  1.,
             1., -1.,  1.,

             1.,  1.,  1.,
            -1., -1.,  1.,
            -1.,  1.,  1.,

            // Left
            -1., -1., -1.,
            -1.,  1., -1.,
            -1., -1.,  1.,

            -1., -1.,  1.,
            -1.,  1., -1.,
            -1.,  1.,  1.,

            // Right
             1., -1., -1.,
             1., -1.,  1.,
             1.,  1.,  1.,

             1.,  1.,  1.,
             1.,  1., -1.,
             1., -1., -1.,

            // Bottom
            -1., -1., -1.,
            -1., -1.,  1.,
             1., -1., -1.,

             1., -1., -1.,
            -1., -1.,  1.,
             1., -1.,  1.,

            // Top
            -1.,  1., -1.,
             1.,  1., -1.,
             1.,  1.,  1.,

             1.,  1.,  1.,
            -1.,  1.,  1.,
            -1.,  1., -1.,
        ])).with(Dimensions(3)).build();

        let coloring_model = world.create_entity().with(BufferData(vec![
            // Front
            0., 0., 0.,
            0., 0., 0.,
            0., 0., 0.,

            0., 0., 1.,
            0., 0., 1.,
            0., 0., 1.,

            // Back
            0., 1., 0.,
            0., 1., 0.,
            0., 1., 0.,

            0., 1., 1.,
            0., 1., 1.,
            0., 1., 1.,

            // Left
            1., 0., 0.,
            1., 0., 0.,
            1., 0., 0.,

            1., 0., 1.,
            1., 0., 1.,
            1., 0., 1.,

            // Right
            1., 1., 0.,
            1., 1., 0.,
            1., 1., 0.,

            0., 0., 0.5,
            0., 0., 0.5,
            0., 0., 0.5,

            // Bottom
            0., 0.5, 0.,
            0., 0.5, 0.,
            0., 0.5, 0.,

            0.5, 0., 0.,
            0.5, 0., 0.,
            0.5, 0., 0.,

            // Top
            0.5, 0., 1.,
            0.5, 0., 1.,
            0.5, 0., 1.,

            1., 0.5, 0.,
            1., 0.5, 0.,
            1., 0.5, 0.,
        ])).with(Dimensions(3)).build();

        world.create_entity()
            .with(Geometry { model: geometry_model })
            .with(Coloring { model: coloring_model })
            .with(Transform(Matrix4f::scaling(0.4, 0.4, 0.4)))
            .build();
    });

    game_loop.run(move |world| {
        animation.run_now(world);
    }, move |world| {
        webgl_viewport.run_now(world);
        webgl_buffer.run_now(world);
        webgl_render.run_now(world);
    });
}
