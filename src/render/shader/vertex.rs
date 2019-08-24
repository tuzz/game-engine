use super::*;

type V = Vec<&'static str>;

pub struct VertexShader {
    pub compiled: WebGlShader,
    pub attributes: V,
    pub uniforms: V,
}

impl VertexShader {
    fn new(context: &GL, source: &str, attributes: V, uniforms: V) -> Self {
        let compiled = Self::compile(context, GL::VERTEX_SHADER, source);

        Self { compiled, attributes, uniforms }
    }
}

impl Shader for VertexShader {
    fn default(context: &GL) -> Self {
        let source = "
            attribute vec4 a_position;

            attribute vec4 a_color;
            varying vec4 v_color;

            uniform mat4 u_matrix;

            void main() {
              gl_Position = u_matrix * a_position;
              v_color = a_color;
            }
        ";

        Self::new(context, source, vec!["a_position", "a_color"], vec!["u_matrix"])
    }
}
