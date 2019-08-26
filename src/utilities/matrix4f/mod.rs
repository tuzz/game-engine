use std::ops;
use std::ops::Deref;
use std::f32::consts::PI;

pub struct Matrix4f(pub [f32; 16]);

impl Deref for Matrix4f {
    type Target = [f32; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = multiply(left, right);
    Matrix4f([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
});

impl_op_ex!(*= |left: &mut Matrix4f, right: &Matrix4f| {
    let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = multiply(left, right);
    let x = &mut left.0;

     x[0] = a;    x[1] = b;    x[2] = c;    x[3] = d;
     x[4] = e;    x[5] = f;    x[6] = g;    x[7] = h;
     x[8] = i;    x[9] = j;   x[10] = k;   x[11] = l;
    x[12] = m;   x[13] = n;   x[14] = o;   x[15] = p;
});

#[inline]
fn multiply(a: &Matrix4f, b: &Matrix4f) -> (
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
) {
    let (a, b) = (&a.0, &b.0);

    let (a00, a01, a02, a03) = ( a[0],  a[1],  a[2],  a[3]);
    let (a10, a11, a12, a13) = ( a[4],  a[5],  a[6],  a[7]);
    let (a20, a21, a22, a23) = ( a[8],  a[9], a[10], a[11]);
    let (a30, a31, a32, a33) = (a[12], a[13], a[14], a[15]);

    let (b00, b01, b02, b03) = ( b[0],  b[1],  b[2],  b[3]);
    let (b10, b11, b12, b13) = ( b[4],  b[5],  b[6],  b[7]);
    let (b20, b21, b22, b23) = ( b[8],  b[9], b[10], b[11]);
    let (b30, b31, b32, b33) = (b[12], b[13], b[14], b[15]);

    (
        a00 * b00 + a01 * b10 + a02 * b20 + a03 * b30,
        a00 * b01 + a01 * b11 + a02 * b21 + a03 * b31,
        a00 * b02 + a01 * b12 + a02 * b22 + a03 * b32,
        a00 * b03 + a01 * b13 + a02 * b23 + a03 * b33,

        a10 * b00 + a11 * b10 + a12 * b20 + a13 * b30,
        a10 * b01 + a11 * b11 + a12 * b21 + a13 * b31,
        a10 * b02 + a11 * b12 + a12 * b22 + a13 * b32,
        a10 * b03 + a11 * b13 + a12 * b23 + a13 * b33,

        a20 * b00 + a21 * b10 + a22 * b20 + a23 * b30,
        a20 * b01 + a21 * b11 + a22 * b21 + a23 * b31,
        a20 * b02 + a21 * b12 + a22 * b22 + a23 * b32,
        a20 * b03 + a21 * b13 + a22 * b23 + a23 * b33,

        a30 * b00 + a31 * b10 + a32 * b20 + a33 * b30,
        a30 * b01 + a31 * b11 + a32 * b21 + a33 * b31,
        a30 * b02 + a31 * b12 + a32 * b22 + a33 * b32,
        a30 * b03 + a31 * b13 + a32 * b23 + a33 * b33,
    )
}

#[cfg(test)]
mod test;
