use super::*;

impl Matrix4f {
    pub fn look_at(camera_position: &Vector3f, target: &Vector3f, up: &Vector3f) -> Self {
        let z_axis = (camera_position - target).normalize();
        let x_axis = (up * &z_axis).normalize();
        let y_axis = (&z_axis * &x_axis).normalize();

        Self([
           x_axis.x, y_axis.x, z_axis.x, camera_position.x,
           x_axis.y, y_axis.y, z_axis.y, camera_position.y,
           x_axis.z, y_axis.z, z_axis.z, camera_position.z,
                 0.,       0.,       0.,                1.,
        ])
    }
}
