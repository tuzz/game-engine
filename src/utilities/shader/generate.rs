use super::*;

impl Shader {
    pub fn generate_pair(config: &Config) -> (Self, Self) {
        (
            Self::generate_vertex_shader(config),
            Self::generate_fragment_shader(config),
        )
    }

    pub fn generate_vertex_shader(config: &Config) -> Self {
        let mut shader = Self::default();

        shader.attribute("vec4", "a_position");
        shader.attribute("vec4", "a_color");

        shader.generate_varyings(config);

        shader.statement("gl_Position = a_position");
        shader.statement("v_color = a_color");

        shader
    }

    pub fn generate_fragment_shader(config: &Config) -> Self {
        let mut shader = Self::default();

        shader.generate_varyings(config);
        shader.statement("gl_FragColor = v_color");

        shader
    }

    fn generate_varyings(&mut self, _config: &Config) {
        self.varying("vec4", "v_color");
    }
}
