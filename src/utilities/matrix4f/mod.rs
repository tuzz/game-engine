pub struct Matrix4f([f32; 16]);

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

#[cfg(test)]
mod test;
