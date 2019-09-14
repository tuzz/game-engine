use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct FileContent(pub String);

impl Deref for FileContent {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
