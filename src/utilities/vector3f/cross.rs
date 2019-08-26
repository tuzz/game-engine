use std::ops;
use super::*;

impl_op_ex!(* |left: &Vector3f, right: &Vector3f| -> Vector3f {
    left.cross(right)
});

impl_op_ex!(*= |left: &mut Vector3f, right: &Vector3f| {
    left.cross_mut(right);
});

impl Vector3f {
    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        cross(self, other).into()
    }

    pub fn cross_mut(&mut self, other: &Self) -> &mut Self {
        self.assign_tuple(cross(self, other)); self
    }
}

fn cross(a: &Vector3f, b: &Vector3f) -> Tuple {
    let x = a.y * b.z - a.z * b.y;
    let y = a.z * b.x - a.x * b.z;
    let z = a.x * b.y - a.y * b.x;

    (x, y, z)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_the_cross_product() {
        let a = Vector3f::new(1., 2., 3.);
        let b = Vector3f::new(3., 0., 1.);

        let actual = a * b;
        let expected = Vector3f::new(2., 8., -6.);

        assert_approx_eq!(actual.x, expected.x);
        assert_approx_eq!(actual.y, expected.y);
        assert_approx_eq!(actual.z, expected.z);
    }
}
