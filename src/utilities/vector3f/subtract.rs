use std::ops;
use super::*;

impl_op_ex!(- |left: &Vector3f, right: &Vector3f| -> Vector3f {
    left.subtract(right)
});

impl_op_ex!(-= |left: &mut Vector3f, right: &Vector3f| {
    left.subtract_mut(right);
});

impl Vector3f {
    #[must_use]
    pub fn subtract(&self, other: &Self) -> Self {
        subtract(self, other).into()
    }

    pub fn subtract_mut(&mut self, other: &Self) -> &mut Self {
        self.assign_tuple(subtract(self, other)); self
    }
}

fn subtract(left: &Vector3f, right: &Vector3f) -> Tuple {
    (
        left.x - right.x,
        left.y - right.y,
        left.z - right.z,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_subtracts_the_components_of_the_vectors() {
        let a = Vector3f::new(1., 2., 3.);
        let b = Vector3f::new(6., 5., 4.);

        let actual = a - b;
        let expected = Vector3f::new(-5., -3., -1.);

        assert_approx_eq!(actual.x, expected.x);
        assert_approx_eq!(actual.y, expected.y);
        assert_approx_eq!(actual.z, expected.z);
    }
}
