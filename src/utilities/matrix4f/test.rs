use super::*;
use std::f32::consts::PI;

fn assert_eq(actual: [f32; 16], expected: [f32; 16]) {
    for (a, b) in actual.iter().zip(expected.iter()) {
        assert_approx_eq!(a, b);
    }
}

mod identity {
    use super::*;

    #[test]
    fn it_returns_the_identity_matrix() {
        let matrix = Matrix4f::identity();

        assert_eq(matrix.0, [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
    }
}

mod translation {
    use super::*;

    #[test]
    fn it_returns_a_translation_matrix() {
        let (tx, ty, tz) = (1., 2., 3.);
        let matrix = Matrix4f::translation(tx, ty, tz);

        assert_eq(matrix.0, [
            1., 0., 0., tx,
            0., 1., 0., ty,
            0., 0., 1., tz,
            0., 0., 0., 1.,
        ]);
    }
}

mod x_rotation {
    use super::*;

    #[test]
    fn it_returns_a_rotation_matrix_around_the_x_axis() {
        let radians = PI / 2.0;
        let matrix = Matrix4f::x_rotation(radians);

        assert_eq(matrix.0, [
            1., 0.,  0., 0.,
            0., 0., -1., 0.,
            0., 1.,  0., 0.,
            0., 0.,  0., 1.,
        ]);
    }
}

mod y_rotation {
    use super::*;

    #[test]
    fn it_returns_a_rotation_matrix_around_the_y_axis() {
        let radians = PI / 2.0;
        let matrix = Matrix4f::y_rotation(radians);

        assert_eq(matrix.0, [
            0., 0., 1., 0.,
            0., 1., 0., 0.,
           -1., 0., 0., 0.,
            0., 0., 0., 1.,
        ]);
    }
}

mod z_rotation {
    use super::*;

    #[test]
    fn it_returns_a_rotation_matrix_around_the_z_axis() {
        let radians = PI / 2.0;
        let matrix = Matrix4f::z_rotation(radians);

        assert_eq(matrix.0, [
            0., -1., 0., 0.,
            1.,  0., 0., 0.,
            0.,  0., 1., 0.,
            0.,  0., 0., 1.,
        ]);
    }
}

mod scaling {
    use super::*;

    #[test]
    fn it_returns_a_scaling_matrix() {
        let (sx, sy, sz) = (1., 2., 3.);
        let matrix = Matrix4f::scaling(sx, sy, sz);

        assert_eq(matrix.0, [
            sx, 0., 0., 0.,
            0., sy, 0., 0.,
            0., 0., sz, 0.,
            0., 0., 0., 1.,
        ]);
    }
}

mod orthographic {
    use super::*;

    #[test]
    fn it_returns_an_orthographic_matrix() {
        let (left, right, bottom, top, near, far) = (0., 4., 0., 2., 0., 5.);
        let matrix = Matrix4f::orthographic(left, right, bottom, top, near, far);

        assert_eq(matrix.0, [
             0.5, 0.,  0.,  -1.,
             0.,  1.,  0.,  -1.,
             0.,  0., -0.4, -1.,
             0.,  0.,  0.,   1.,
        ]);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_the_matrices() {
        let a = Matrix4f([
            0., 0., 1., 1.,
            2., 2., 3., 3.,
            4., 4., 5., 5.,
            6., 6., 7., 7.,
        ]);

        let b = Matrix4f([
            0., 1., 2., 3.,
            4., 5., 6., 7.,
            0., 1., 2., 3.,
            4., 5., 6., 7.,
        ]);

        let matrix = a * b;

        assert_eq(matrix.0, [
            4.,   6.,   8.,  10.,
            20., 30.,  40.,  50.,
            36., 54.,  72.,  90.,
            52., 78., 104., 130.,
        ]);
    }

    #[test]
    fn it_works_with_references() {
        let (a, b, c) = abc();
        let _ = a * b * c;

        let (a, b, c) = abc();
        let _ = &a * &b * &c;

        let (a, b, c) = abc();
        let _ = a * &b * c;

        let (a, b, c) = abc();
        let _ = &a * &b * a * c;

        let (a, b, c) = abc();
        let _ = &a * &b * a * &c * c;
    }

    #[test]
    fn it_can_multiply_and_assign() {
        let mut matrix = Matrix4f::identity();
        let scaling = Matrix4f::scaling(1., 2., 3.);

        matrix *= &scaling;
        matrix *= &scaling;
        matrix *=  scaling;

        assert_eq(matrix.0, [
            1., 0., 0.,  0.,
            0., 8., 0.,  0.,
            0., 0., 27., 0.,
            0., 0., 0.,  1.,
        ]);
    }

    fn abc() -> (Matrix4f, Matrix4f, Matrix4f) {
        (Matrix4f::identity(), Matrix4f::identity(), Matrix4f::identity())
    }
}

mod chaining {
    use super::*;

    #[test]
    fn it_has_immutable_functions_for_chaining() {
        let matrix = Matrix4f::identity();

        let result = matrix.x_rotate(PI / 2.)
              .translate(1., 2., 3.)
              .y_rotate(-PI / 2.)
              .scale(4., 5., 6.)
              .z_rotate(PI);

        assert_approx_eq!(result.0[2], -6.0);
    }

    #[test]
    fn it_has_mutable_functions_for_chaining() {
        let mut matrix = Matrix4f::identity();

        matrix.x_rotate_mut(PI / 2.)
              .translate_mut(1., 2., 3.)
              .y_rotate_mut(-PI / 2.)
              .scale_mut(4., 5., 6.)
              .z_rotate_mut(PI);

        assert_approx_eq!(matrix.0[2], -6.0);
    }
}
