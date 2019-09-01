use super::*;

impl Matrix4f {
    pub fn position(&self) -> Vector3f {
        let data = self.0;
        let (x, y, z) = (data[3], data[7], data[11]);

        Vector3f::new(x, y, z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_translation_position_of_the_matrix() {
        let matrix = Matrix4f::translation(1., 2., 3.);

        assert_eq!(matrix.position(), Vector3f::new(1., 2., 3.));
    }
}
