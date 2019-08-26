mod convert;
mod multiply;
use std::ops;
use std::f32::consts::PI;

pub struct Matrix4f(pub [f32; 16]);

impl Matrix4f {
    pub fn new(array: [f32; 16]) -> Self {
        Matrix4f(array)
    }

    pub fn identity() -> Self {
        Matrix4f([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Matrix4f([
            1., 0., 0., tx,
            0., 1., 0., ty,
            0., 0., 1., tz,
            0., 0., 0., 1.,
        ])
    }

    pub fn x_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
            1., 0., 0., 0.,
            0.,  c, -s, 0.,
            0.,  s,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn y_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
             c, 0.,  s, 0.,
            0., 1., 0., 0.,
            -s, 0.,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn z_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
             c, -s, 0., 0.,
             s,  c, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Matrix4f([
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let sx = 2.0 / (right - left);
        let sy = 2.0 / (top - bottom);
        let sz = 2.0 / (near - far);

        let tx = (left + right) / (left - right);
        let ty = (bottom + top) / (bottom - top);
        let tz = (near + far) / (near - far);

        Matrix4f([
            sx, 0., 0., tx,
            0., sy, 0., ty,
            0., 0., sz, tz,
            0., 0., 0., 1.,
        ])
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let theta = (PI - fovy) / 2.0;
        let tangent = theta.tan();
        let depth_inv = 1.0 / (near - far);

        let sx = tangent / aspect;
        let sy = tangent;
        let sz = (near + far) * depth_inv;

        let tz = near * far * depth_inv * 2.0;

        Matrix4f([
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, tz,
            0., 0., -1., 0.
        ])
    }

    // Immutable functions for chaining:
    pub fn translate(&self, tx: f32, ty: f32, tz: f32) -> Self {
        self * Matrix4f::translation(tx, ty, tz)
    }

    pub fn x_rotate(&self, radians: f32) -> Self {
        self * Matrix4f::x_rotation(radians)
    }

    pub fn y_rotate(&self, radians: f32) -> Self {
        self * Matrix4f::y_rotation(radians)
    }

    pub fn z_rotate(&self, radians: f32) -> Self {
        self * Matrix4f::z_rotation(radians)
    }

    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        self * Matrix4f::scaling(sx, sy, sz)
    }

    // Mutable functions for chaining:
    pub fn translate_mut(&mut self, tx: f32, ty: f32, tz: f32) -> &mut Self {
        *self *= Matrix4f::translation(tx, ty, tz); self
    }

    pub fn x_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Matrix4f::x_rotation(radians); self
    }

    pub fn y_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Matrix4f::y_rotation(radians); self
    }

    pub fn z_rotate_mut(&mut self, radians: f32) -> &mut Self {
        *self *= Matrix4f::z_rotation(radians); self
    }

    pub fn scale_mut(&mut self, sx: f32, sy: f32, sz: f32) -> &mut Self {
        *self *= Matrix4f::scaling(sx, sy, sz); self
    }
}

impl_op_ex!(* |left: &Matrix4f, right: &Matrix4f| -> Matrix4f {
    multiply::multiply(left, right).into()
});

impl_op_ex!(*= |left: &mut Matrix4f, right: &Matrix4f| {
    left.assign_tuple(multiply::multiply(left, right));
});

#[cfg(test)]
mod test;
