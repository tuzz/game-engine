use specs::prelude::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ModelsToLoad {
    pub object_filenames: Vec<(String, Option<Entity>)>,
    pub material_filenames: Vec<(String, Option<Entity>)>,

    pub preloading: bool,
    pub preloaded: bool,
}

impl ModelsToLoad {
    pub fn new(object_filenames: &[&str], material_filenames: &[&str]) -> Self {
        Self {
            object_filenames: with_entities(object_filenames),
            material_filenames: with_entities(material_filenames),
            preloading: false,
            preloaded: false,
        }
    }
}

fn with_entities(filenames: &[&str]) -> Vec<(String, Option<Entity>)> {
    filenames.iter().map(|s| (s.to_string(), None)).collect::<Vec<_>>()
}
