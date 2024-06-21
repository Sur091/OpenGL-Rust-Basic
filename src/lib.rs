use glfw::Context;
use nalgebra_glm as glm;

mod camera;
mod index_buffer;
mod renderer;
mod shader;
mod texture;
mod vertex_array;
mod vertex_buffer;

use camera::Camera;
use index_buffer::IndexBuffer;
use renderer::Renderer;
use shader::Shader;
use texture::Texture;
use vertex_array::vertex_buffer_layout::VertexBufferLayout;
use vertex_array::VertexArray;
use vertex_buffer::VertexBuffer;
use camera::CameraMovement;

const TITLE: &str = "My First GLFW window";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub fn run() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .unwrap();
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.set_framebuffer_size_callback(|_window, x, y| unsafe { gl::Viewport(0, 0, x, y) });

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    // unsafe {
    //     gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
    // }

    // Set up for basic texture
    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // Enable error handling
    #[cfg(debug_assertions)]
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(message_callback), std::ptr::null());
    }

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
    }
    // -------------------------------------------

    const VERT_SHADER_PATH: &str = "./src/shader/vertex_shader.glsl";

    const FRAG_SHADER_PATH: &str = "./src/shader/fragment_shader.glsl";

    let mut shader = Shader::new(VERT_SHADER_PATH, FRAG_SHADER_PATH);

    // const SIZE: f32 = 0.5;
    #[rustfmt::skip]
    const VERTICES: [f32; 36*5] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,
    
        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
    
        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,
    
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
    
        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
    
        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
    ];

    const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];

    let vao = VertexArray::new();

    let vbo = VertexBuffer::new(&VERTICES);

    let ib = IndexBuffer::new(&INDICES);

    let mut layout = VertexBufferLayout::new();
    layout.push_f32(3);
    layout.push_f32(2);
    // layout.push_f32(3);

    vao.add_buffer(&vbo, &layout);

    let texture1 = Texture::new("./assets/FlowerPattern2.png");

    let texture2 = Texture::new("./assets/BlueFlowers.jpg");

    shader.bind();
    shader.set_uniform_1i("texture1", 0);
    shader.set_uniform_1i("texture2", 1);

    vbo.unbind();
    vao.unbind();
    ib.unbind();
    // shader.unbind();

    let renderer = Renderer {};

    // -------------------------------------------
    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );

    let mut camera = Camera::default();

    let mut first_mouse = true;
    let mut last_x = screen_width as f32 / 2.0;
    let mut last_y = screen_height as f32 / 2.0;

    let start_time = std::time::Instant::now();
    let mut last_frame = std::time::Instant::now();
    let mut delta_time = 0.0;


    while !window.should_close() {

        delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = std::time::Instant::now();


        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event, &mut camera, delta_time);
        }

        let (x, y) = window.get_cursor_pos();
        process_mouse_movements(x as f32, y as f32, &mut camera, &mut last_x, &mut last_y, &mut first_mouse);

        renderer.clear_color(renderer::Color(0.6, 0.5, 0.1, 1.0));

        // let time = start_time.elapsed().as_secs_f32();
        // let radius: f32 = 10.0;
        // let (cam_x, cam_z) = (radius * time.sin(), radius * time.cos());
        // let camera_position = glm::vec3(cam_x, 0.0, cam_z);
        // camera.change_position(&camera_position);

        let (screen_width, screen_height) = window.get_framebuffer_size();

        let model = glm::identity();
        // let translate_proj = glm::translate(&proj, &glm::vec3(, -0.5, 0.0));
        let model = glm::rotate(
            &model,
            -55.0 * glm::pi::<f32>() / 180.0 * start_time.elapsed().as_secs_f32(),
            &glm::vec3(0.5, 1.0, 0.0),
        );

        let view = camera.get_view_matrix();
        // let view = glm::identity();
        // let view = glm::translate(&view, &glm::vec3(0.0, 0.0, -1.0));

        let projection = glm::perspective(
            glm::quarter_pi::<f32>(),
            screen_width as f32 / screen_height as f32,
            000.1,
            100.0,
        );

        // shader.set_uniform_1f("u_aspect_ratio", screen_width as f32 / screen_height as f32);
        // shader.set_uniform_1f("u_time", start_time.elapsed().as_secs_f32());
        // let identity = glm::identity();
        shader.bind();
        shader.set_uniform_mat4f("u_model", &model);
        shader.set_uniform_mat4f("u_view", &view);
        shader.set_uniform_mat4f("u_projection", &projection);
        texture1.bind(0);
        texture2.bind(1);
        vao.bind();
        ib.bind();

        renderer.clear();
        // renderer.draw(&vao, &ib, &shader);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        vao.unbind();
        ib.unbind();

        window.swap_buffers();
    }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn process_mouse_movements(x: f32, y: f32, camera: &mut Camera, last_x: &mut f32, last_y: &mut f32, first_mouse: &mut bool) {

    if *first_mouse {
        *last_x = x;
        *last_y = y;
        *first_mouse = false;
    }
    let x_offset = x - *last_x;
    let y_offset = *last_y - y;

    *last_x = x;
    *last_y = y;

    camera.process_mouse_movements(x_offset, y_offset);
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent, camera: &mut Camera, delta_time: f32) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;


    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        Event::Key(Key::W, _, Action::Repeat | Action::Press, _) => {
            camera.process_keyboard(CameraMovement::FORWARD, delta_time)
        }
        Event::Key(Key::A, _, Action::Repeat | Action::Press, _) => {
            camera.process_keyboard(CameraMovement::LEFT, delta_time)
        }
        Event::Key(Key::S, _, Action::Repeat | Action::Press, _) => {
            camera.process_keyboard(CameraMovement::BACKWARD, delta_time)
        }
        Event::Key(Key::D, _, Action::Repeat | Action::Press, _) => {
            camera.process_keyboard(CameraMovement::RIGHT, delta_time)
        }
        // Event::MouseButton(, , )
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
