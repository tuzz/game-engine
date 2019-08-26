use std::ops;
use super::*;

impl_op_ex!(* |vector: &Vector3f, scalar: f32| -> Vector3f {
    vector.scale(scalar)
});

impl_op_ex!(*= |vector: &mut Vector3f, scalar: f32| {
    vector.scale_mut(scalar);
});

impl Vector3f {
    #[must_use]
    pub fn scale(&self, scalar: f32) -> Self {
        scale(self, scalar).into()
    }

    pub fn scale_mut(&mut self, scalar: f32) -> &mut Self {
        self.assign_tuple(scale(self, scalar)); self
    }
}

fn scale(vector: &Vector3f, scalar: f32) -> Tuple {
    (
        vector.x * scalar,
        vector.y * scalar,
        vector.z * scalar,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_scales_the_components_of_the_vectors() {
        let vector = Vector3f::new(1., 2., 3.);

        let actual = vector * 5.;
        let expected = Vector3f::new(5., 10., 15.);

        assert_approx_eq!(actual.x, expected.x);
        assert_approx_eq!(actual.y, expected.y);
        assert_approx_eq!(actual.z, expected.z);
    }
}
