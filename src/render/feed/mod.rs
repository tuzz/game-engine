use super::{GL, WebGlBuffer};
use crate::resources::shader_types::*;

pub fn feed_attribute(context: &GL, program: &ShaderProgram, name: &str, buffer: &WebGlBuffer, size: i32) {
    let location = program.attribute_location(name);

    context.enable_vertex_attrib_array(location);
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

pub fn feed_uniform(context: &GL, program: &ShaderProgram, name: &str, matrix: &[f32]) {
    let location = program.uniform_location(name);

    context.uniform_matrix4fv_with_f32_array(Some(&location), false, matrix);
}
