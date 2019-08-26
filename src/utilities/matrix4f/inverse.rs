use super::*;

impl Matrix4f {
    #[must_use]
    pub fn inverse(&self) -> Self {
        inverse(self).into()
    }

    pub fn inverse_mut(&mut self) -> &mut Self {
        self.assign_tuple(inverse(self)); self
    }
}

// The inverse function is adapted from this answer:
// https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix#answer-44446912

fn inverse(x: &[f32; 16]) -> Tuple {
    let (a, b, c, d) = ( x[0],  x[1],  x[2],  x[3]);
    let (e, f, g, h) = ( x[4],  x[5],  x[6],  x[7]);
    let (i, j, k, l) = ( x[8],  x[9], x[10], x[11]);
    let (m, n, o, p) = (x[12], x[13], x[14], x[15]);

    let (t0, t1, t2) = (k * p - l * o,  j * p - l * n,  j * o - k * n);
    let (t3, t4, t5) = (i * p - l * m,  i * o - k * m,  i * n - j * m);
    let (t6, t7, t8) = (g * p - h * o,  f * p - h * n,  f * o - g * n);
    let (t9, u0, u1) = (g * l - h * k,  f * l - h * j,  f * k - g * j);
    let (u2, u3, u4) = (e * p - h * m,  e * o - g * m,  e * l - h * i);
    let (u5, u6, u7) = (e * k - g * i,  e * n - f * m,  e * j - f * i);

    let (d0, d1) = (f * t0 - g * t1 + h * t2, e * t0 - g * t3 + h * t4);
    let (d2, d3) = (e * t1 - f * t3 + h * t5, e * t2 - f * t4 + g * t5);

    let det = 1.0 / (a * d0 - b * d1 + c * d2 - d * d3);

    (det *   (f *  t0 - g * t1 + h * t2), det * - (b * t0 - c * t1 + d * t2),
     det *   (b *  t6 - c * t7 + d * t8), det * - (b * t9 - c * u0 + d * u1),
     det * - (e *  t0 - g * t3 + h * t4), det *   (a * t0 - c * t3 + d * t4),
     det * - (a *  t6 - c * u2 + d * u3), det *   (a * t9 - c * u4 + d * u5),
     det *   (e *  t1 - f * t3 + h * t5), det * - (a * t1 - b * t3 + d * t5),
     det *   (a *  t7 - b * u2 + d * u6), det * - (a * u0 - b * u4 + d * u7),
     det * - (e *  t2 - f * t4 + g * t5), det *   (a * t2 - b * t4 + c * t5),
     det * - (a *  t8 - b * u3 + c * u6), det *   (a * u1 - b * u5 + c * u7))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utilities::test_helpers::*;

    #[test]
    fn it_returns_a_matrix_that_reverses_the_transforms() {
        let matrix = Matrix4f::identity()
            .translate(1., 2., 3.)
            .x_rotate(PI / 3.)
            .scale(2., -5., 10.);

        let expected_inverse = Matrix4f::identity()
            .scale(0.5, -0.2, 0.1)
            .x_rotate(-PI / 3.)
            .translate(-1., -2., -3.);

        assert_approx_eq_slice(&matrix.inverse().0, &expected_inverse.0);
    }
}
