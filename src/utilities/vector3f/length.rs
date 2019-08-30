use super::*;

impl Vector3f {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        let (x, y, z) = (self.x, self.y, self.z);

        x * x + y * y + z * z
    }
}
