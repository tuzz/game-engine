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
}

#[cfg(test)]
mod test;
