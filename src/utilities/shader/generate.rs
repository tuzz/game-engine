use super::*;
use crate::resources::ShaderConfig;

const VERT: bool = true;
const FRAG: bool = false;

impl Shader {
    pub fn generate_pair(config: &ShaderConfig) -> (Self, Self) {
        (
            Self::generate_vertex_shader(config),
            Self::generate_fragment_shader(config),
        )
    }

    pub fn generate_vertex_shader(config: &ShaderConfig) -> Self {
        let mut shader = Self::default();

        shader.attribute("vec4", "a_position");
        shader.attribute("vec4", "a_color");
        shader.uniform("mat4", "u_world_view_projection");

        shader.varying("vec4", "v_color");

        vertex_normals(config, &mut shader, VERT);
        directional_lights(config, &mut shader, VERT);

        // I'm post-multiplying instead of pre-multiplying the matrices because
        // they're in row-major form which is more natural to me.
        shader.statement("gl_Position = a_position * u_world_view_projection");
        shader.statement("v_color = a_color");

        shader
    }

    pub fn generate_fragment_shader(config: &ShaderConfig) -> Self {
        let mut shader = Self::default();

        shader.header("precision mediump float");
        shader.varying("vec4", "v_color");
        shader.statement("gl_FragColor = v_color");

        vertex_normals(config, &mut shader, FRAG);
        directional_lights(config, &mut shader, FRAG);

        shader
    }
}

fn vertex_normals(config: &ShaderConfig, shader: &mut Shader, shader_type: bool) {
    if config.total_lights() == 0 {
        return;
    }

    match shader_type {
        VERT => {
            shader.attribute("vec3", "a_normal");
            shader.uniform("mat4", "u_inverse_world");
            shader.varying("vec3", "v_normal");

            // This matrix is pre-multiplied because we want its transpose.
            shader.statement("v_normal = mat3(u_inverse_world) * a_normal");
        },
        FRAG => {
            shader.varying("vec3", "v_normal");
            shader.statement("vec3 normal = normalize(v_normal)");
        },
    }
}

fn directional_lights(config: &ShaderConfig, shader: &mut Shader, shader_type: bool) {
    if config.directional_lights == 0 {
        return;
    }

    match shader_type {
        VERT => {},
        FRAG => {
            shader.statement(&format!("float directional_light = 0.0"));

            for i in 0..config.directional_lights {
                let name = format!("u_directional_light_vector_{}", i);

                shader.uniform("vec3", &name);
                shader.statement(&format!("directional_light += max(dot(normal, {}), 0.0)", name));
            }

            shader.statement("gl_FragColor.rgb *= directional_light");
        },
    }
}
