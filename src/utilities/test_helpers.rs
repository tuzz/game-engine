pub use std::f32::consts::PI;

pub fn assert_approx_eq_slice(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());

    for (a, b) in actual.iter().zip(expected.iter()) {
        assert_approx_eq!(a, b);
    }
}
