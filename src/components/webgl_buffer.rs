use specs::prelude::*;
use web_sys::WebGlBuffer as Buffer;
use std::ops::Deref;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct WebGlBuffer {
    pub buffer: Buffer,
    pub len: usize,
}

impl Deref for WebGlBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

unsafe impl Send for WebGlBuffer {}
unsafe impl Sync for WebGlBuffer {}
