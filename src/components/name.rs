use specs::prelude::*;
use std::ops::Deref;

pub struct Name(pub String);

impl Component for Name {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
