use specs::prelude::*;
use web_sys::HtmlImageElement;
use std::ops::Deref;
use super::ImageToLoad;

pub struct Image(pub HtmlImageElement);

impl Component for Image {
    type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

impl Deref for Image {
    type Target = HtmlImageElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ImageToLoad> for Image {
    fn from(image_to_load: ImageToLoad) -> Self {
        Self(image_to_load.image.unwrap())
    }
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}
