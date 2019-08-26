use std::ops;
use super::*;

impl_op_ex!(+ |left: &Vector3f, right: &Vector3f| -> Vector3f {
    add(left, right).into()
});

impl_op_ex!(+= |left: &mut Vector3f, right: &Vector3f| {
    left.assign_tuple(add(left, right));
});

fn add(left: &Vector3f, right: &Vector3f) -> Tuple {
    (
        left.x + right.x,
        left.y + right.y,
        left.z + right.z,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_adds_the_components_of_the_vectors() {
        let a = Vector3f::new(1., 2., 3.);
        let b = Vector3f::new(4., 5., 6.);

        let actual = a + b;
        let expected = Vector3f::new(5., 7., 9.);

        assert_approx_eq!(actual.x, expected.x);
        assert_approx_eq!(actual.y, expected.y);
        assert_approx_eq!(actual.z, expected.z);
    }
}
