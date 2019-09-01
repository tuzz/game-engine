use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

pub struct WebGlRender;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,

    context: ReadExpect<'a, WebGlContext>,
    programs: ReadExpect<'a, ShaderPrograms>,
    locations: ReadExpect<'a, ShaderLocations>,

    cameras: ReadStorage<'a, Camera>,
    projections: ReadStorage<'a, ProjectionTransform>,
    viewports: ReadStorage<'a, Viewport>,
    clear_colors: ReadStorage<'a, ClearColor>,

    world_transforms: ReadStorage<'a, WorldTransform>,
    inverse_transforms: ReadStorage<'a, InverseWorldTransform>,
    geometries: ReadStorage<'a, Geometry>,
    normals: ReadStorage<'a, Normals>,
    colorings: ReadStorage<'a, Coloring>,

    directional_lights: ReadStorage<'a, DirectionalLight>,
    point_lights: ReadStorage<'a, PointLight>,

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
        let config = shader_config(&s);

        let program = s.programs.map.get(&config).unwrap();
        let locations = s.locations.map.get(&config).unwrap();

        // TODO: only use program if it has changed from previously
        s.context.use_program(Some(&program.compiled));

        for (entity, _camera, viewport, clear_color, projection) in (
            &s.entities, &s.cameras, &s.viewports, &s.clear_colors, &s.projections
        ).join() {
            let view = s.inverse_transforms.get(entity).unwrap();
            let view_projection = projection.multiply(&view);

            clear_viewport(&s.context, viewport, clear_color);

            for (index, light) in s.directional_lights.join().enumerate() {
                set_uniform_from_vector(&s.context, &locations.u_directional_light_vector[index], &light.direction_to_light);
            }

            for (index, (_light, _position)) in (&s.point_lights, &s.world_transforms).join().enumerate() {
                                                                                              // TODO
                set_uniform_from_vector(&s.context, &locations.u_point_light_position[index], &Vector3f::new(0., 5., 0.));
            }

            for (geometry, normals, coloring, world_transform, inverse_world) in (
                &s.geometries, &s.normals, &s.colorings, &s.world_transforms, &s.inverse_transforms
            ).join() {
                let world_view_projection = view_projection.multiply(&world_transform);

                set_uniform_from_matrix(&s.context, &locations.u_world_view_projection, &world_view_projection);

                set_attribute_from_model(&s, locations.a_position, geometry.model);
                set_attribute_from_model(&s, locations.a_color, coloring.model);

                if let Some(a_normal) = locations.a_normal {
                    set_attribute_from_model(&s, a_normal, normals.model);
                }

                if let Some(u_inverse_world) = &locations.u_inverse_world {
                    set_uniform_from_matrix(&s.context, u_inverse_world, &inverse_world);
                }

                if let Some(u_world) = &locations.u_world {
                    set_uniform_from_matrix(&s.context, &u_world, &world_transform);
                }

                //set_uniform_from_vector(&s.context, &locations.u_directional_light_color, &Vector3f::new(1., 1., 1.));
                //set_uniform_from_vector(&s.context, &locations.u_point_light_color, &Vector3f::new(1., 1., 1.));
                //set_uniform_from_vector(&s.context, &locations.u_specular_light_color, &Vector3f::new(1., 1., 1.));
                //set_uniform_from_float(&s.context, &locations.u_shininess, 300.0);

                s.context.draw_arrays(GL::TRIANGLES, 0, number_of_elements(&s, geometry.model));
            }
        }
    }
}

fn shader_config(s: &SysData) -> ShaderConfig {
    let point_lights = s.point_lights.join().count() as u32;
    let directional_lights = s.directional_lights.join().count() as u32;
    let spot_lights = 0;

    ShaderConfig { point_lights, directional_lights, spot_lights }
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

fn set_uniform_from_float(context: &GL, location: &UniformLocation, float: f32) {
    context.uniform1f(Some(location), float);
}

fn number_of_elements(s: &SysData, model: Entity) -> i32 {
    let buffer = s.buffers.get(model).unwrap();
    let dimensions = s.dimensions.get(model).unwrap();

    (buffer.len / **dimensions as usize) as i32
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
