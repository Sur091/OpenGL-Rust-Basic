use std::ffi::CString;

use nalgebra_glm as glm;

use super::{
    camera::Camera, index_buffer::IndexBuffer, shader::Shader, texture::Texture,
    vertex_array::vertex_buffer_layout::VertexBufferLayout, vertex_array::VertexArray,
    vertex_buffer::VertexBuffer,
};

pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub struct Renderer {
    // _gl_display: glutin::display::Display,
    _vertex_buffer: VertexBuffer,
    vertex_array: VertexArray,
    index_buffer: IndexBuffer,
    shader: Shader,
    textures: Vec<Texture>,
    pub camera: Camera,
    projection: glm::Mat4,
    start_time: std::time::Instant,
}

impl Renderer {
    #[rustfmt::skip]
    const VERTICES: [f32; 36*5] = [
        -0.5, -0.5, -0.5, 0.0, 0.0,
         0.5, -0.5, -0.5, 1.0, 0.0,
         0.5,  0.5, -0.5, 1.0, 1.0,
         0.5,  0.5, -0.5, 1.0, 1.0,
        -0.5,  0.5, -0.5, 0.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 0.0,
        //
        -0.5, -0.5,  0.5, 0.0, 0.0,
         0.5, -0.5,  0.5, 1.0, 0.0,
         0.5,  0.5,  0.5, 1.0, 1.0,
         0.5,  0.5,  0.5, 1.0, 1.0,
        -0.5,  0.5,  0.5, 0.0, 1.0,
        -0.5, -0.5,  0.5, 0.0, 0.0,
        //
        -0.5,  0.5,  0.5, 1.0, 0.0,
        -0.5,  0.5, -0.5, 1.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 1.0,
        -0.5, -0.5,  0.5, 0.0, 0.0,
        -0.5,  0.5,  0.5, 1.0, 0.0,
        //
         0.5,  0.5,  0.5, 1.0, 0.0,
         0.5,  0.5, -0.5, 1.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5,  0.5, 0.0, 0.0,
         0.5,  0.5,  0.5, 1.0, 0.0,
        //
        -0.5, -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, -0.5, 1.0, 1.0,
         0.5, -0.5,  0.5, 1.0, 0.0,
         0.5, -0.5,  0.5, 1.0, 0.0,
        -0.5, -0.5,  0.5, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0,
        //
        -0.5,  0.5, -0.5, 0.0, 1.0,
         0.5,  0.5, -0.5, 1.0, 1.0,
         0.5,  0.5,  0.5, 1.0, 0.0,
         0.5,  0.5,  0.5, 1.0, 0.0,
        -0.5,  0.5,  0.5, 0.0, 0.0,
        -0.5,  0.5, -0.5, 0.0, 1.0,
    ];
    const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];

    extern "system" fn message_callback(
        source: gl::types::GLenum,
        ty: gl::types::GLenum,
        _id: gl::types::GLuint,
        severity: gl::types::GLenum,
        _length: gl::types::GLsizei,
        message: *const gl::types::GLchar,
        _user_param: *mut std::os::raw::c_void,
    ) {
        let message_str = unsafe { std::ffi::CStr::from_ptr(message).to_string_lossy() };

        let message_source = match source {
            gl::DEBUG_SOURCE_API => "OpenGL API",
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
            gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
            gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
            gl::DEBUG_SOURCE_APPLICATION => "Application",
            gl::DEBUG_SOURCE_OTHER => "Other",
            _ => "Unknown",
        };

        let message_type = match ty {
            gl::DEBUG_TYPE_ERROR => "Error",
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
            gl::DEBUG_TYPE_PORTABILITY => "Portability",
            gl::DEBUG_TYPE_PERFORMANCE => "Performance",
            gl::DEBUG_TYPE_MARKER => "Marker",
            gl::DEBUG_TYPE_OTHER => "Other",
            gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
            gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
            _ => "Unknown",
        };

        let message_severity = match severity {
            gl::DEBUG_SEVERITY_HIGH => "High",
            gl::DEBUG_SEVERITY_MEDIUM => "Medium",
            gl::DEBUG_SEVERITY_LOW => "Low",
            gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
            _ => "Unknown",
        };

        panic!(
            "OpenGL Debug Message: Source: {}, Type: {}, Severity: {}, Message: {}",
            message_source, message_type, message_severity, message_str
        );
    }

    fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
        let v = unsafe { gl::GetString(name) };
        let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
        v.to_str().unwrap()
    }

    pub fn new(gl_display: &glutin::display::Display) -> Self {
        use glutin::display::GlDisplay;

        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str())
        });

        println!("Running on {}", Self::gl_get_string(gl::RENDERER));
        println!("OpenGl Version {}", Self::gl_get_string(gl::VERSION));
        println!(
            "Shaders version on {}",
            Self::gl_get_string(gl::SHADING_LANGUAGE_VERSION)
        );

        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        #[cfg(debug_assertions)]
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(Self::message_callback), std::ptr::null());
        }

        const VERT_SHADER_PATH: &str = "./src/shader/vertex_shader.glsl";

        const FRAG_SHADER_PATH: &str = "./src/shader/fragment_shader.glsl";

        let mut shader = Shader::new(VERT_SHADER_PATH, FRAG_SHADER_PATH);

        // const SIZE: f32 = 0.5;

        let vertex_array = VertexArray::new();
        // let light_vertex_array = VertexArray::new();

        let vbo = VertexBuffer::new(&Self::VERTICES);

        let index_buffer = IndexBuffer::new(&Self::INDICES);

        let mut layout = VertexBufferLayout::new();
        layout.push_f32(3);
        layout.push_f32(2);

        vertex_array.add_buffer(&vbo, &layout);

        let texture0 = Texture::new("./assets/FlowerPattern2.png");

        let texture1 = Texture::new("./assets/BlueFlowers.jpg");

        shader.bind();
        shader.set_uniform_1i("texture0", 0);
        shader.set_uniform_1i("texture1", 1);

        vbo.unbind();
        vertex_array.unbind();
        index_buffer.unbind();

        let camera = Camera::default();

        Self::clear_color(Color(0.6, 0.5, 0.1, 1.0));

        // gl_display.
        let aspect_ratio = 1.0;
        let projection = glm::perspective(glm::quarter_pi::<f32>(), aspect_ratio, 000.1, 100.0);

        let start_time = std::time::Instant::now();

        Self {
            // _gl_display: gl_display.clone(),
            vertex_array,
            _vertex_buffer: vbo,
            shader,
            index_buffer,
            textures: vec![texture0, texture1],
            camera,
            projection,
            start_time,
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn bind(&mut self) {
        self.shader.bind();
        self.vertex_array.bind();
        self.index_buffer.bind();

        for (i, texture) in self.textures.iter().enumerate() {
            texture.bind(i as u32);
        }

        self.shader
            .set_uniform_mat4f("u_view", &self.camera.get_view_matrix());
        self.shader
            .set_uniform_mat4f("u_projection", &self.projection);
    }

    pub fn draw(&mut self) {
        self.bind();
        self.clear();
        
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_buffer.get_count(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }


    }

    pub fn draw_array(&mut self) {
        self.bind();
        self.clear();

        
        let cubes: [glm::Vec3; 6] = [
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.0, 2.0, 3.0),
            glm::vec3(10.0, -7.0, 2.0),
            glm::vec3(3.0, -9.0, 1.0),
            glm::vec3(-6.0, 0.0, -7.0),
            glm::vec3(-1.0, 0.0, -8.0),
        ];

        for cube_orgin in cubes {
            let model = glm::identity();
            let model = glm::translate(&model, &cube_orgin);
            let model = glm::rotate(
                &model,
                -55.0 * glm::pi::<f32>() / 180.0 * self.start_time.elapsed().as_secs_f32(),
                &glm::vec3(0.5, 1.0, 0.0),
            );
            self.shader.set_uniform_mat4f("u_model", &model);
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, Self::VERTICES.len() as i32) }
        }

    }

    pub fn clear_color(c: Color) {
        unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
    }
}
