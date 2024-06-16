use super::{
    vertex_array::VertexArray,
    index_buffer::IndexBuffer,
    shader::Shader
};

pub struct Renderer {}

impl Renderer {
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, vertex_array: &VertexArray, index_buffer: &IndexBuffer, shader: &Shader) {
        shader.bind();
        vertex_array.bind();
        index_buffer.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, index_buffer.get_count(), gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}