use specs::prelude::*;
use crate::resources::*;

pub struct LocationLookup;

impl<'a> System<'a> for LocationLookup {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let shader_programs = world.fetch::<ShaderPrograms>();

        let mut locations = LocationMap::new();

        for (config, program) in &shader_programs.map {
            let location = shader_location(config, program);
            locations.insert(config.clone(), location);
        }

        drop(shader_programs);

        world.insert(ShaderLocations { map: locations });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn shader_location(config: &ShaderConfig, program: &ShaderProgram) -> ShaderLocation {
    ShaderLocation {
        a_position: attribute(program, "a_position"),
        a_normal: optional_attribute(program, "a_normal"),
        a_color: attribute(program, "a_color"),

        //u_world: program.uniform_map.get("u_world").unwrap().to_owned(),
        u_world_view_projection: uniform(program, "u_world_view_projection"),
        u_inverse_world: optional_uniform(program, "u_inverse_world"),
        u_directional_light_vector: uniform_array(program, "u_directional_light_vector", config.directional_lights),

        //u_camera_position: program.uniform_map.get("u_camera_position").unwrap().to_owned(),
        //u_point_light_position: program.uniform_map.get("u_point_light_position").unwrap().to_owned(),
        //u_to_directional_light: program.uniform_map.get("u_to_directional_light").unwrap().to_owned(),

        //u_directional_light_color: program.uniform_map.get("u_directional_light_color").unwrap().to_owned(),
        //u_point_light_color: program.uniform_map.get("u_point_light_color").unwrap().to_owned(),
        //u_specular_light_color: program.uniform_map.get("u_specular_light_color").unwrap().to_owned(),
        //u_shininess: program.uniform_map.get("u_shininess").unwrap().to_owned(),
    }
}

fn attribute(program: &ShaderProgram, name: &str) -> AttributeLocation {
    *program.attribute_map.get(name).unwrap()
}

fn optional_attribute(program: &ShaderProgram, name: &str) -> Option<AttributeLocation> {
    program.attribute_map.get(name).cloned()
}

fn uniform(program: &ShaderProgram, name: &str) -> UniformLocation {
    program.uniform_map.get(name).unwrap().to_owned()
}

fn optional_uniform(program: &ShaderProgram, name: &str) -> Option<UniformLocation> {
    program.uniform_map.get(name).cloned()
}

fn uniform_array(program: &ShaderProgram, basename: &str, count: u32) -> Vec<UniformLocation> {
    (0..count).map(|i| uniform(program, &format!("{}_{}", basename, i))).collect()
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
