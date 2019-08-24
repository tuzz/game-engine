use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::resources::{*, shader_types::*};
use crate::components::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadExpect<'a, WebGlContext>,
        ReadExpect<'a, ShaderPrograms>,

        ReadStorage<'a, Geometry>,
        ReadStorage<'a, Coloring>,
        ReadStorage<'a, WebGlBuffer>,
        ReadStorage<'a, Dimensions>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        let (context, programs, geometries, colorings, webgl_buffers, dimensions) = system_data;

        let program = &programs.default;
        context.use_program(Some(&program.compiled));

        for (geometry, coloring) in (&geometries, &colorings).join() {
            let geometry_buffer = webgl_buffers.get(geometry.model).unwrap();
            let coloring_buffer = webgl_buffers.get(coloring.model).unwrap();

            let geometry_dimensions = dimensions.get(geometry.model).unwrap();
            let coloring_dimensions = dimensions.get(coloring.model).unwrap();

            feed_attribute(&context, program, "a_position", geometry_buffer, **geometry_dimensions as i32);
            feed_attribute(&context, program, "a_color", coloring_buffer, **coloring_dimensions as i32);

            feed_uniform(&context, program, "u_matrix", &[
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]);

            let elements = geometry_buffer.len / **geometry_dimensions as usize;
            context.draw_arrays(GL::TRIANGLES, 0, elements as i32);
        }
    }
}

pub fn feed_attribute(context: &GL, program: &ShaderProgram, name: &str, buffer: &WebGlBuffer, size: i32) {
    let location = program.attribute_map.get(name).unwrap().to_owned();

    context.enable_vertex_attrib_array(location);
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

pub fn feed_uniform(context: &GL, program: &ShaderProgram, name: &str, matrix: &[f32]) {
    let location = program.uniform_map.get(name).unwrap().to_owned();

    context.uniform_matrix4fv_with_f32_array(Some(&location), false, matrix);
}
