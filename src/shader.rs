use nalgebra_glm as glm;

struct ShaderProgramSource {
    vertex_source: String,
    fragment_source: String,
}

pub struct Shader {
    _vertex_file_path: String,

    _fragment_file_path: String,

    renderer_id: u32,
    uniform_location_cache: std::collections::HashMap<String, i32>,
}

impl Shader {
    pub fn new(vertex_file_path: &str, fragment_file_path: &str) -> Shader {
        let source: ShaderProgramSource =
            Self::parse_shader(&vertex_file_path, &fragment_file_path);
        Self {
            _vertex_file_path: vertex_file_path.to_string(),
            _fragment_file_path: fragment_file_path.to_string(),
            renderer_id: Self::create_shader(&source.vertex_source, &source.fragment_source),
            uniform_location_cache: Default::default(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.renderer_id);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_1i(&mut self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_1f(&mut self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_4f(&mut self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), v0, v1, v2, v3);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_2f(&mut self, name: &str, v0: f32, v1: f32) {
        unsafe {
            gl::Uniform2f(self.get_uniform_location(name), v0, v1);
        }
    }

    #[allow(dead_code)]
    pub fn set_uniform_3f(&mut self, name: &str, v0: f32, v1: f32, v2: f32) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), v0, v1, v2);
        }
    }

    pub fn set_uniform_mat4f(&mut self, name: &str, proj: &glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                glm::value_ptr(&proj).as_ptr().cast(),
            )
        }
    }

    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn create_shader(vertex_shader: &str, fragment_shader: &str) -> u32 {
        let program = unsafe { gl::CreateProgram() };
        let vs = Self::compile_shader(gl::VERTEX_SHADER, vertex_shader);
        let fs = Self::compile_shader(gl::FRAGMENT_SHADER, fragment_shader);

        unsafe {
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::ValidateProgram(program);

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
        return program;
    }

    fn parse_shader(vertex_file_path: &str, fragment_file_path: &str) -> ShaderProgramSource {
        let vertex_source = std::fs::read_to_string(vertex_file_path)
            .expect(format!("Can't open the file {vertex_file_path}").as_str());
        let fragment_source = std::fs::read_to_string(fragment_file_path)
            .expect(format!("Can't open the file {fragment_file_path}").as_str());
        return ShaderProgramSource {
            vertex_source,
            fragment_source,
        };
    }

    fn compile_shader(shader_type: u32, source: &str) -> u32 {
        let shader = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(
                shader,
                1,
                &source.as_bytes().as_ptr().cast(),
                &source.len().try_into().unwrap(),
            );
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(512);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(shader, 512, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!(
                    "{} Shader Compile Error: {}",
                    if shader_type == gl::FRAGMENT_SHADER {
                        "Fragment"
                    } else {
                        "Vertex"
                    },
                    String::from_utf8_lossy(&v)
                );
            }
        }
        return shader;
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        match self.uniform_location_cache.get(name) {
            Some(uniform_location) => *uniform_location,
            None => {
                let location: i32 = unsafe {
                    let c_string = std::ffi::CString::new(name).unwrap();
                    gl::GetUniformLocation(self.renderer_id, c_string.as_ptr().cast())
                };
                if location == -1 {
                    println!("Warning: Uniform {name} doesn't exist.");
                }
                self.uniform_location_cache
                    .insert(name.to_string(), location);
                location
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.renderer_id);
        }
    }
}
