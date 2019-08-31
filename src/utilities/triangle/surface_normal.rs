use super::*;

impl Triangle {
    pub fn surface_normal(&self) -> Vector3f {
        let u = &self.p2 - &self.p1;
        let v = &self.p3 - &self.p1;

        let x = (u.y * v.z) - (u.z * v.y);
        let y = (u.z * v.x) - (u.x * v.z);
        let z = (u.x * v.y) - (u.y * v.x);

        Vector3f { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_the_surface_normal_from_three_points_of_a_triangle() {
        let p1 = Vector3f::new(0., 0., 0.);
        let p2 = Vector3f::new(1., 0., 0.);
        let p3 = Vector3f::new(0., 1., 0.);

        let actual = Triangle::new(p1, p2, p3).surface_normal();
        let expected = Vector3f::new(0., 0., 1.);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_considers_outward_faces_to_be_when_the_winding_is_anti_clockwise() {
        let p1 = Vector3f::new(0., 0., 0.);
        let p2 = Vector3f::new(0., 1., 0.); // swapped
        let p3 = Vector3f::new(1., 0., 0.); // swapped

        let actual = Triangle::new(p1, p2, p3).surface_normal();
        let expected = Vector3f::new(0., 0., -1.);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_does_not_normalize_the_length_of_the_normal() {
        let p1 = Vector3f::new(1., 1., 1.);
        let p2 = Vector3f::new(2., 3., 4.);
        let p3 = Vector3f::new(5., 5., 5.);

        let normal = Triangle::new(p1, p2, p3).surface_normal();

        assert!(normal.length() > 9.0);
    }

    #[test]
    fn it_sets_the_normal_to_0_0_0_when_points_are_linearly_dependent() {
        let p1 = Vector3f::new(0., -3., 5.);
        let p2 = Vector3f::new(1., -3., 5.);
        let p3 = Vector3f::new(2., -3., 5.);

        let actual = Triangle::new(p1, p2, p3).surface_normal();
        let expected = Vector3f::default();

        assert_eq!(actual, expected);
    }
}
