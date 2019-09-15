use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::Float32Array;

use crate::resources::WebGlContext;
use crate::components::BufferData;
use crate::components::WebGlBuffer as Buffer;

pub struct WebGlBuffer;

impl<'a> System<'a> for WebGlBuffer {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, WebGlContext>,
        ReadStorage<'a, BufferData>,
        WriteStorage<'a, Buffer>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        let (entities, context, buffer_datas, mut webgl_buffers) = system_data;

        let entities_to_update = (&entities, &buffer_datas, !&webgl_buffers).join()
            .map(|(entity, _, _)| entity)
            .collect::<Vec<_>>();

        for entity in entities_to_update {
            let buffer_data = buffer_datas.get(entity).unwrap();

            let buffer = context.create_buffer().unwrap();
            let array = unsafe { Float32Array::view(&buffer_data) };

            context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
            context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);

            let webgl_buffer = Buffer { buffer, len: buffer_data.len() };
            webgl_buffers.insert(entity, webgl_buffer).unwrap();
        }
    }
}
