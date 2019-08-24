use specs::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;
use crate::resources::{*, shader_types::*};

pub struct Shader;

impl<'a> System<'a> for Shader {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let context = world.get_mut::<WebGlContext>().unwrap();

        let vert = default_vertex_shader(context);
        let frag = default_fragment_shader(context);

        world.insert(VertexShaders { default: vert });
        world.insert(FragmentShaders { default: frag });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn default_vertex_shader(context: &GL) -> VertexShader {
    vertex_shader(context, "
        attribute vec4 a_position;

        attribute vec4 a_color;
        varying vec4 v_color;

        uniform mat4 u_matrix;

        void main() {
          gl_Position = u_matrix * a_position;
          v_color = a_color;
        }
    ", vec![
        "a_position",
        "a_color",
    ], vec![
        "u_matrix",
    ])
}

fn default_fragment_shader(context: &GL) -> FragmentShader {
    fragment_shader(context, "
        precision mediump float;

        varying vec4 v_color;

        void main() {
          gl_FragColor = v_color;
        }
    ")
}

fn vertex_shader(context: &GL, source: &str, attributes: Attributes, uniforms: Uniforms) -> VertexShader {
    let compiled = compile(context, GL::VERTEX_SHADER, source);
    VertexShader { compiled, attributes, uniforms }
}

fn fragment_shader(context: &GL, source: &str) -> FragmentShader {
    FragmentShader { compiled: compile(context, GL::FRAGMENT_SHADER, source) }
}

fn compile(context: &GL, kind: u32, source: &str) -> WebGlShader {
    let shader = context.create_shader(kind).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    shader
}
