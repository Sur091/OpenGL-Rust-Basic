use super::{index_buffer::IndexBuffer, shader::Shader, vertex_array::VertexArray};

pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub struct Renderer {}

impl Renderer {
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&self, vertex_array: &VertexArray, index_buffer: &IndexBuffer, shader: &Shader) {
        shader.bind();
        vertex_array.bind();
        index_buffer.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                index_buffer.get_count(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }

    pub fn clear_color(&self, c: Color) {
        unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
    }
}
