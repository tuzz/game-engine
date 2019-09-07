use specs::prelude::*;
use web_sys::HtmlImageElement;
use super::ImageToLoad;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Image(pub HtmlImageElement);

impl From<ImageToLoad> for Image {
    fn from(image_to_load: ImageToLoad) -> Self {
        Self(image_to_load.image.unwrap())
    }
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}
