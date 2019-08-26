use std::ops::Deref;
use super::*;

impl From<Tuple> for Vector3f {
    fn from(t: Tuple) -> Self {
        Self([t.0,  t.1,  t.2])
    }
}

impl Vector3f {
    pub fn assign_tuple(&mut self, t: Tuple) {
        let v = &mut self.0;

        v[0] =  t.0;
        v[1] =  t.1;
        v[2] =  t.2;
    }
}

impl Deref for Vector3f {
    type Target = [f32; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
