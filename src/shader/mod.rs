use specs::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;
use super::resources::*;

pub struct Shader;

impl<'a> System<'a> for Shader {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let context = world.get_mut::<WebGlContext>().unwrap();

        let vert = compile(context, GL::VERTEX_SHADER, include_str!("default.vert"));
        let frag = compile(context, GL::FRAGMENT_SHADER, include_str!("default.frag"));

        world.insert(VertexShaders { default: vert });
        world.insert(FragmentShaders { default: frag });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn compile(context: &GL, kind: u32, source: &str) -> WebGlShader {
    let shader = context.create_shader(kind).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    shader
}
