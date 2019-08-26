use super::*;

impl Matrix4f {
    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self([
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, 0.,
            0., 0., 0., 1.,
        ])
    }

    #[must_use]
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        self * Self::scaling(sx, sy, sz)
    }

    pub fn scale_mut(&mut self, sx: f32, sy: f32, sz: f32) -> &mut Self {
        *self *= Self::scaling(sx, sy, sz); self
    }
}
