use specs::prelude::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct ModelGroups(pub Map);
type Map = HashMap<String, BitSet>;

impl Deref for ModelGroups {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModelGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ModelGroups {
    pub fn add(&mut self, name: String, entity: &Entity) {
        self.entry(name).or_insert_with(|| BitSet::new()).add(entity.id());
    }
}
