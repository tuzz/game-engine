use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::resources::{*, shader_types::*};
use crate::components::*;
use crate::utilities::*;

pub struct WebGlRender;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,

    context: ReadExpect<'a, WebGlContext>,
    programs: ReadExpect<'a, ShaderPrograms>,

    cameras: ReadStorage<'a, Camera>,
    projections: ReadStorage<'a, ProjectionTransform>,
    viewports: ReadStorage<'a, Viewport>,
    clear_colors: ReadStorage<'a, ClearColor>,

    world_transforms: ReadStorage<'a, WorldTransform>,
    inverse_transforms: ReadStorage<'a, InverseWorldTransform>,
    geometries: ReadStorage<'a, Geometry>,
    normals: ReadStorage<'a, Normals>,
    colorings: ReadStorage<'a, Coloring>,

    directional_light: ReadStorage<'a, DirectionalLight>,

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
        let locations = shader_program_locations(&program);

        s.context.use_program(Some(&program.compiled));

        let directional_light = s.directional_light.join().next().unwrap();

        for (entity, _camera, viewport, clear_color, projection) in (
            &s.entities, &s.cameras, &s.viewports, &s.clear_colors, &s.projections
        ).join() {
            let view = s.inverse_transforms.get(entity).unwrap();
            let view_projection = projection.multiply(&view);

            clear_viewport(&s.context, viewport, clear_color);

            for (geometry, normals, coloring, world_transform, inverse_world) in (
                &s.geometries, &s.normals, &s.colorings, &s.world_transforms, &s.inverse_transforms
            ).join() {
                let world_view_projection = view_projection.multiply(&world_transform);

                set_uniform_from_matrix(&s.context, &locations.u_world_view_projection, &world_view_projection);
                set_uniform_from_matrix(&s.context, &locations.u_inverse_world, &inverse_world);
                set_uniform_from_vector(&s.context, &locations.u_reverse_light_direction, &directional_light.reverse_light_direction);

                set_attribute_from_model(&s, locations.a_position, geometry.model);
                set_attribute_from_model(&s, locations.a_normal, normals.model);
                set_attribute_from_model(&s, locations.a_color, coloring.model);

                s.context.draw_arrays(GL::TRIANGLES, 0, number_of_elements(&s, geometry.model));
            }
        }
    }
}

fn shader_program_locations(program: &ShaderProgram) -> ShaderProgramLocations {
    ShaderProgramLocations {
        a_position: *program.attribute_map.get("a_position").unwrap(),
        a_normal: *program.attribute_map.get("a_normal").unwrap(),
        a_color: *program.attribute_map.get("a_color").unwrap(),

        u_world_view_projection: program.uniform_map.get("u_world_view_projection").unwrap().to_owned(),
        u_inverse_world: program.uniform_map.get("u_inverse_world").unwrap().to_owned(),
        u_reverse_light_direction: program.uniform_map.get("u_reverse_light_direction").unwrap().to_owned(),
    }
}

struct ShaderProgramLocations {
    a_position: AttributeLocation,
    a_normal: AttributeLocation,
    a_color: AttributeLocation,

    u_world_view_projection: UniformLocation,
    u_inverse_world: UniformLocation,
    u_reverse_light_direction: UniformLocation,
}

fn clear_viewport(context: &GL, viewport: &Viewport, clear_color: &ClearColor) {
    context.viewport(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
    context.scissor(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);

    context.clear_color(clear_color.red, clear_color.green, clear_color.blue, clear_color.alpha);
    context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
}

fn set_attribute_from_model(s: &SysData, location: AttributeLocation, model: Entity) {
    let buffer = s.buffers.get(model).unwrap();
    let dimensions = s.dimensions.get(model).unwrap();

    set_attribute_from_buffer(&s.context, location, buffer, **dimensions as i32);
}

fn set_attribute_from_buffer(context: &GL, location: AttributeLocation, buffer: &WebGlBuffer, size: i32) {
    context.enable_vertex_attrib_array(location);
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(location, size, GL::FLOAT, false, 0, 0);
}

fn set_uniform_from_matrix(context: &GL, location: &UniformLocation, matrix: &[f32; 16]) {
    context.uniform_matrix4fv_with_f32_array(Some(location), false, matrix);
}

fn set_uniform_from_vector(context: &GL, location: &UniformLocation, vector: &Vector3f) {
    context.uniform3fv_with_f32_array(Some(location), &[vector.x, vector.y, vector.z]);
}

fn number_of_elements(s: &SysData, model: Entity) -> i32 {
    let buffer = s.buffers.get(model).unwrap();
    let dimensions = s.dimensions.get(model).unwrap();

    (buffer.len / **dimensions as usize) as i32
}
