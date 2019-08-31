use super::*;

impl Vector3f {
    pub fn angle(&self, other: &Self) -> f32 {
        let numerator = self.dot(other);
        let denominator = self.length() * other.length();

        (numerator / denominator).acos()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_the_expected_solution_from_the_youtube_video() {
        // https://www.youtube.com/watch?v=aXUq_EnX5Wk

        let u = Vector3f::new(3., 4., 0.);
        let v = Vector3f::new(5., -12., 0.);

        assert_eq!(u.angle(&v), (-33_f32 / 65.).acos());
    }
}
