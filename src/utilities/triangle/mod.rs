mod area;
mod surface_normal;

use super::Vector3f;

pub struct Triangle {
    pub p1: Vector3f,
    pub p2: Vector3f,
    pub p3: Vector3f,
}

impl Triangle {
    pub fn new(p1: Vector3f, p2: Vector3f, p3: Vector3f) -> Self {
        Self { p1, p2, p3 }
    }
}
