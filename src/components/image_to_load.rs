use specs::prelude::*;
use web_sys::HtmlImageElement;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ImageToLoad {
    pub src: String,
    pub image: Option<HtmlImageElement>,
}

impl ImageToLoad {
    pub fn new(src: &str) -> Self {
        Self { src: src.to_string(), image: None }
    }
}

unsafe impl Send for ImageToLoad {}
unsafe impl Sync for ImageToLoad {}
