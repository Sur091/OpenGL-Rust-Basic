use nalgebra_glm as glm;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    // Vectors
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    // Angles
    yaw: f32,
    pitch: f32,
    // Camera Options
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Camera {
    const YAW: f32 = -90.0;
    const PITCH: f32 = 0.0;
    const MOVEMENT_SPEED: f32 = 2.5;
    const MOUSE_SENSITIVITY: f32 = 0.1;
    const ZOOM: f32 = 45.0;


    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let world_up = up;

        let front = Self::calculate_front_vector(pitch, yaw);
        let right = glm::normalize(&glm::cross(&front, &world_up));
        let up = glm::normalize(&glm::cross(&right, &front));

        let movement_speed = Self::MOVEMENT_SPEED;
        let mouse_sensitivity = Self::MOUSE_SENSITIVITY;
        let zoom = Self::ZOOM;

        Self {
            position,
            front,
            up,
            right,
            world_up,

            yaw,
            pitch,

            movement_speed,
            mouse_sensitivity,
            zoom,
        }
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        
        match direction {
            CameraMovement::FORWARD => self.position += self.front * velocity,
            CameraMovement::BACKWARD => self.position -= self.front * velocity,
            CameraMovement::LEFT => self.position -= self.right * velocity,
            CameraMovement::RIGHT => self.position += self.right * velocity,
        }
    }

    pub fn process_mouse_movements(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset * self.mouse_sensitivity;
        self.pitch += y_offset * self.mouse_sensitivity;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.front = Self::calculate_front_vector(self.pitch, self.yaw);
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;
        self.zoom = self.zoom.clamp(1.0, 45.0);
    }

    fn calculate_front_vector(pitch: f32, yaw: f32) -> glm::Vec3 {
        let (pitch, yaw) = (pitch.to_radians(), yaw.to_radians());
        let front = glm::vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        );
        glm::normalize(&front)
    }
}

impl Default for Camera {

    fn default() -> Self {
        let position = glm::vec3(0.0, 0.0, 0.0);
        let world_up = glm::vec3(0.0, 1.0, 0.0);

        let yaw = Self::YAW;
        let pitch = Self::PITCH;

        let front = Self::calculate_front_vector(pitch, yaw);
        let right = glm::normalize(&glm::cross(&front, &world_up));
        let up = glm::normalize(&glm::cross(&right, &front));

        let movement_speed = Self::MOVEMENT_SPEED;
        let mouse_sensitivity = Self::MOUSE_SENSITIVITY;
        let zoom = Self::ZOOM;
        // let direction = glm::normalize(&(position - target));
        // let right = glm::normalize(&glm::cross(&up, &direction));

        Self {
            position,
            front,
            up,
            right,
            world_up,

            yaw,
            pitch,

            movement_speed,
            mouse_sensitivity,
            zoom,
        }
    }
}
