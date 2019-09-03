use specs::prelude::*;
use crate::resources::*;
use crate::components::*;

pub struct UseProgram;

#[derive(SystemData)]
pub struct SysData<'a> {
    programs: ReadExpect<'a, ShaderPrograms>,
    context: ReadExpect<'a, WebGlContext>,

    directional_lights: ReadStorage<'a, DirectionalLight>,
    point_lights: ReadStorage<'a, PointLight>,
    transforms: ReadStorage<'a, WorldTransform>,

    active_config: Write<'a, ActiveConfig>,
}

impl<'a> System<'a> for UseProgram {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: Self::SystemData) {
        let config = Self::shader_config_for_number_of_lights(&s);

        if config == s.active_config.config {
            return;
        }

        let program = s.programs.map.get(&config).unwrap();
        s.context.use_program(Some(&program.compiled));

        *s.active_config = ActiveConfig { config };
    }
}

impl UseProgram {
    fn shader_config_for_number_of_lights(s: &SysData) -> ShaderConfig {
        let point_lights = (&s.point_lights, &s.transforms).join().count() as u32;
        let directional_lights = (&s.directional_lights, &s.transforms).join().count() as u32;

        ShaderConfig { point_lights, directional_lights }
    }
}
