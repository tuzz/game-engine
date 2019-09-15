use specs::prelude::*;
use web_sys::{WebGlShader, WebGlProgram, WebGlRenderingContext as GL};
use crate::resources::*;
use crate::utilities::Shader;

pub struct ShaderCompiler;

impl<'a> System<'a> for ShaderCompiler {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let context = world.fetch::<WebGlContext>();
        let shader_config = ShaderConfig::default();

        let mut programs = ProgramMap::new();

        for config in shader_config.combinations() {
            let program = shader_program(&context, &config);
            programs.insert(config, program);
        }

        drop(context);

        world.insert(shader_config);
        world.insert(ShaderPrograms { map: programs });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn shader_program(context: &GL, config: &ShaderConfig) -> ShaderProgram {
    let (vert, frag) = Shader::generate_pair(config);

    let program = link(context,
        &compile(context, GL::VERTEX_SHADER, &vert.source()),
        &compile(context, GL::FRAGMENT_SHADER, &frag.source()),
    );

    ShaderProgram {
        attribute_map: attribute_map(context, &program, &vert),
        uniform_map: uniform_map(context, &program, &vert, &frag),
        compiled: program,
    }
}

fn compile(context: &GL, kind: u32, source: &str) -> WebGlShader {
    let shader = context.create_shader(kind).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    shader
}

fn link(context: &GL, vert: &WebGlShader, frag: &WebGlShader) -> WebGlProgram {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert);
    context.attach_shader(&program, frag);
    context.link_program(&program);

    program
}

fn attribute_map(context: &GL, program: &WebGlProgram, vert: &Shader) -> AttributeMap {
    let vert_names = vert.attributes.iter().map(|a| &a.name);

    vert_names.map(|name| {
        (name.clone(), context.get_attrib_location(&program, name) as AttributeLocation)
    }).collect()
}

fn uniform_map(context: &GL, program: &WebGlProgram, vert: &Shader, frag: &Shader) -> UniformMap {
    let vert_names = vert.uniforms.iter().map(|u| &u.name);
    let frag_names = frag.uniforms.iter().map(|u| &u.name);

    vert_names.chain(frag_names).map(|name| {
        (name.clone(), context.get_uniform_location(&program, name).unwrap() as UniformLocation)
    }).collect()
}
