use super::*;

impl Matrix4f {
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let sx = 2.0 / (right - left);
        let sy = 2.0 / (top - bottom);
        let sz = 2.0 / (near - far);

        let tx = (left + right) / (left - right);
        let ty = (bottom + top) / (bottom - top);
        let tz = (near + far) / (near - far);

        Self([
            sx, 0., 0., tx,
            0., sy, 0., ty,
            0., 0., sz, tz,
            0., 0., 0., 1.,
        ])
    }
}
