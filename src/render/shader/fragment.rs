use super::*;

type V = Vec<&'static str>;

pub struct FragmentShader {
    pub compiled: WebGlShader,
}

impl FragmentShader {
    fn new(context: &GL, source: &str) -> Self {
        let compiled = Self::compile(context, GL::FRAGMENT_SHADER, source);

        Self { compiled }
    }
}

impl Shader for FragmentShader {
    fn default(context: &GL) -> Self {
        let source = "
            precision mediump float;

            varying vec4 v_color;

            void main() {
              gl_FragColor = v_color;
            }
        ";

        Self::new(context, source)
    }
}
