use glam::{Mat4, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 3.0),
            up: Vec3::Y,
            forward: Vec3::NEG_Z,
        }
    }

    pub fn rotate(&mut self, dyaw: f32, dpitch: f32) {
        self.forward = Mat4::from_rotation_y(dyaw)
            .transform_vector3(self.forward)
            .normalize();

        let right = self.forward.cross(self.up).normalize();
        let pitched = Mat4::from_axis_angle(right, dpitch)
            .transform_vector3(self.forward)
            .normalize();

        if pitched.dot(self.up).abs() < 0.999 {
            self.forward = pitched;
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.pos + self.forward, self.up)
    }
}
