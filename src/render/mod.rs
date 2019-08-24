use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use crate::resources::{*, WebGlContext};
use crate::components::*;

mod feed;

use feed::*;

#[derive(Default)]
pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadExpect<'a, WebGlContext>,
        ReadExpect<'a, ShaderPrograms>,

        ReadStorage<'a, Geometry>,
        ReadStorage<'a, Coloring>,
        ReadStorage<'a, WebGlBuffer>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        let (context, programs, geometries, colorings, webgl_buffers) = system_data;

        let program = &programs.default;
        context.use_program(Some(&program.compiled));

        for (geometry, coloring) in (&geometries, &colorings).join() {
            let geometry = webgl_buffers.get(geometry.model).unwrap();
            let coloring = webgl_buffers.get(coloring.model).unwrap();

            feed_attribute(&context, program, "a_position", &geometry, 2); // TODO
            feed_attribute(&context, program, "a_color", &coloring, 4); // TODO

            feed_uniform(&context, program, "u_matrix", &[
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]);

            context.draw_arrays(GL::TRIANGLES, 0, 3); // TODO
        }
    }
}
