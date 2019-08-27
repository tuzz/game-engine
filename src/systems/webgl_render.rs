use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::resources::{*, shader_types::*};
use crate::components::*;

pub struct WebGlRender;

#[derive(SystemData)]
pub struct SysData<'a> {
    context: ReadExpect<'a, WebGlContext>,
    programs: ReadExpect<'a, ShaderPrograms>,

    cameras: ReadStorage<'a, Camera>,
    projections: ReadStorage<'a, ProjectionTransform>,
    viewports: ReadStorage<'a, Viewport>,
    clear_colors: ReadStorage<'a, ClearColor>,

    transforms: ReadStorage<'a, Transform>,
    geometries: ReadStorage<'a, Geometry>,
    colorings: ReadStorage<'a, Coloring>,

    buffers: ReadStorage<'a, WebGlBuffer>,
    dimensions: ReadStorage<'a, Dimensions>,
}

impl<'a> System<'a> for WebGlRender {
    type SystemData = SysData<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let context = world.fetch::<WebGlContext>();

        context.enable(GL::BLEND);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        context.enable(GL::CULL_FACE);
        context.enable(GL::DEPTH_TEST);
        context.enable(GL::SCISSOR_TEST);
    }

    fn run(&mut self, s: Self::SystemData) {
        let program = &s.programs.default;
        s.context.use_program(Some(&program.compiled));

        for (_camera, viewport, clear_color, transform, projection) in (&s.cameras, &s.viewports, &s.clear_colors, &s.transforms, &s.projections).join() {
            let view = transform.inverse();
            let view_projection = projection.multiply(&view);

            s.context.viewport(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
            s.context.scissor(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);

            let ClearColor(r, g, b, a) = clear_color;
            s.context.clear_color(*r, *g, *b, *a);
            s.context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

            for (geometry, coloring, transform) in (&s.geometries, &s.colorings, &s.transforms).join() {
                let model_view_projection = view_projection.multiply(&transform);

                let geometry_buffer = s.buffers.get(geometry.model).unwrap();
                let coloring_buffer = s.buffers.get(coloring.model).unwrap();

                let geometry_dimensions = s.dimensions.get(geometry.model).unwrap();
                let coloring_dimensions = s.dimensions.get(coloring.model).unwrap();

                feed_attribute(&s.context, program, "a_position", geometry_buffer, **geometry_dimensions as i32);
                feed_attribute(&s.context, program, "a_color", coloring_buffer, **coloring_dimensions as i32);
                feed_uniform(&s.context, program, "u_matrix", &model_view_projection);

                let elements = geometry_buffer.len / **geometry_dimensions as usize;
                s.context.draw_arrays(GL::TRIANGLES, 0, elements as i32);
            }
        }
    }
}

pub fn feed_attribute(context: &GL, program: &ShaderProgram, name: &str, buffer: &WebGlBuffer, size: i32) {
    let location = program.attribute_map.get(name).unwrap().to_owned();

    context.enable_vertex_attrib_array(location);
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

pub fn feed_uniform(context: &GL, program: &ShaderProgram, name: &str, matrix: &[f32; 16]) {
    let location = program.uniform_map.get(name).unwrap().to_owned();

    context.uniform_matrix4fv_with_f32_array(Some(&location), false, matrix);
}
