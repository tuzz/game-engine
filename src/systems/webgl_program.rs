use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;
use web_sys::WebGlProgram as Program;
use crate::resources::{*, shader_types::*};

pub struct WebGlProgram;

impl<'a> System<'a> for WebGlProgram {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let program = default_program(world);

        world.insert(ShaderPrograms { default: program })
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn default_program(world: &World) -> ShaderProgram {
    let context = world.fetch::<WebGlContext>();

    let vert = &world.fetch::<VertexShaders>().default;
    let frag = &world.fetch::<FragmentShaders>().default;

    let program = compile(&context, &vert.compiled, &frag.compiled);

    ShaderProgram {
        attribute_map: attribute_map(&context, &program, vert),
        uniform_map: uniform_map(&context, &program, vert),
        compiled: program,
    }
}

fn compile(context: &GL, vert: &WebGlShader, frag: &WebGlShader) -> Program {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert);
    context.attach_shader(&program, frag);
    context.link_program(&program);

    program
}

fn attribute_map(context: &GL, program: &Program, vert: &VertexShader) -> AttributeMap {
    vert.attributes.iter().map(|&name| {
        (name, context.get_attrib_location(&program, name) as AttributeLocation)
    }).collect()
}

fn uniform_map(context: &GL, program: &Program, vert: &VertexShader) -> UniformMap {
    vert.uniforms.iter().map(|&name| {
        (name, context.get_uniform_location(&program, name).unwrap() as UniformLocation)
    }).collect()
}
