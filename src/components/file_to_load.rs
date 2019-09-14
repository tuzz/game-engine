use specs::prelude::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct FileToLoad {
    pub src: String,
    pub loading: bool,
}

impl FileToLoad {
    pub fn new(src: &str) -> Self {
        Self { src: src.to_string(), loading: false }
    }
}
