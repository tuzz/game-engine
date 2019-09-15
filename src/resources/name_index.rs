use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct NameIndex {
    pub index: HashMap<String, Entity>,
    pub reverse: HashMap<Entity, String>,
}

impl NameIndex {
    pub fn get(&self, name: &str) -> Option<&Entity> {
        self.index.get(name)
    }

    pub fn insert(&mut self, name: &str, entity: &Entity) {
        self.index.insert(name.to_string(), *entity);
        self.reverse.insert(*entity, name.to_string());
    }

    pub fn remove(&mut self, entity: &Entity) {
        let name = self.reverse.remove(entity).unwrap();
        self.index.remove(&name).unwrap();
    }
}
