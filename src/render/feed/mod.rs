use super::{GL, Buffer, Program};

pub fn feed_attribute(context: &GL, program: &Program, name: &str, buffer: &Buffer, size: i32) {
    let location = program.attribute_location(name);

    context.enable_vertex_attrib_array(location);
    buffer.bind(context);
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

pub fn feed_uniform(context: &GL, program: &Program, name: &str, matrix: &[f32]) {
    let location = program.uniform_location(name);

    context.uniform_matrix4fv_with_f32_array(Some(&location), false, matrix);
}
