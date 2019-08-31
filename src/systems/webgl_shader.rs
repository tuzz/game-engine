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
        varying vec3 v_to_point_light;

        uniform mat4 u_world;
        uniform mat4 u_world_view_projection;
        uniform mat4 u_inverse_world;
        uniform vec3 u_point_light_position;

        void main() {
          // I'm post-multiplying instead of pre-multiplying the matrices
          // because they're in row-major form which is more natural to me.

          gl_Position = a_position * u_world_view_projection;

          v_color = a_color;
          v_normal = a_normal * mat3(u_inverse_world);

          vec3 world_position = (a_position * u_world).xyz;
          v_to_point_light = u_point_light_position - world_position;
        }
    ", vec![
        "a_position",
        "a_normal",
        "a_color",
    ], vec![
        "u_world",
        "u_world_view_projection",
        "u_inverse_world",
        "u_point_light_position",
    ])
}

fn default_fragment_shader(context: &GL) -> FragmentShader {
    fragment_shader(context, "
        precision mediump float;

        varying vec3 v_normal;
        varying vec4 v_color;
        varying vec3 v_to_point_light;

        uniform vec3 u_to_directional_light;

        void main() {
          vec3 normal = normalize(v_normal);

          float light2 = dot(normal, u_to_directional_light); // TODO: combine lights
          float light = dot(normal, normalize(v_to_point_light));

          gl_FragColor = v_color;
          gl_FragColor.rgb *= light;
        }
    ", vec![
        "u_to_directional_light",
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
