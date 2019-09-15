use super::*;

pub type Tuple = (f32, f32, f32);

impl From<Tuple> for Vector3f {
    fn from(t: Tuple) -> Self {
        Self { x: t.0, y: t.1, z: t.2 }
    }
}

impl From<[f32; 3]> for Vector3f {
    fn from(arr: [f32; 3]) -> Self {
        Self { x: arr[0], y: arr[1], z: arr[2] }
    }
}

impl Vector3f {
    pub fn assign_tuple(&mut self, t: Tuple) {
        self.x = t.0;
        self.y = t.1;
        self.z = t.2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_converts_from_a_tuple_to_a_vector() {
        let vector: Vector3f = (1., 2., 3.).into();

        assert_eq!(vector.x, 1.);
        assert_eq!(vector.y, 2.);
        assert_eq!(vector.z, 3.);
    }

    #[test]
    fn it_assigns_values_from_a_tuple_to_the_vector() {
        let mut vector = Vector3f::new(1., 2., 3.);

        vector.assign_tuple((4., 5., 6.));

        assert_eq!(vector.x, 4.);
        assert_eq!(vector.y, 5.);
        assert_eq!(vector.z, 6.);
    }
}
