use specs::*;

use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;
use web_sys::WebGlProgram;

use super::resources::*;

pub struct Program;

impl<'a> System<'a> for Program {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let default = default_program(world);
        world.insert(ShaderPrograms { default });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn default_program(world: &World) -> WebGlProgram {
    let context = world.fetch::<WebGlContext>();

    let verts = world.fetch::<VertexShaders>();
    let frags = world.fetch::<FragmentShaders>();

    link(&context, &verts.default, &frags.default)
}

fn link(context: &GL, vert: &WebGlShader, frag: &WebGlShader) -> WebGlProgram {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert);
    context.attach_shader(&program, frag);
    context.link_program(&program);

    program
}
