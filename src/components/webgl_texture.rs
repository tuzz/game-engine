use specs::prelude::*;
use web_sys::WebGlTexture as Texture;
use std::ops::Deref;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct WebGlTexture(pub Texture);

impl Deref for WebGlTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for WebGlTexture {}
unsafe impl Sync for WebGlTexture {}
