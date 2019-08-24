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
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            tx, ty, tz, 1.,
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
            1.,  0., 0., 0.,
            0.,  0., 1., 0.,
            0., -1., 0., 0.,
            0.,  0., 0., 1.,
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
            0., 0., -1., 0.,
            0., 1.,  0., 0.,
            1., 0.,  0., 0.,
            0., 0.,  0., 1.,
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
            0., 1., 0., 0.,
           -1., 0., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
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
