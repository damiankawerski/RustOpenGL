use glam::*;

pub struct Frame {
    pub pos: Vec3,
    pub up: Vec3,
    pub forward: Vec3
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pos: Vec3::ZERO,
            up: Vec3::Y,
            forward: Vec3::Z
        }
    }

    pub fn matrix(&self) -> Mat4 {
        let right = self.forward.cross(self.up).normalize();
        let up = right.cross(self.forward).normalize();
        Mat4::from_cols(
            right.extend(0.0),
            up.extend(0.0),
            (-self.forward).extend(0.0),
            self.pos.extend(1.0)
        )
    }

}