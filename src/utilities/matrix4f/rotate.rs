use super::*;

impl Matrix4f {
    pub fn x_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Self([
            1., 0., 0., 0.,
            0.,  c, -s, 0.,
            0.,  s,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn y_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Self([
             c, 0.,  s, 0.,
            0., 1., 0., 0.,
            -s, 0.,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn z_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Self([
             c, -s, 0., 0.,
             s,  c, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    #[must_use]
    pub fn x_rotate(&self, radians: f32) -> Self {
        self * Self::x_rotation(radians)
    }

    #[must_use]
    pub fn y_rotate(&self, radians: f32) -> Self {
        self * Self::y_rotation(radians)
    }

    #[must_use]
    pub fn z_rotate(&self, radians: f32) -> Self {
        self * Self::z_rotation(radians)
    }

    pub fn x_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Self::x_rotation(radians); self
    }

    pub fn y_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Self::y_rotation(radians); self
    }

    pub fn z_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Self::z_rotation(radians); self
    }
}
