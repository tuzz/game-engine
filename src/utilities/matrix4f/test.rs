use super::*;

mod identity {
    use super::*;

    #[test]
    fn it_returns_the_identity_matrix() {
        let matrix = Matrix4f::identity();

        assert_eq!(matrix.0, [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]);
    }
}
