pub mod vertex_buffer_layout;

pub struct VertexArray {
    renderer_id: u32,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self { renderer_id: vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.renderer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn add_buffer(
        &self,
        vb: &super::vertex_buffer::VertexBuffer,
        layout: &vertex_buffer_layout::VertexBufferLayout,
    ) {
        self.bind();
        vb.bind();

        let elements = layout.get_elements();

        let mut offset: i32 = 0;

        for (i, element) in elements.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    element.count,
                    element.element_type,
                    element.normalized,
                    layout.get_stride(),
                    offset as *const _,
                );
            }
            offset += element.count
                * vertex_buffer_layout::VertexBufferElement::get_size_of_type(element.element_type);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.renderer_id) }
    }
}
