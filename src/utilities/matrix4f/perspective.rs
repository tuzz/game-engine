use super::*;

impl Matrix4f {
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let theta = (std::f32::consts::PI - fovy) / 2.0;
        let tangent = theta.tan();
        let depth_inv = 1.0 / (near - far);

        let sx = tangent / aspect;
        let sy = tangent;
        let sz = (near + far) * depth_inv;

        let tz = near * far * depth_inv * 2.0;

        Self([
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, tz,
            0., 0., -1., 0.
        ])
    }
}
