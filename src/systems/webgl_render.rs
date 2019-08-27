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
        context.enable(GL::CULL_FACE);
        context.enable(GL::DEPTH_TEST);
        context.enable(GL::SCISSOR_TEST);

        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    }

    fn run(&mut self, s: Self::SystemData) {
        let program = &s.programs.default;

        let (a_position, a_color, u_matrix) = lookup_locations(&program);

        s.context.use_program(Some(&program.compiled));

        for (_camera, viewport, clear_color, transform, projection) in (&s.cameras, &s.viewports, &s.clear_colors, &s.transforms, &s.projections).join() {
            let view = transform.inverse();
            let view_projection = projection.multiply(&view);

            clear_viewport(&s.context, viewport, clear_color);

            for (geometry, coloring, transform) in (&s.geometries, &s.colorings, &s.transforms).join() {
                let model_view_projection = view_projection.multiply(&transform);
                set_uniform(&s.context, &u_matrix, &model_view_projection);

                let geometry_buffer = s.buffers.get(geometry.model).unwrap();
                let coloring_buffer = s.buffers.get(coloring.model).unwrap();

                let geometry_dimensions = s.dimensions.get(geometry.model).unwrap();
                let coloring_dimensions = s.dimensions.get(coloring.model).unwrap();

                feed_attribute(&s.context, a_position, geometry_buffer, **geometry_dimensions as i32);
                feed_attribute(&s.context, a_color, coloring_buffer, **coloring_dimensions as i32);


                let elements = geometry_buffer.len / **geometry_dimensions as usize;
                s.context.draw_arrays(GL::TRIANGLES, 0, elements as i32);
            }
        }
    }
}

fn clear_viewport(context: &GL, viewport: &Viewport, clear_color: &ClearColor) {
    context.viewport(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
    context.scissor(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);

    context.clear_color(clear_color.red, clear_color.green, clear_color.blue, clear_color.alpha);
    context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
}

fn lookup_locations(program: &ShaderProgram) -> (AttributeLocation, AttributeLocation, UniformLocation) {
    let a_position = *program.attribute_map.get("a_position").unwrap();
    let a_color = *program.attribute_map.get("a_color").unwrap();
    let u_matrix = program.uniform_map.get("u_matrix").unwrap().to_owned();

    (a_position, a_color, u_matrix)
}

fn feed_attribute(context: &GL, location: AttributeLocation, buffer: &WebGlBuffer, size: i32) {
    context.enable_vertex_attrib_array(location);
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

fn set_uniform(context: &GL, location: &UniformLocation, matrix: &[f32; 16]) {
    context.uniform_matrix4fv_with_f32_array(Some(location), false, matrix);
}
