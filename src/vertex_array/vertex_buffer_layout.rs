pub struct VertexBufferElement {
    pub element_type: u32,
    pub count: i32,
    pub normalized: u8,
}

impl VertexBufferElement {
    pub fn get_size_of_type(element_type: u32) -> i32 {
        match element_type {
            gl::FLOAT => 4,
            gl::UNSIGNED_INT => 4,
            gl::UNSIGNED_BYTE => 1,
            _ => panic!("The type {} doesn't exits", element_type),
        }
    }
}

pub struct VertexBufferLayout {
    elements: Vec<VertexBufferElement>,
    stride: i32,
}

impl VertexBufferLayout {
    pub fn new() -> VertexBufferLayout {
        VertexBufferLayout {
            elements: vec![],
            stride: 0,
        }
    }

    pub fn push_f32(&mut self, count: i32) {
        self.elements.push(VertexBufferElement {
            element_type: gl::FLOAT,
            count,
            normalized: gl::FALSE,
        });
        self.stride += VertexBufferElement::get_size_of_type(gl::UNSIGNED_INT) * count;
    }

    #[inline]
    pub fn get_elements(&self) -> &[VertexBufferElement] {
        &self.elements
    }

    #[inline]
    pub fn get_stride(&self) -> i32 {
        self.stride
    }
}
