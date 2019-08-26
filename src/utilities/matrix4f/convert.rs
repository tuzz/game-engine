use std::ops::Deref;
use super::*;

pub type Tuple = (
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
);

impl From<Tuple> for Matrix4f {
    fn from(t: Tuple) -> Self {
        Self([
           t.0,  t.1,  t.2, t.3,
           t.4,  t.5,  t.6, t.7,
           t.8,  t.9, t.10, t.11,
          t.12, t.13, t.14, t.15,
        ])
    }
}

impl Matrix4f {
    pub fn assign_tuple(&mut self, t: Tuple) {
        let m = &mut self.0;

          m[0] =  t.0;    m[1] =  t.1;    m[2] =  t.2;    m[3] =  t.3;
          m[4] =  t.4;    m[5] =  t.5;    m[6] =  t.6;    m[7] =  t.7;
          m[8] =  t.8;    m[9] =  t.9;   m[10] = t.10;   m[11] = t.11;
         m[12] = t.12;   m[13] = t.13;   m[14] = t.14;   m[15] = t.15;
    }
}

impl Deref for Matrix4f {
    type Target = [f32; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
