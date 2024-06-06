use glfw::Context;

mod index_buffer;
mod shader;
mod vertex_array;

use index_buffer::IndexBuffer;
use vertex_array::vertex_buffer::VertexBuffer;
use vertex_array::vertex_buffer_layout::VertexBufferLayout;
use vertex_array::VertexArray;
use shader::Shader;

const TITLE: &str = "My First GLFW window";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn run() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .unwrap();
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    // Enable error handling
    #[cfg(debug_assertions)]
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(message_callback), std::ptr::null());
    }

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        clear_color(Color(0.4, 0.4, 0.4, 1.0));
    }
    // -------------------------------------------

    const VERT_SHADER_PATH: &str = "./src/shader/vertex_shader.glsl";

    const FRAG_SHADER_PATH: &str = "./src/shader/fragment_shader.glsl";

    let mut shader = Shader::new(VERT_SHADER_PATH, FRAG_SHADER_PATH);

    const SIZE: f32 = 0.5;
    const VERTICES: [f32; 12] = [
        -SIZE, -SIZE, 0.0, 
         SIZE, -SIZE, 0.0, 
         SIZE,  SIZE, 0.0,
        -SIZE,  SIZE, 0.0,
    ];

    const INDICES: [u32; 6] = [
        0, 1, 2,
        0, 2, 3,
    ];

    let vao = VertexArray::new();

    let vbo = VertexBuffer::new(&VERTICES);

    let mut layout = VertexBufferLayout::new();
    layout.push_f32(3);

    vao.add_buffer(&vbo, &layout);

    vbo.unbind();
    vao.unbind();

    let ib = IndexBuffer::new(&INDICES);
    ib.unbind();

    // -------------------------------------------
    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );

    let start_time = std::time::Instant::now();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        clear_color(Color(0.3, 0.4, 0.6, 1.0));

        shader.bind();
        let (screen_width, screen_height) = window.get_framebuffer_size();
        shader.set_uniform_1f("u_aspect_ratio", screen_width as f32 / screen_height as f32);
        shader.set_uniform_1f("u_time", start_time.elapsed().as_secs_f32());
        vao.bind();
        ib.bind();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, ib.get_count(), gl::UNSIGNED_INT, 0 as *const _);
        }
        vao.unbind();
        ib.unbind();

        window.swap_buffers();
    }
}

pub struct Color(f32, f32, f32, f32);

pub fn clear_color(c: Color) {
    unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

#[allow(dead_code)]
extern "system" fn message_callback(
    source: gl::types::GLenum,
    ty: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut std::os::raw::c_void,
) {
    let message_str = unsafe {
        std::ffi::CStr::from_ptr(message).to_string_lossy()
    };

    let message_source = match source {
        gl::DEBUG_SOURCE_API => "OpenGL API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        gl::DEBUG_SOURCE_OTHER => "Other",
        _ => "Unknown"
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
