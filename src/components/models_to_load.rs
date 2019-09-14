use specs::prelude::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ModelsToLoad {
    pub object_filenames: Vec<(String, Option<Entity>)>,

    pub preloading: bool,
    pub preloaded: bool,
}

impl ModelsToLoad {
    pub fn new(object_filenames: &[&str]) -> Self {
        let vec = object_filenames.iter()
            .map(|s| (s.to_string(), None))
            .collect::<Vec<_>>();

        Self { object_filenames: vec, preloading: false, preloaded: false }
    }
}
