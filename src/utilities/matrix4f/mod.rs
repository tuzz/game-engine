mod convert;
mod translate;
mod rotate;
mod scale;
mod orthographic;
mod perspective;
mod multiply;
mod inverse;

use convert::Tuple;

pub struct Matrix4f(pub [f32; 16]);

impl Matrix4f {
    pub fn new(array: [f32; 16]) -> Self {
        Self(array)
    }

    pub fn identity() -> Self {
        Self([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utilities::test_helpers::*;

    #[test]
    fn it_has_immutable_functions_for_chaining() {
        let _ = Matrix4f::identity()
            .x_rotate(PI / 2.)
            .translate(1., 2., 3.)
            .y_rotate(-PI / 2.)
            .scale(4., 5., 6.)
            .z_rotate(PI)
            .inverse();
    }

    #[test]
    fn it_has_mutable_functions_for_chaining() {
        Matrix4f::identity()
            .x_rotate_mut(PI / 2.)
            .translate_mut(1., 2., 3.)
            .y_rotate_mut(-PI / 2.)
            .scale_mut(4., 5., 6.)
            .z_rotate_mut(PI)
            .inverse_mut();
    }
}
