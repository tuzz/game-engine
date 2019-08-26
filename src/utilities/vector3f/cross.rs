use super::*;

impl Vector3f {
    pub fn cross(&self, other: &Self) -> Self {
        cross(self, other).into()
    }

    pub fn cross_mut(&mut self, other: &Self) -> &mut Self {
        self.assign_tuple(cross(self, other)); self
    }
}

fn cross(a: &[f32; 3], b: &[f32; 3]) -> Tuple {
    let x = a[1] * b[2] - a[2] * b[1];
    let y = a[2] * b[0] - a[0] * b[2];
    let z = a[0] * b[1] - a[1] * b[0];

    (x, y, z)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utilities::test_helpers::*;

    #[test]
    fn it_calculates_the_cross_product() {
        let a = Vector3f::new(1., 2., 3.);
        let b = Vector3f::new(3., 0., 1.);

        assert_approx_eq_slice(&a.cross(&b).0, &[2., 8., -6.]);
    }
}
