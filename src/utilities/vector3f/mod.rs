mod convert;
mod add;
mod subtract;
mod scale;
mod dot;
mod cross;
mod length;
mod normalize;
mod angle;

use convert::Tuple;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_has_immutable_functions_for_chaining() {
        let _ = Vector3f::new(1., 2., 3.)
              + Vector3f::new(1., 2., 3.)
              - Vector3f::new(1., 2., 3.)
              * Vector3f::new(1., 2., 3.)
              * 1.;

        let _ = Vector3f::new(1., 2., 3.)
              .add(&Vector3f::new(1., 2., 3.))
              .subtract(&Vector3f::new(1., 2., 3.))
              .cross(&Vector3f::new(1., 2., 3.))
              .scale(1.)
              .normalize();
    }

    #[test]
    fn it_has_mutable_functions_for_chaining() {
        let mut vector = Vector3f::new(1., 2., 3.);

        vector += Vector3f::new(1., 2., 3.);
        vector -= Vector3f::new(1., 2., 3.);
        vector *= Vector3f::new(1., 2., 3.);
        vector *= 1.;

        Vector3f::new(1., 2., 3.)
            .add_mut(&Vector3f::new(1., 2., 3.))
            .subtract_mut(&Vector3f::new(1., 2., 3.))
            .cross_mut(&Vector3f::new(1., 2., 3.))
            .scale_mut(1.)
            .normalize_mut();
    }
}
