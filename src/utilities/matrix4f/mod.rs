use std::ops;

pub struct Matrix4f(pub [f32; 16]);

impl Matrix4f {
    fn identity() -> Self {
        Matrix4f([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Matrix4f([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            tx, ty, tz, 1.,
        ])
    }

    fn x_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
            1., 0., 0., 0.,
            0.,  c,  s, 0.,
            0., -s,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    fn y_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
             c, 0., -s, 0.,
            0., 1., 0., 0.,
             s, 0.,  c, 0.,
            0., 0., 0., 1.,
        ])
    }

    fn z_rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();

        Matrix4f([
             c,  s, 0., 0.,
            -s,  c, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Matrix4f([
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, 0.,
            0., 0., 0., 1.,
        ])
    }
}

impl_op_ex!(* |a: &Matrix4f, b: &Matrix4f| -> Matrix4f {
    let (a, b) = (a.0, b.0);

    let (a00, a01, a02, a03) = ( a[0],  a[1],  a[2],  a[3]);
    let (a10, a11, a12, a13) = ( a[4],  a[5],  a[6],  a[7]);
    let (a20, a21, a22, a23) = ( a[8],  a[9], a[10], a[11]);
    let (a30, a31, a32, a33) = (a[12], a[13], a[14], a[15]);

    let (b00, b01, b02, b03) = ( b[0],  b[1],  b[2],  b[3]);
    let (b10, b11, b12, b13) = ( b[4],  b[5],  b[6],  b[7]);
    let (b20, b21, b22, b23) = ( b[8],  b[9], b[10], b[11]);
    let (b30, b31, b32, b33) = (b[12], b[13], b[14], b[15]);

    Matrix4f([
        // First row:
        a00 * b00 + a01 * b10 + a02 * b20 + a03 * b30,
        a00 * b01 + a01 * b11 + a02 * b21 + a03 * b31,
        a00 * b02 + a01 * b12 + a02 * b22 + a03 * b32,
        a00 * b03 + a01 * b13 + a02 * b23 + a03 * b33,

        // Sebond row:
        a10 * b00 + a11 * b10 + a12 * b20 + a13 * b30,
        a10 * b01 + a11 * b11 + a12 * b21 + a13 * b31,
        a10 * b02 + a11 * b12 + a12 * b22 + a13 * b32,
        a10 * b03 + a11 * b13 + a12 * b23 + a13 * b33,

        // Third row:
        a20 * b00 + a21 * b10 + a22 * b20 + a23 * b30,
        a20 * b01 + a21 * b11 + a22 * b21 + a23 * b31,
        a20 * b02 + a21 * b12 + a22 * b22 + a23 * b32,
        a20 * b03 + a21 * b13 + a22 * b23 + a23 * b33,

        // Fourth row:
        a30 * b00 + a31 * b10 + a32 * b20 + a33 * b30,
        a30 * b01 + a31 * b11 + a32 * b21 + a33 * b31,
        a30 * b02 + a31 * b12 + a32 * b22 + a33 * b32,
        a30 * b03 + a31 * b13 + a32 * b23 + a33 * b33,
    ])
});

#[cfg(test)]
mod test;
