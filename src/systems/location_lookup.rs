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

fn shader_location(_config: &ShaderConfig, program: &ShaderProgram) -> ShaderLocation {
    ShaderLocation {
        a_position: *program.attribute_map.get("a_position").unwrap(),
        //a_normal: *program.attribute_map.get("a_normal").unwrap(),
        a_color: *program.attribute_map.get("a_color").unwrap(),

        //u_world: program.uniform_map.get("u_world").unwrap().to_owned(),
        u_world_view_projection: program.uniform_map.get("u_world_view_projection").unwrap().to_owned(),
        //u_inverse_world: program.uniform_map.get("u_inverse_world").unwrap().to_owned(),

        //u_camera_position: program.uniform_map.get("u_camera_position").unwrap().to_owned(),
        //u_point_light_position: program.uniform_map.get("u_point_light_position").unwrap().to_owned(),
        //u_to_directional_light: program.uniform_map.get("u_to_directional_light").unwrap().to_owned(),

        //u_directional_light_color: program.uniform_map.get("u_directional_light_color").unwrap().to_owned(),
        //u_point_light_color: program.uniform_map.get("u_point_light_color").unwrap().to_owned(),
        //u_specular_light_color: program.uniform_map.get("u_specular_light_color").unwrap().to_owned(),
        //u_shininess: program.uniform_map.get("u_shininess").unwrap().to_owned(),
    }
}
