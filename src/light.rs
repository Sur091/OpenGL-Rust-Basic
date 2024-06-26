use nalgebra_glm as glm;

use super::{
    shader::Shader,
    vertex_array::VertexArray,
};

pub struct Light {
    pub shader: Shader,
    pub vertex_array: VertexArray, 
    pub position: glm::Vec3,
    scale: glm::Vec3,
}

impl Light {
    pub fn new(shader: Shader, vertex_array: VertexArray, position: glm::Vec3) -> Self {
        Self {
            shader,
            vertex_array,
            position: position,
            scale: glm::vec3(0.2, 0.2, 0.2),
        }
    }
    pub fn get_model_matrix(&self) -> glm::Mat4 {
        let model = glm::identity();
        let model = glm::translate(&model, &self.position);
        let model = glm::scale(&model, &self.scale);
        model
    }

    // pub fn move_light(&mut self, new_pos: glm::Vec3) {
    //     self.position = new_pos;
    // }

    pub fn bind(&mut self, view: &glm::Mat4, projection: &glm::Mat4) {
        self.shader.bind();
        self.vertex_array.bind();
        self.shader.set_uniform_mat4f("u_model", &self.get_model_matrix());
        self.shader.set_uniform_mat4f("u_view", &view);
        self.shader.set_uniform_mat4f("u_projection", &projection);
    }
}