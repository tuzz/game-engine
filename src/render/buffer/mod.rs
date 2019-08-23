use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlBuffer;

use js_sys::Float32Array;

pub struct Buffer {
    buffer: WebGlBuffer,
    len: usize,
}

impl Buffer {
    pub fn new(context: &GL, data: &[f32]) -> Self {
        let buffer = context.create_buffer().unwrap();
        let array = unsafe { Float32Array::view(&data) };

        context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);

        Buffer { buffer, len: data.len() }
    }

    pub fn bind(&self, context: &GL) {
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
    }

    pub fn len(&self, elements_per_point: usize) -> i32 {
        (self.len / elements_per_point) as i32
    }
}
