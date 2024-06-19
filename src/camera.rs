use nalgebra_glm as glm;

pub struct Camera {
    position: glm::Vec3,
    target: glm::Vec3,
    direction: glm::Vec3,
    right: glm::Vec3,
    up: glm::Vec3,
}

impl Camera {
    fn look_at_matrix(&self) -> glm::Mat4 {
        let view = glm::look_at(&self.position, &self.target, &self.up);
        view
    }
}

impl Default for Camera {
    fn default() -> Self {
        let position = glm::vec3(0.0, 0.0, -3.0);
        let target = glm::vec3(0.0, 0.0, 0.0);
        let direction = glm::normalize(&(position - target));
        let right = glm::normalize(&glm::cross(&glm::vec3(0.0, 1.0, 0.0), &direction));
        let up = glm::cross(&direction, &right);

        Self {
            position,
            target,
            direction,
            right,
            up,
        }
    }
}
