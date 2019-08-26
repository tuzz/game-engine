use std::ops;
use super::*;

impl_op_ex!(* |left: &Matrix4f, right: &Matrix4f| -> Matrix4f {
    left.multiply(right)
});

impl_op_ex!(*= |left: &mut Matrix4f, right: &Matrix4f| {
    left.multiply_mut(right);
});

impl Matrix4f {
    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        multiply(self, other).into()
    }

    pub fn multiply_mut(&mut self, other: &Self) -> &mut Self {
        self.assign_tuple(multiply(self, other)); self
    }
}

fn multiply(a: &[f32; 16], b: &[f32; 16]) -> Tuple {
    let (a00, a01, a02, a03) = ( a[0],  a[1],  a[2],  a[3]);
    let (a10, a11, a12, a13) = ( a[4],  a[5],  a[6],  a[7]);
    let (a20, a21, a22, a23) = ( a[8],  a[9], a[10], a[11]);
    let (a30, a31, a32, a33) = (a[12], a[13], a[14], a[15]);

    let (b00, b01, b02, b03) = ( b[0],  b[1],  b[2],  b[3]);
    let (b10, b11, b12, b13) = ( b[4],  b[5],  b[6],  b[7]);
    let (b20, b21, b22, b23) = ( b[8],  b[9], b[10], b[11]);
    let (b30, b31, b32, b33) = (b[12], b[13], b[14], b[15]);

    (
        a00 * b00 + a01 * b10 + a02 * b20 + a03 * b30,
        a00 * b01 + a01 * b11 + a02 * b21 + a03 * b31,
        a00 * b02 + a01 * b12 + a02 * b22 + a03 * b32,
        a00 * b03 + a01 * b13 + a02 * b23 + a03 * b33,

        a10 * b00 + a11 * b10 + a12 * b20 + a13 * b30,
        a10 * b01 + a11 * b11 + a12 * b21 + a13 * b31,
        a10 * b02 + a11 * b12 + a12 * b22 + a13 * b32,
        a10 * b03 + a11 * b13 + a12 * b23 + a13 * b33,

        a20 * b00 + a21 * b10 + a22 * b20 + a23 * b30,
        a20 * b01 + a21 * b11 + a22 * b21 + a23 * b31,
        a20 * b02 + a21 * b12 + a22 * b22 + a23 * b32,
        a20 * b03 + a21 * b13 + a22 * b23 + a23 * b33,

        a30 * b00 + a31 * b10 + a32 * b20 + a33 * b30,
        a30 * b01 + a31 * b11 + a32 * b21 + a33 * b31,
        a30 * b02 + a31 * b12 + a32 * b22 + a33 * b32,
        a30 * b03 + a31 * b13 + a32 * b23 + a33 * b33,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utilities::test_helpers::*;

    #[test]
    fn it_multiplies_the_matrices() {
        let mut matrix = Matrix4f::identity();
        let scaling = Matrix4f::scaling(1., 2., 3.);

        matrix *= &scaling;
        matrix *= &scaling;
        matrix *=  scaling;

        assert_approx_eq_slice(&matrix.0, &[
            1., 0., 0.,  0.,
            0., 8., 0.,  0.,
            0., 0., 27., 0.,
            0., 0., 0.,  1.,
        ]);
    }
}
