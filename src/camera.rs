use nalgebra_glm as glm;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    // Vectors
    pub aspect_ratio: f32,
    pub image_width: f32,
    pub image_height: f32,
    pub center: glm::Vec3,
    pub pixel00_loc: glm::Vec3,
    pub pixel_delta_u: glm::Vec3,
    pub pixel_delta_v: glm::Vec3,
}

impl Camera {
    const YAW: f32 = -90.0;
    const PITCH: f32 = 0.0;
    const MOVEMENT_SPEED: f32 = 2.5;
    const MOUSE_SENSITIVITY: f32 = 0.1;
    const ZOOM: f32 = 45.0;

    // pub fn get_view_matrix(&self) -> glm::Mat4 {
    //     glm::look_at(&self.position, &(self.position + self.front), &self.up)
    // }

    pub fn new(aspect_ratio: f32, image_width: f32) -> Self {
        let image_height = image_width / aspect_ratio;
        let center = glm::vec3(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        // Vectors across the viewport edges
        let viewport_u = glm::vec3(viewport_width, 0.0, 0.0);
        let viewport_v = glm::vec3(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calcualte the location of the upper left pixel.
        let viewport_upper_left = center - glm::vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        // let velocity = self.movement_speed * delta_time;

        // match direction {
        //     CameraMovement::FORWARD => self.center += self.front * velocity,
        //     CameraMovement::BACKWARD => self.center -= self.front * velocity,
        //     CameraMovement::LEFT => self.center -= self.right * velocity,
        //     CameraMovement::RIGHT => self.center += self.right * velocity,
        // }
    }

    pub fn process_mouse_movements(&mut self, x_offset: f32, y_offset: f32) {
        // self.yaw += x_offset * self.mouse_sensitivity;
        // self.pitch += y_offset * self.mouse_sensitivity;

        // self.pitch = self.pitch.clamp(-89.0, 89.0);

        // self.front = Self::calculate_front_vector(self.pitch, self.yaw);
        // self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        // self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }

    // pub fn _process_mouse_scroll(&mut self, y_offset: f32) {
    //     self._zoom -= y_offset;
    //     self._zoom = self._zoom.clamp(1.0, 45.0);
    // }

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
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 800.0;
        let image_height = image_width / aspect_ratio;
        let center = glm::vec3(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        // Vectors across the viewport edges
        let viewport_u = glm::vec3(viewport_width, 0.0, 0.0);
        let viewport_v = glm::vec3(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calcualte the location of the upper left pixel.
        let viewport_upper_left = center - glm::vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
