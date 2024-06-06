pub struct IndexBuffer {
    renderer_id: u32,
    count: i32,
}

impl IndexBuffer {
    pub fn new<T>(data: &[T]) -> IndexBuffer {
        let mut ib: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut ib);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ib);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        };
        Self {
            renderer_id: ib,
            count: data.len() as i32,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id) }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.renderer_id) }
    }
}
