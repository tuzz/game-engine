use super::*;

impl Matrix4f {
    pub fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self([
            1., 0., 0., tx,
            0., 1., 0., ty,
            0., 0., 1., tz,
            0., 0., 0., 1.,
        ])
    }

    #[must_use]
    pub fn translate(&self, tx: f32, ty: f32, tz: f32) -> Self {
        self * Self::translation(tx, ty, tz)
    }

    pub fn translate_mut(&mut self, tx: f32, ty: f32, tz: f32) -> &mut Self {
        *self *= Self::translation(tx, ty, tz); self
    }
}
