use super::*;

impl Vector3f {
    #[must_use]
    pub fn normalize(&self) -> Self {
        normalize(self).into()
    }

    pub fn normalize_mut(&mut self) -> &mut Self {
        self.assign_tuple(normalize(self)); self
    }
}

fn normalize(vector: &Vector3f) -> Tuple {
    let (x, y, z) = (vector.x, vector.y, vector.z);
    let length = vector.length();

    if length <= 0.00001 {
        (0., 0., 0.)
    } else {
        (x / length, y / length, z / length)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_divides_each_component_by_the_vectors_length() {
        let vector = Vector3f::new(2., 3., 6.);
        let normalized = vector.normalize();

        assert_approx_eq!(normalized.x, 2. / 7.);
        assert_approx_eq!(normalized.y, 3. / 7.);
        assert_approx_eq!(normalized.z, 6. / 7.);
    }
}
