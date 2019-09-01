use super::*;
use crate::resources::ShaderConfig;

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
        shader.generate_varyings(config);

        shader.statement("gl_Position = a_position * u_world_view_projection");
        shader.statement("v_color = a_color");

        shader
    }

    pub fn generate_fragment_shader(config: &ShaderConfig) -> Self {
        let mut shader = Self::default();

        shader.header("precision mediump float");
        shader.generate_varyings(config);

        shader.statement("gl_FragColor = v_color");

        shader
    }

    fn generate_varyings(&mut self, _config: &ShaderConfig) {
        self.varying("vec4", "v_color");
    }
}
