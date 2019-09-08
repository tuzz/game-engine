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
        shader.attribute("vec3", "a_color");
        shader.attribute("vec2", "a_texcoord");

        shader.varying("vec3", "v_color");
        shader.varying("vec2", "v_texcoord");

        shader.uniform("mat4", "u_world_view_projection");

        vertex_normals(config, &mut shader, VERT);
        camera_vector(config, &mut shader, VERT);
        directional_lights(config, &mut shader, VERT);
        point_lights(config, &mut shader, VERT);

        // I'm post-multiplying instead of pre-multiplying the matrices because
        // they're in row-major form which is more natural to me.
        shader.statement("gl_Position = a_position * u_world_view_projection");
        shader.statement("v_color = a_color");
        shader.statement("v_texcoord = a_texcoord");

        shader
    }

    pub fn generate_fragment_shader(config: &ShaderConfig) -> Self {
        let mut shader = Self::default();

        shader.header("precision mediump float");
        shader.varying("vec3", "v_color");
        shader.varying("vec2", "v_texcoord");

        shader.uniform("vec3", "u_material_ambient");
        shader.uniform("vec3", "u_material_diffuse");
        shader.uniform("vec3", "u_material_specular");
        shader.uniform("float", "u_material_shininess");

        shader.uniform("sampler2D", "u_texture");

        shader.statement("vec3 diffuse = vec3(0.0, 0.0, 0.0)");
        shader.statement("vec3 specular = vec3(0.0, 0.0, 0.0)");

        // TODO: add uniforms for the color of lights

        vertex_normals(config, &mut shader, FRAG);
        camera_vector(config, &mut shader, FRAG);
        directional_lights(config, &mut shader, FRAG);
        point_lights(config, &mut shader, FRAG);

        // TODO make u_material_shininess optional

        shader.statement("vec3 color = v_color * texture2D(u_texture, v_texcoord).xyz");

        shader.statement("gl_FragColor = vec4(u_material_ambient * color, 1.0)");
        shader.statement("gl_FragColor.xyz += diffuse * u_material_diffuse * color");
        shader.statement("gl_FragColor.xyz += specular * u_material_specular + (u_material_shininess * 0.0)");

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

fn camera_vector(config: &ShaderConfig, shader: &mut Shader, shader_type: bool) {
    if config.total_lights() == 0 {
        return;
    }

    match shader_type {
        VERT => {
            shader.uniform("mat4", "u_world");
            shader.statement("vec3 world_position = (a_position * u_world).xyz");

            shader.uniform("vec3", "u_camera_position");
            shader.varying("vec3", "v_surface_to_camera");
            shader.statement("v_surface_to_camera = u_camera_position - world_position");
        },
        FRAG => {
            shader.varying("vec3", "v_surface_to_camera");
            shader.statement("vec3 to_camera = normalize(v_surface_to_camera)");
        },
    }
}

fn directional_lights(config: &ShaderConfig, shader: &mut Shader, shader_type: bool) {
    match shader_type {
        VERT => {},
        FRAG => {
            for i in 0..config.directional_lights {
                let to_light = format!("u_directional_light_vector_{}", i);
                let half_vec = format!("directional_half_vec_{}", i);
                let diffuse = format!("directional_diffuse_{}", i);

                shader.uniform("vec3", &to_light);

                shader.statement(&format!("float {} = dot(normal, {})", diffuse, to_light));
                shader.statement(&format!("diffuse += max({}, 0.0)", diffuse));

                shader.statement(&format!("vec3 {} = normalize({} + to_camera)", half_vec, to_light));

                shader.statement(&format!("if ({} > 0.0) {{", diffuse));
                // TODO: does it make a difference if I add all specular components THEN pow them?
                shader.statement(&format!("specular += pow(dot(normal, {}), u_material_shininess)", half_vec));
                shader.statement("}");
            }
        },
    }
}

fn point_lights(config: &ShaderConfig, shader: &mut Shader, shader_type: bool) {
    match shader_type {
        VERT => {
            for i in 0..config.point_lights {
                let uniform = format!("u_point_light_position_{}", i);
                let varying = format!("v_surface_to_point_light_{}", i);

                shader.uniform("vec3", &uniform);
                shader.varying("vec3", &varying);

                shader.statement(&format!("{} = {} - world_position", varying, uniform));
            }
        },
        FRAG => {
            for i in 0..config.point_lights {
                let varying = format!("v_surface_to_point_light_{}", i);
                let to_light = format!("to_point_light_{}", i);
                let half_vec = format!("point_half_vec_{}", i);
                let diffuse = format!("point_diffuse_{}", i);

                shader.varying("vec3", &varying);
                shader.statement(&format!("vec3 {} = normalize({})", to_light, varying));
                shader.statement(&format!("float {} = dot(normal, {})", diffuse, to_light));
                shader.statement(&format!("diffuse += max({}, 0.0)", diffuse));

                shader.statement(&format!("vec3 {} = normalize({} + to_camera)", half_vec, to_light));

                shader.statement(&format!("if ({} > 0.0) {{", diffuse));
                // TODO: does it make a difference if I add all specular components THEN pow them?
                shader.statement(&format!("specular += pow(dot(normal, {}), u_material_shininess)", half_vec));
                shader.statement("}");
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_transforms_vertex_positions_by_the_world_view_projection_matrix() {
        let config = ShaderConfig::no_lights();
        let (vert, _frag) = Shader::generate_pair(&config);

        assert_contains(&vert, &[
            "attribute vec4 a_position;",
            "uniform mat4 u_world_view_projection;",

            "void main() {",
            "    gl_Position = a_position * u_world_view_projection;",
            "}",
        ]);
    }

    #[test]
    fn it_calculates_normals_using_the_inverse_world_matrix_when_there_are_lights() {
        let config = ShaderConfig::one_of_each_light();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&vert, &[
            "attribute vec3 a_normal;",
            "uniform mat4 u_inverse_world;",
            "varying vec3 v_normal;",

            "void main() {",
            "    v_normal = mat3(u_inverse_world) * a_normal;",
            "}",
        ]);

        assert_contains(&frag, &[
            "varying vec3 v_normal;",

            "void main() {",
            "    vec3 normal = normalize(v_normal);",
            "}",
        ]);
    }

    #[test]
    fn it_calculates_the_vector_from_the_surface_to_the_camera() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&vert, &[
            "uniform vec3 u_camera_position;",
            "varying vec3 v_surface_to_camera;",

            "void main() {",
            "    v_surface_to_camera = u_camera_position - world_position;",
            "}",
        ]);
    }

    #[test]
    fn it_calculates_the_vector_from_the_surface_to_each_of_the_point_lights() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&vert, &[
            "uniform mat4 u_world;",

            "uniform vec3 u_point_light_position_0;",
            "uniform vec3 u_point_light_position_1;",

            "varying vec3 v_surface_to_point_light_0;",
            "varying vec3 v_surface_to_point_light_1;",

            "void main() {",
            "    vec3 world_position = (a_position * u_world).xyz;",

            "    v_surface_to_point_light_0 = u_point_light_position_0 - world_position;",
            "    v_surface_to_point_light_1 = u_point_light_position_1 - world_position;",
            "}",
        ]);
    }

    #[test]
    fn it_accumulates_diffuse_light_from_the_directional_lights() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&frag, &[
            "uniform vec3 u_directional_light_vector_0;",
            "uniform vec3 u_directional_light_vector_1;",

            "void main() {",
            "    float directional_diffuse_0 = dot(normal, u_directional_light_vector_0);",
            "    float directional_diffuse_1 = dot(normal, u_directional_light_vector_1);",

            "    diffuse += max(directional_diffuse_0, 0.0);",
            "    diffuse += max(directional_diffuse_1, 0.0);",
            "}",
        ]);
    }

    #[test]
    fn it_accumulates_diffuse_light_from_the_point_lights() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&frag, &[
            "varying vec3 v_surface_to_camera;",

            "varying vec3 v_surface_to_point_light_0;",
            "varying vec3 v_surface_to_point_light_1;",

            "void main() {",
            "    vec3 to_point_light_0 = normalize(v_surface_to_point_light_0);",
            "    vec3 to_point_light_1 = normalize(v_surface_to_point_light_1);",

            "    float point_diffuse_0 = dot(normal, to_point_light_0);",
            "    float point_diffuse_1 = dot(normal, to_point_light_1);",

            "    diffuse += max(point_diffuse_0, 0.0);",
            "    diffuse += max(point_diffuse_1, 0.0);",
            "}",
        ]);
    }

    #[test]
    fn it_accumulates_specular_light_from_the_directional_lights() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&frag, &[
            "void main() {",
            "    vec3 directional_half_vec_0 = normalize(u_directional_light_vector_0 + to_camera);",
            "    vec3 directional_half_vec_1 = normalize(u_directional_light_vector_1 + to_camera);",

            "    if (directional_diffuse_0 > 0.0) {;",
            "    specular += pow(dot(normal, directional_half_vec_0), u_material_shininess);",
            "    };",

            "    if (directional_diffuse_1 > 0.0) {;",
            "    specular += pow(dot(normal, directional_half_vec_1), u_material_shininess);",
            "    };",
            "}",
        ]);
    }

    #[test]
    fn it_accumulates_specular_light_from_the_point_lights() {
        let config = ShaderConfig::a_few_lights();
        let (vert, frag) = Shader::generate_pair(&config);

        assert_contains(&frag, &[
            "void main() {",
            "    vec3 point_half_vec_0 = normalize(to_point_light_0 + to_camera);",
            "    vec3 point_half_vec_1 = normalize(to_point_light_1 + to_camera);",

            "    if (point_diffuse_0 > 0.0) {;",
            "    specular += pow(dot(normal, point_half_vec_0), u_material_shininess);",
            "    };",

            "    if (point_diffuse_1 > 0.0) {;",
            "    specular += pow(dot(normal, point_half_vec_1), u_material_shininess);",
            "    };",
            "}",
        ]);
    }

    // TODO: proper accumulation of light (needs investigation)
    //  - ensure all light contributions are between 0 and 1
    //  - sum light contributions together
    //  - use terms like 'irradiance' and 'intensity' (component?)
    // TODO: investigate whether to enable gamma setting for GL
    // TODO: delete webgl_shader and consider renames

    fn assert_contains(shader: &Shader, expected: &'static [&str]) {
        let lines = shader.lines();

        for l in expected {
            let test = lines.contains(&l.to_string());
            assert!(test, "\n{}\n\nDid not contain '{}'\n\n", shader.source(), l);
        }
    }
}
