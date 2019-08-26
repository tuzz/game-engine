use super::*;

pub type Tuple = (f32, f32, f32);

impl From<Tuple> for Vector3f {
    fn from(t: Tuple) -> Self {
        Self { x: t.0, y: t.1, z: t.2 }
    }
}

impl Vector3f {
    pub fn assign_tuple(&mut self, t: Tuple) {
        self.x = t.0;
        self.y = t.1;
        self.z = t.2;
    }
}
