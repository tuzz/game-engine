use web_sys::WebGlTexture;
use web_sys::WebGlRenderingContext as GL;

pub fn create_blank_texcoords(length: usize) -> Vec<f32> {
    (0..length).map(|_| 0.0).collect::<Vec<_>>()
}

pub fn create_blank_texture(context: &GL, color: &[u8]) -> WebGlTexture {
    let texture = context.create_texture().unwrap();

    context.bind_texture(GL::TEXTURE_2D, Some(&texture));
    fill_texture_with_a_single_pixel(context, color);

    texture
}

fn fill_texture_with_a_single_pixel(context: &GL, color: &[u8]) {
    let target = GL::TEXTURE_2D;
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let format = GL::RGBA;
    let type_ = GL::UNSIGNED_BYTE;

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        target, level, internal_format, width, height, border, format, type_, Some(color),
    ).unwrap();
}
