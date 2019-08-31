use super::*;

impl Triangle {
    pub fn area(&self) -> f32 {
        let a = &self.p2 - &self.p1;
        let b = &self.p3 - &self.p1;

        (a * b).length() / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::SQRT_2;

    #[test]
    fn it_calculates_the_expected_solution_from_the_youtube_video() {
        // https://www.youtube.com/watch?v=QHaAoQQy07I

        let triangle = Triangle::new(
            Vector3f::new(-1., 0., 1.),
            Vector3f::new(0., 2., 2.),
            Vector3f::new(0., -1., 2.),
        );

        assert_eq!(triangle.area(), 3.0 * SQRT_2 / 2.0);
    }
}
