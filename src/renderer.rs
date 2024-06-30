use std::ffi::CString;

use nalgebra_glm as glm;

use super::{
    camera::Camera, 
    index_buffer::IndexBuffer, 
    shader::Shader, 
    texture::Texture,
    vertex_array::vertex_buffer_layout::VertexBufferLayout, 
    vertex_array::VertexArray,
    vertex_buffer::VertexBuffer,
    // light::Light,
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
    // light: Light,
}

impl Renderer {
    #[rustfmt::skip]
    const VERTICES: [f32; 8] = [
        -1.0, -1.0,
         1.0, -1.0,
         1.0,  1.0,
        -1.0,  1.0,
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

        const VERT_SHADER_PATH: &str = "./src/shader/cubes.vert";

        const FRAG_SHADER_PATH: &str = "./src/shader/cubes.frag";

        let cube_shader = Shader::new(VERT_SHADER_PATH, FRAG_SHADER_PATH);
        // let mut object_shader = Shader::new("./src/shader/object.vert", "./src/shader/object.frag");
        // let light_shader = Shader::new("./src/shader/light.vert", "./src/shader/light.frag");

        // object_shader.bind();
        // object_shader.set_uniform_3f("u_object_color", 1.0, 0.5, 0.31);
        // object_shader.set_uniform_3f("u_light_color", 1.0, 1.0, 1.0);

        // const SIZE: f32 = 0.5;

        let vertex_array = VertexArray::new();
        let light_vertex_array = VertexArray::new();

        let vertex_buffer = VertexBuffer::new(&Self::VERTICES);

        let index_buffer = IndexBuffer::new(&Self::INDICES);

        let mut layout = VertexBufferLayout::new();
        layout.push_f32(2);

        vertex_array.add_buffer(&vertex_buffer, &layout);
        light_vertex_array.add_buffer(&vertex_buffer, &layout);

        vertex_buffer.unbind();
        vertex_array.unbind();
        index_buffer.unbind();

        let camera = Camera::default();

        Self::clear_color(Color(0.0, 0.0, 0.0, 1.0));

        // gl_display.
        let aspect_ratio = 1.0;
        let projection = glm::perspective(glm::quarter_pi::<f32>(), aspect_ratio, 000.1, 100.0);

        let start_time = std::time::Instant::now();

        Self {
            // _gl_display: gl_display.clone(),
            vertex_array,
            _vertex_buffer: vertex_buffer,
            shader: cube_shader,
            index_buffer,
            textures: vec![],
            camera,
            projection,
            start_time,
            // light: Light::new(light_shader, light_vertex_array, glm::vec3(-0.5, 1.0, -7.5)),
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

    fn draw_elements(&mut self) {
        // self.clear();
        
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_buffer.get_count(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }


    }

    fn draw_array(&mut self) {
        // self.clear();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, Self::VERTICES.len() as i32) }
    }

    pub fn draw(&mut self, time: f32) {
        self.clear();
                
        self.shader.bind();
        // self.shader
        //     .set_uniform_mat4f("u_view", &self.camera.get_view_matrix());
        // self.shader
        //     .set_uniform_mat4f("u_projection", &self.projection);
        // let model = glm::identity();
        // self.shader.set_uniform_mat4f("u_model", &model);

        // self.shader.set_uniform_2f("u_resolution",  800.0, 450.0);
        // self.shader.set_uniform_1f("u_aspect_ratio", 16.0 / 9.0);

        self.shader.set_uniform_vec3("u_camera.center", &self.camera.center);
        self.shader.set_uniform_vec3("u_camera.pixel00_loc", &self.camera.pixel00_loc);
        self.shader.set_uniform_vec3("u_camera.pixel_delta_u", &self.camera.pixel_delta_u);
        self.shader.set_uniform_vec3("u_camera.pixel_delta_v", &self.camera.pixel_delta_v);
        self.shader.set_uniform_1f("u_camera.aspect_ratio", self.camera.aspect_ratio);
        self.shader.set_uniform_1f("u_camera.image_width", self.camera.image_width);
        self.shader.set_uniform_1f("u_camera.image_height", self.camera.image_height);
        self.shader.set_uniform_1f("u_time", time);
        self.vertex_array.bind();
        self.index_buffer.bind();
        self.draw_elements();
    }
    pub fn clear_color(c: Color) {
        unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
    }
}
