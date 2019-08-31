use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader as Shader;
use crate::resources::{*, shader_types::*};

pub struct WebGlShader;

impl<'a> System<'a> for WebGlShader {
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
        attribute vec3 a_normal;
        attribute vec4 a_color;

        varying vec4 v_color;
        varying vec3 v_normal;

        uniform mat4 u_world_view_projection;
        uniform mat4 u_inverse_world;

        void main() {
          // I'm post-multiplying instead of pre-multiplying the matrices
          // because they're in row-major form which is more natural to me.

          gl_Position = a_position * u_world_view_projection;

          v_color = a_color;
          v_normal = a_normal * mat3(u_inverse_world);
        }
    ", vec![
        "a_position",
        "a_normal",
        "a_color",
    ], vec![
        "u_world_view_projection",
        "u_inverse_world",
    ])
}

fn default_fragment_shader(context: &GL) -> FragmentShader {
    fragment_shader(context, "
        precision mediump float;

        varying vec3 v_normal;
        varying vec4 v_color;

        uniform vec3 u_direction_to_light;

        void main() {
          vec3 normal = normalize(v_normal);
          float light = dot(normal, u_direction_to_light);

          gl_FragColor = v_color;
          gl_FragColor.rgb *= light;
        }
    ", vec![
        "u_direction_to_light",
    ])
}

fn vertex_shader(context: &GL, source: &str, attributes: Attributes, uniforms: Uniforms) -> VertexShader {
    let compiled = compile(context, GL::VERTEX_SHADER, source);
    VertexShader { compiled, attributes, uniforms }
}

fn fragment_shader(context: &GL, source: &str, uniforms: Uniforms) -> FragmentShader {
    let compiled = compile(context, GL::FRAGMENT_SHADER, source);
    FragmentShader { compiled, uniforms  }
}

fn compile(context: &GL, kind: u32, source: &str) -> Shader {
    let shader = context.create_shader(kind).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    shader
}
